use anyhow::Result;
use crate::types::{DecodedEvent, RawEvent};
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
