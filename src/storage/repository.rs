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
        from_ledger: Option<i64>,
        to_ledger: Option<i64>,
        event_type: Option<&str>,
    ) -> Result<Vec<ContractEvent>> {
        let events = sqlx::query_as::<_, ContractEvent>(
            r#"SELECT id, ledger_seq, tx_hash, contract_id, event_type,
                      topics, data, created_at
               FROM contract_events
               WHERE contract_id = $1
                 AND ($4::bigint IS NULL OR ledger_seq >= $4)
                 AND ($5::bigint IS NULL OR ledger_seq <= $5)
                 AND ($6::text IS NULL OR event_type = $6)
               ORDER BY ledger_seq DESC
               LIMIT $2 OFFSET $3"#,
        )
        .bind(contract_id)
        .bind(limit)
        .bind(offset)
        .bind(from_ledger)
        .bind(to_ledger)
        .bind(event_type)
        .fetch_all(&self.pool)
        .await?;
        Ok(events)
    }

    pub async fn count_events_by_contract(
        &self,
        contract_id: &str,
        from_ledger: Option<i64>,
        to_ledger: Option<i64>,
        event_type: Option<&str>,
    ) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"SELECT COUNT(*)
               FROM contract_events
               WHERE contract_id = $1
                 AND ($2::bigint IS NULL OR ledger_seq >= $2)
                 AND ($3::bigint IS NULL OR ledger_seq <= $3)
                 AND ($4::text IS NULL OR event_type = $4)"#,
        )
        .bind(contract_id)
        .bind(from_ledger)
        .bind(to_ledger)
        .bind(event_type)
        .fetch_one(&self.pool)
        .await?;
        Ok(count)
    }

    pub async fn get_latest_ledger(&self) -> Result<Option<i64>> {
        let row: Option<i64> = sqlx::query_scalar("SELECT MAX(sequence) FROM ledgers")
            .fetch_one(&self.pool)
            .await?;
        Ok(row)
    }
}
