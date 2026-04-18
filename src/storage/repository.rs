use crate::storage::models::ContractEvent;
use anyhow::Result;
use sqlx::PgPool;

pub struct EventRepository {
    pool: PgPool,
}

impl EventRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_events_by_contract(
        &self,
        contract_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ContractEvent>> {
        let events = sqlx::query_as::<_, ContractEvent>(
            r#"SELECT id, ledger_seq, tx_hash, contract_id, event_type,
                      topics, data, created_at
               FROM contract_events
               WHERE contract_id = $1
               ORDER BY ledger_seq DESC
               LIMIT $2 OFFSET $3"#,
        )
        .bind(contract_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok(events)
    }

    pub async fn get_latest_ledger(&self) -> Result<Option<i64>> {
        let row: Option<i64> = sqlx::query_scalar("SELECT MAX(sequence) FROM ledgers")
            .fetch_one(&self.pool)
            .await?;
        Ok(row)
    }
}
