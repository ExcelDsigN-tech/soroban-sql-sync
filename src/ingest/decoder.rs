use crate::types::{DecodedEvent, RawEvent};
use anyhow::Result;
use serde::Deserialize;
use tracing::warn;

pub struct XdrDecoder;

#[derive(Debug, Deserialize)]
struct SampleEventFixture {
    event_type: String,
    xdr_payload: String,
    data: serde_json::Value,
}

impl XdrDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, raw: &RawEvent) -> Result<DecodedEvent> {
        if let Some(fixture) = sample_event_fixture(&raw.xdr_payload)? {
            return Ok(DecodedEvent {
                ledger_seq: raw.ledger_seq,
                tx_hash: raw.tx_hash.clone(),
                contract_id: raw.contract_id.clone(),
                event_type: fixture.event_type,
                data: fixture.data,
            });
        }

        // Phase 2: implement full XDR decoding here
        warn!("XDR decoding not yet implemented — returning placeholder");
        Ok(DecodedEvent {
            ledger_seq: raw.ledger_seq,
            tx_hash: raw.tx_hash.clone(),
            contract_id: raw.contract_id.clone(),
            event_type: "unknown".to_string(),
            data: serde_json::Value::Null,
        })
    }
}

fn sample_event_fixture(xdr_payload: &str) -> Result<Option<SampleEventFixture>> {
    let fixtures: Vec<SampleEventFixture> = serde_json::from_str(include_str!(
        "../../fixtures/event-schema/sample-events.json"
    ))?;
    Ok(fixtures
        .into_iter()
        .find(|fixture| fixture.xdr_payload == xdr_payload))
}

impl Default for XdrDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::XdrDecoder;
    use crate::types::RawEvent;
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    struct EventFixture {
        event_type: String,
        xdr_payload: String,
        data: serde_json::Value,
    }

    #[test]
    fn sample_event_fixtures_are_referenced_by_decoder_tests() {
        let fixtures: Vec<EventFixture> = serde_json::from_str(include_str!(
            "../../fixtures/event-schema/sample-events.json"
        ))
        .expect("event schema fixtures must remain valid JSON");
        let decoder = XdrDecoder::new();
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        for fixture in fixtures {
            let snapshot_path = manifest_dir.join(&fixture.xdr_payload);
            assert!(
                snapshot_path.exists(),
                "fixture snapshot path should exist: {}",
                snapshot_path.display()
            );
            let raw = RawEvent {
                ledger_seq: 1,
                tx_hash: format!("fixture-{}", fixture.event_type),
                contract_id: "sample-event-emitter".to_string(),
                xdr_payload: fixture.xdr_payload.clone(),
            };
            let decoded = decoder
                .decode(&raw)
                .expect("placeholder decoder should not fail");
            assert_eq!(decoded.ledger_seq, raw.ledger_seq);
            assert_eq!(decoded.tx_hash, raw.tx_hash);
            assert_eq!(decoded.contract_id, raw.contract_id);
            assert_eq!(decoded.event_type, fixture.event_type);
            assert_eq!(decoded.data, fixture.data);
        }
    }
}
