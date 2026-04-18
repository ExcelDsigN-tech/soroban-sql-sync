//! Shared type definitions for events, ledgers, and API responses.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEvent {
    pub ledger_seq: u64,
    pub tx_hash: String,
    pub contract_id: String,
    pub xdr_payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedEvent {
    pub ledger_seq: u64,
    pub tx_hash: String,
    pub contract_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub ingestion: String,
    pub latest_ledger: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsResponse {
    pub events: Vec<DecodedEvent>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
