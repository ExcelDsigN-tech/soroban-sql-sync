use crate::storage::models::ContractEvent;
use anyhow::Result;
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventFilters {
    pub from_ledger: Option<i64>,
    pub to_ledger: Option<i64>,
    pub event_type: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

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
        let filters = EventFilters {
            limit,
            offset,
            ..EventFilters::default()
        };

        self.get_events_by_contract_filtered(contract_id, &filters)
            .await
    }

    pub async fn get_events_by_contract_filtered(
        &self,
        contract_id: &str,
        filters: &EventFilters,
    ) -> Result<Vec<ContractEvent>> {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"SELECT id, ledger_seq, tx_hash, contract_id, event_type,
                      topics, data, created_at
               FROM contract_events
               WHERE contract_id = "#,
        );
        query.push_bind(contract_id);
        push_event_filters(&mut query, filters);
        query
            .push(" ORDER BY ledger_seq DESC, id DESC LIMIT ")
            .push_bind(filters.limit)
            .push(" OFFSET ")
            .push_bind(filters.offset);

        let events = query
            .build_query_as::<ContractEvent>()
            .fetch_all(&self.pool)
            .await?;
        Ok(events)
    }

    pub async fn count_events_by_contract_filtered(
        &self,
        contract_id: &str,
        filters: &EventFilters,
    ) -> Result<i64> {
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT COUNT(*) FROM contract_events WHERE contract_id = ",
        );
        query.push_bind(contract_id);
        push_event_filters(&mut query, filters);

        let count = query
            .build_query_scalar::<i64>()
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

fn push_event_filters(query: &mut QueryBuilder<Postgres>, filters: &EventFilters) {
    if let Some(from_ledger) = filters.from_ledger {
        query.push(" AND ledger_seq >= ").push_bind(from_ledger);
    }

    if let Some(to_ledger) = filters.to_ledger {
        query.push(" AND ledger_seq <= ").push_bind(to_ledger);
    }

    if let Some(event_type) = filters.event_type.as_deref() {
        query.push(" AND event_type = ").push_bind(event_type);
    }
}
