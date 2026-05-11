use crate::types::{DecodedEvent, RawEvent};
use anyhow::Result;
use tracing::warn;

pub struct XdrDecoder;

impl XdrDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, raw: &RawEvent) -> Result<DecodedEvent> {
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
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct EventFixture {
        event_type: String,
        xdr_payload: String,
    }

    #[test]
    fn sample_event_fixtures_are_referenced_by_decoder_tests() {
        let fixtures: Vec<EventFixture> = serde_json::from_str(include_str!(
            "../../fixtures/event-schema/sample-events.json"
        ))
        .expect("event schema fixtures must remain valid JSON");
        let decoder = XdrDecoder::new();

        for fixture in fixtures {
            assert!(
                Path::new(&fixture.xdr_payload).exists(),
                "fixture snapshot path should exist: {}",
                fixture.xdr_payload
            );
            let raw = RawEvent {
                ledger_seq: 1,
                tx_hash: format!("fixture-{}", fixture.event_type),
                contract_id: "sample-event-emitter".to_string(),
                xdr_payload: fixture.xdr_payload,
            };
            let decoded = decoder
                .decode(&raw)
                .expect("placeholder decoder should not fail");
            assert_eq!(decoded.ledger_seq, raw.ledger_seq);
            assert_eq!(decoded.tx_hash, raw.tx_hash);
            assert_eq!(decoded.contract_id, raw.contract_id);
        }
    }
}
