use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ledger {
    pub id: i64,
    pub sequence: i64,
    pub hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tx_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContractEvent {
    pub id: i64,
    pub ledger_seq: i64,
    pub tx_hash: String,
    pub contract_id: String,
    pub event_type: Option<String>,
    pub topics: Option<String>,
    pub data: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransactionMeta {
    pub id: i64,
    pub tx_hash: String,
    pub ledger_seq: i64,
    pub fee: Option<i64>,
    pub status: Option<String>,
    pub raw_meta: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
