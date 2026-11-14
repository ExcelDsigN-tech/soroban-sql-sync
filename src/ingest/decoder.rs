use crate::types::{DecodedEvent, RawEvent};
use anyhow::Result;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use soroban_sdk::xdr::{
    ContractEvent, ContractEventBody, Int128Parts, Limits, ReadXdr, ScVal, UInt128Parts,
};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DecodeError {
    #[error("empty XDR payload")]
    EmptyPayload,
    #[error("malformed XDR payload: {0}")]
    MalformedXdr(String),
    #[error("unsupported Soroban XDR type: {0}")]
    UnsupportedType(String),
}

#[derive(Debug, Deserialize)]
struct EnvelopePayload {
    #[serde(default)]
    topic: Vec<String>,
    value: String,
}

pub struct XdrDecoder;

impl XdrDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, raw: &RawEvent) -> Result<DecodedEvent> {
        let trimmed = raw.xdr_payload.trim();
        if trimmed.is_empty() {
            return Err(DecodeError::EmptyPayload.into());
        }

        let (event_type, data) = self.decode_payload_str(trimmed)?;

        Ok(DecodedEvent {
            ledger_seq: raw.ledger_seq,
            tx_hash: raw.tx_hash.clone(),
            contract_id: raw.contract_id.clone(),
            event_type,
            data,
        })
    }

    pub fn decode_non_fatal(&self, raw: &RawEvent) -> DecodedEvent {
        match self.decode(raw) {
            Ok(decoded) => decoded,
            Err(err) => {
                warn!(
                    ledger_seq = raw.ledger_seq,
                    tx_hash = %raw.tx_hash,
                    contract_id = %raw.contract_id,
                    error = %err,
                    "Non-fatal XDR decode error; returning fallback DecodedEvent"
                );
                DecodedEvent {
                    ledger_seq: raw.ledger_seq,
                    tx_hash: raw.tx_hash.clone(),
                    contract_id: raw.contract_id.clone(),
                    event_type: "decode_error".to_string(),
                    data: json!({
                        "error": err.to_string(),
                        "raw_payload": raw.xdr_payload,
                    }),
                }
            }
        }
    }

    fn decode_payload_str(&self, payload: &str) -> Result<(String, Value), DecodeError> {
        if payload.starts_with('{') {
            let env: EnvelopePayload = serde_json::from_str(payload)
                .map_err(|e| DecodeError::MalformedXdr(e.to_string()))?;
            let mut decoded_topics = Vec::with_capacity(env.topic.len());
            for topic_xdr in &env.topic {
                let sc_val = ScVal::from_xdr_base64(topic_xdr.trim(), Limits::none())
                    .map_err(|e| DecodeError::MalformedXdr(e.to_string()))?;
                decoded_topics.push(scval_to_json(&sc_val)?);
            }
            let value_sc = ScVal::from_xdr_base64(env.value.trim(), Limits::none())
                .map_err(|e| DecodeError::MalformedXdr(e.to_string()))?;
            let decoded_value = scval_to_json(&value_sc)?;
            let event_type = extract_event_type(&decoded_topics);
            return Ok((
                event_type,
                json!({
                    "topics": decoded_topics,
                    "value": decoded_value,
                }),
            ));
        }

        if let Ok(contract_ev) = ContractEvent::from_xdr_base64(payload, Limits::none()) {
            let ContractEventBody::V0(v0) = contract_ev.body;
            let mut decoded_topics = Vec::with_capacity(v0.topics.len());
            for topic_sc in v0.topics.iter() {
                decoded_topics.push(scval_to_json(topic_sc)?);
            }
            let decoded_value = scval_to_json(&v0.data)?;
            let event_type = extract_event_type(&decoded_topics);
            return Ok((
                event_type,
                json!({
                    "topics": decoded_topics,
                    "value": decoded_value,
                }),
            ));
        }

        let sc_val = ScVal::from_xdr_base64(payload, Limits::none())
            .map_err(|e| DecodeError::MalformedXdr(e.to_string()))?;
        let json_val = scval_to_json(&sc_val)?;
        let event_type = match &sc_val {
            ScVal::Symbol(sym) => sym.to_utf8_string_lossy(),
            _ => "scval".to_string(),
        };
        Ok((event_type, json_val))
    }
}

pub fn scval_to_json(val: &ScVal) -> Result<Value, DecodeError> {
    match val {
        ScVal::Void => Ok(Value::Null),
        ScVal::Bool(b) => Ok(Value::Bool(*b)),
        ScVal::U32(n) => Ok(json!(*n)),
        ScVal::I32(n) => Ok(json!(*n)),
        ScVal::U64(n) => Ok(json!(*n)),
        ScVal::I64(n) => Ok(json!(*n)),
        ScVal::Timepoint(tp) => Ok(json!(tp.0)),
        ScVal::Duration(dur) => Ok(json!(dur.0)),
        ScVal::U128(parts) => Ok(Value::String(u128_from_parts(parts).to_string())),
        ScVal::I128(parts) => Ok(Value::String(i128_from_parts(parts).to_string())),
        ScVal::Symbol(sym) => Ok(Value::String(sym.to_utf8_string_lossy())),
        ScVal::String(s) => Ok(Value::String(s.to_utf8_string_lossy())),
        ScVal::Bytes(bytes) => Ok(Value::String(bytes_to_hex(bytes.as_slice()))),
        ScVal::Address(addr) => Ok(Value::String(addr.to_string())),
        ScVal::Vec(opt_vec) => {
            let Some(vec_m) = opt_vec else {
                return Ok(Value::Array(Vec::new()));
            };
            let mut items = Vec::with_capacity(vec_m.len());
            for item in vec_m.iter() {
                items.push(scval_to_json(item)?);
            }
            Ok(Value::Array(items))
        }
        ScVal::Map(opt_map) => {
            let Some(map_m) = opt_map else {
                return Ok(Value::Object(Map::new()));
            };
            let mut obj = Map::new();
            for entry in map_m.iter() {
                let key_json = scval_to_json(&entry.key)?;
                let key_str = match key_json {
                    Value::String(s) => s,
                    other => other.to_string(),
                };
                let val_json = scval_to_json(&entry.val)?;
                obj.insert(key_str, val_json);
            }
            Ok(Value::Object(obj))
        }
        other => Err(DecodeError::UnsupportedType(format!("{other:?}"))),
    }
}

fn u128_from_parts(parts: &UInt128Parts) -> u128 {
    (u128::from(parts.hi) << 64) | u128::from(parts.lo)
}

fn i128_from_parts(parts: &Int128Parts) -> i128 {
    ((i128::from(parts.hi)) << 64) | (i128::from(parts.lo) & 0xFFFF_FFFF_FFFF_FFFF)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{b:02x}");
    }
    out
}

fn extract_event_type(topics: &[Value]) -> String {
    topics
        .first()
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("contract_event")
        .to_string()
}

impl Default for XdrDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::xdr::{
        ContractId, Hash, ScAddress, ScBytes, ScSymbol, VecM, WriteXdr,
    };

    #[test]
    fn test_decode_symbol_address_integers_bytes_and_vec() {
        let decoder = XdrDecoder::new();

        let topic_sym = ScVal::Symbol(ScSymbol::try_from("transfer").unwrap())
            .to_xdr_base64(Limits::none())
            .unwrap();
        let addr_val = ScVal::Address(ScAddress::Contract(ContractId(Hash([7_u8; 32]))))
            .to_xdr_base64(Limits::none())
            .unwrap();

        let vec_items: VecM<ScVal> = vec![
            ScVal::I128(Int128Parts { hi: 0, lo: 500_000 }),
            ScVal::U128(UInt128Parts { hi: 0, lo: 999_999 }),
            ScVal::Bytes(ScBytes::try_from(vec![0xde, 0xad, 0xbe, 0xef]).unwrap()),
        ]
        .try_into()
        .unwrap();
        let value_xdr = ScVal::Vec(Some(vec_items))
            .to_xdr_base64(Limits::none())
            .unwrap();

        let raw = RawEvent {
            ledger_seq: 42,
            tx_hash: "tx-abc".to_string(),
            contract_id: "CCONTRACT".to_string(),
            xdr_payload: json!({
                "topic": [topic_sym, addr_val],
                "value": value_xdr,
            })
            .to_string(),
        };

        let decoded = decoder.decode(&raw).expect("valid XDR envelope should decode");
        assert_eq!(decoded.event_type, "transfer");
        assert_eq!(decoded.ledger_seq, 42);
        assert_eq!(decoded.data["value"][0], json!("500000"));
        assert_eq!(decoded.data["value"][1], json!("999999"));
        assert_eq!(decoded.data["value"][2], json!("deadbeef"));
    }

    #[test]
    fn test_decode_non_fatal_handles_malformed_xdr_gracefully() {
        let decoder = XdrDecoder::new();
        let bad_raw = RawEvent {
            ledger_seq: 99,
            tx_hash: "tx-bad".to_string(),
            contract_id: "CCONTRACT".to_string(),
            xdr_payload: "!!!not-valid-base64-xdr!!!".to_string(),
        };

        assert!(decoder.decode(&bad_raw).is_err());
        let fallback = decoder.decode_non_fatal(&bad_raw);
        assert_eq!(fallback.event_type, "decode_error");
        assert_eq!(fallback.ledger_seq, 99);
        assert!(fallback.data["error"].as_str().is_some());
    }
}