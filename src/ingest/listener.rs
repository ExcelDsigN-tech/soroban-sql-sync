use crate::config::Config;
use crate::types::RawEvent;
use anyhow::Result;
use sqlx::PgPool;
use std::time::Duration;
use stellar_rpc_client::{Client, Event, EventStart, EventType};
use tokio::time::sleep;
use tracing::{error, info, warn};

const DEFAULT_POLL_INTERVAL_MS: u64 = 5_000;
const DEFAULT_BASE_BACKOFF_MS: u64 = 1_000;
const DEFAULT_MAX_BACKOFF_MS: u64 = 30_000;
const DEFAULT_BATCH_LIMIT: usize = 100;

pub struct EventListener {
    config: Config,
    pool: Option<PgPool>,
    poll_interval: Duration,
    base_backoff_ms: u64,
    max_backoff_ms: u64,
    batch_limit: usize,
}

impl EventListener {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pool: None,
            poll_interval: Duration::from_millis(DEFAULT_POLL_INTERVAL_MS),
            base_backoff_ms: DEFAULT_BASE_BACKOFF_MS,
            max_backoff_ms: DEFAULT_MAX_BACKOFF_MS,
            batch_limit: DEFAULT_BATCH_LIMIT,
        }
    }

    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn with_poll_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    pub fn backoff_duration(&self, attempt: u32) -> Duration {
        compute_backoff(attempt, self.base_backoff_ms, self.max_backoff_ms)
    }

    pub async fn load_checkpoint(&self) -> Result<Option<u32>> {
        let Some(pool) = &self.pool else {
            return Ok(None);
        };
        let latest: Option<i64> = sqlx::query_scalar("SELECT MAX(sequence) FROM ledgers")
            .fetch_one(pool)
            .await?;
        Ok(latest.and_then(|seq| u32::try_from(seq).ok()))
    }

    pub async fn save_checkpoint(&self, ledger_seq: u32) -> Result<()> {
        let Some(pool) = &self.pool else {
            return Ok(());
        };
        let seq_i64 = i64::from(ledger_seq);
        let synthetic_hash = format!("ledger-{ledger_seq}");
        sqlx::query(
            r#"INSERT INTO ledgers (sequence, hash, timestamp, tx_count)
               VALUES ($1, $2, NOW(), 0)
               ON CONFLICT (sequence) DO NOTHING"#,
        )
        .bind(seq_i64)
        .bind(synthetic_hash)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub fn filter_watched_events(&self, events: Vec<Event>) -> Vec<RawEvent> {
        let mut matched = Vec::new();
        for ev in events {
            if !self
                .config
                .contract_ids
                .iter()
                .any(|cid| cid == &ev.contract_id)
            {
                continue;
            }
            let tx_hash = ev.tx_hash.clone().unwrap_or_else(|| ev.id.clone());
            let payload_json = serde_json::json!({
                "id": ev.id,
                "type": ev.event_type,
                "topic": ev.topic,
                "value": ev.value,
            });
            let xdr_payload = payload_json.to_string();

            info!(
                ledger = ev.ledger,
                contract_id = %ev.contract_id,
                tx_hash = %tx_hash,
                event_id = %ev.id,
                topic_count = ev.topic.len(),
                xdr_payload = %xdr_payload,
                "Ingested Soroban contract event"
            );

            matched.push(RawEvent {
                ledger_seq: u64::from(ev.ledger),
                tx_hash,
                contract_id: ev.contract_id,
                xdr_payload,
            });
        }
        matched
    }

    pub async fn poll_once(
        &self,
        client: &Client,
        cursor_ledger: &mut Option<u32>,
    ) -> Result<Vec<RawEvent>> {
        let start_ledger = match *cursor_ledger {
            Some(seq) => seq,
            None => match self.load_checkpoint().await? {
                Some(saved) => {
                    let next = saved.saturating_add(1);
                    *cursor_ledger = Some(next);
                    next
                }
                None => {
                    let latest = client.get_latest_ledger().await?;
                    *cursor_ledger = Some(latest.sequence);
                    latest.sequence
                }
            },
        };

        let response = client
            .get_events(
                EventStart::Ledger(start_ledger),
                Some(EventType::Contract),
                &self.config.contract_ids,
                &[],
                Some(self.batch_limit),
            )
            .await?;

        let raw_events = self.filter_watched_events(response.events);
        let highest_ledger = raw_events
            .iter()
            .map(|e| u32::try_from(e.ledger_seq).unwrap_or(start_ledger))
            .max()
            .unwrap_or(response.latest_ledger.max(start_ledger));

        if highest_ledger >= start_ledger {
            self.save_checkpoint(highest_ledger).await?;
            *cursor_ledger = Some(highest_ledger.saturating_add(1));
        }

        Ok(raw_events)
    }

    pub async fn run(&self) -> Result<()> {
        info!(
            rpc_url = %self.config.soroban_rpc_url,
            contracts = ?self.config.contract_ids,
            "EventListener starting for contracts"
        );

        let client = Client::new(&self.config.soroban_rpc_url)?;
        let mut cursor_ledger: Option<u32> = None;
        let mut consecutive_failures: u32 = 0;

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Shutdown signal received; stopping EventListener gracefully");
                    break;
                }
                poll_res = self.poll_once(&client, &mut cursor_ledger) => {
                    match poll_res {
                        Ok(events) => {
                            consecutive_failures = 0;
                            if !events.is_empty() {
                                info!(count = events.len(), "Processed batch of contract events");
                            }
                            sleep(self.poll_interval).await;
                        }
                        Err(err) => {
                            consecutive_failures = consecutive_failures.saturating_add(1);
                            let delay = self.backoff_duration(consecutive_failures);
                            warn!(
                                attempt = consecutive_failures,
                                backoff_ms = delay.as_millis() as u64,
                                error = %err,
                                "Transient Soroban RPC poll failure; retrying after backoff"
                            );
                            error!("RPC poll error details: {err:#}");
                            sleep(delay).await;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn compute_backoff(attempt: u32, base_ms: u64, max_ms: u64) -> Duration {
    let shift = attempt.saturating_sub(1).min(16);
    let factor = 1_u64.checked_shl(shift).unwrap_or(u64::MAX);
    let delay_ms = base_ms.saturating_mul(factor).min(max_ms);
    Duration::from_millis(delay_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> Config {
        Config {
            database_url: "postgres://localhost/soroban_sql_sync_test".to_string(),
            soroban_rpc_url: "https://soroban-testnet.stellar.org:443".to_string(),
            contract_ids: vec!["CCONTRACT1".to_string(), "CCONTRACT2".to_string()],
        }
    }

    #[test]
    fn test_exponential_backoff_caps_at_max() {
        let listener = EventListener::new(sample_config());
        assert_eq!(listener.backoff_duration(1), Duration::from_millis(1_000));
        assert_eq!(listener.backoff_duration(2), Duration::from_millis(2_000));
        assert_eq!(listener.backoff_duration(3), Duration::from_millis(4_000));
        assert_eq!(listener.backoff_duration(10), Duration::from_millis(30_000));
    }

    #[test]
    fn test_filter_watched_events_matches_configured_contracts() {
        let listener = EventListener::new(sample_config());
        let events = vec![
            Event {
                event_type: "contract".to_string(),
                ledger: 120,
                ledger_closed_at: "2026-04-17T00:00:00Z".to_string(),
                contract_id: "CCONTRACT1".to_string(),
                id: "000120-0001".to_string(),
                operation_index: Some(0),
                transaction_index: Some(0),
                tx_hash: Some("txhash120".to_string()),
                #[allow(deprecated)]
                is_successful_contract_call: Some(true),
                topic: vec!["AAAADwAAAAh0cmFuc2Zlcg==".to_string()],
                value: "AAAAAwAAABk=".to_string(),
            },
            Event {
                event_type: "contract".to_string(),
                ledger: 121,
                ledger_closed_at: "2026-04-17T00:00:05Z".to_string(),
                contract_id: "CUNWATCHED".to_string(),
                id: "000121-0001".to_string(),
                operation_index: Some(0),
                transaction_index: Some(0),
                tx_hash: Some("txhash121".to_string()),
                #[allow(deprecated)]
                is_successful_contract_call: Some(true),
                topic: vec![],
                value: "AAAAAwAAABk=".to_string(),
            },
        ];

        let filtered = listener.filter_watched_events(events);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].contract_id, "CCONTRACT1");
        assert_eq!(filtered[0].ledger_seq, 120);
        assert_eq!(filtered[0].tx_hash, "txhash120");
        assert!(filtered[0].xdr_payload.contains("AAAADwAAAAh0cmFuc2Zlcg=="));
    }
}
