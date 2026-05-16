use crate::{
    api::errors::ApiError,
    storage::{ContractEvent, EventFilters, EventRepository},
    types::HealthResponse,
};
use anyhow::Result;
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderName, HeaderValue, Request},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::Instrument;

static CORRELATION_COUNTER: AtomicU64 = AtomicU64::new(1);

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn latest_ledger(&self) -> Result<Option<i64>>;

    async fn events_by_contract(
        &self,
        contract_id: &str,
        filters: &EventFilters,
    ) -> Result<Vec<ContractEvent>>;

    async fn count_events_by_contract(
        &self,
        contract_id: &str,
        filters: &EventFilters,
    ) -> Result<i64>;
}

#[async_trait]
impl EventStore for EventRepository {
    async fn latest_ledger(&self) -> Result<Option<i64>> {
        self.get_latest_ledger().await
    }

    async fn events_by_contract(
        &self,
        contract_id: &str,
        filters: &EventFilters,
    ) -> Result<Vec<ContractEvent>> {
        self.get_events_by_contract_filtered(contract_id, filters)
            .await
    }

    async fn count_events_by_contract(
        &self,
        contract_id: &str,
        filters: &EventFilters,
    ) -> Result<i64> {
        self.count_events_by_contract_filtered(contract_id, filters)
            .await
    }
}

#[derive(Clone)]
pub struct ApiState {
    store: Arc<dyn EventStore>,
}

impl ApiState {
    pub fn new(store: Arc<dyn EventStore>) -> Self {
        Self { store }
    }
}

pub fn create_router(pool: PgPool) -> Router {
    create_router_with_store(Arc::new(EventRepository::new(pool)))
}

pub fn create_router_with_store(store: Arc<dyn EventStore>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/events/:contract_id", get(events_by_contract))
        .layer(middleware::from_fn(correlation_id_middleware))
        .with_state(ApiState::new(store))
}

#[derive(Debug, Deserialize)]
struct EventQuery {
    from_ledger: Option<i64>,
    to_ledger: Option<i64>,
    event_type: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
struct EventListResponse {
    contract_id: String,
    total: i64,
    page: i64,
    page_size: i64,
    events: Vec<ContractEvent>,
}

async fn health(State(state): State<ApiState>) -> Result<Json<HealthResponse>, ApiError> {
    let latest_ledger = state
        .store
        .latest_ledger()
        .await
        .map_err(ApiError::Internal)?
        .and_then(|ledger| u64::try_from(ledger).ok());

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        database: "connected".to_string(),
        ingestion: "idle".to_string(),
        latest_ledger,
    }))
}

async fn events_by_contract(
    State(state): State<ApiState>,
    Path(contract_id): Path<String>,
    Query(query): Query<EventQuery>,
) -> Result<Json<EventListResponse>, ApiError> {
    let contract_id = contract_id.trim();
    if contract_id.is_empty() {
        return Err(ApiError::BadRequest(
            "contract_id path parameter must not be empty".to_string(),
        ));
    }

    let (filters, page, page_size) = query.into_filters()?;
    let total = state
        .store
        .count_events_by_contract(contract_id, &filters)
        .await
        .map_err(ApiError::Internal)?;
    let events = state
        .store
        .events_by_contract(contract_id, &filters)
        .await
        .map_err(ApiError::Internal)?;

    Ok(Json(EventListResponse {
        contract_id: contract_id.to_string(),
        total,
        page,
        page_size,
        events,
    }))
}

impl EventQuery {
    fn into_filters(self) -> Result<(EventFilters, i64, i64), ApiError> {
        let page = self.page.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(50);

        if page < 1 {
            return Err(ApiError::BadRequest(
                "page must be greater than or equal to 1".to_string(),
            ));
        }

        if !(1..=500).contains(&page_size) {
            return Err(ApiError::BadRequest(
                "page_size must be between 1 and 500".to_string(),
            ));
        }

        if let (Some(from_ledger), Some(to_ledger)) = (self.from_ledger, self.to_ledger) {
            if from_ledger > to_ledger {
                return Err(ApiError::BadRequest(
                    "from_ledger must be less than or equal to to_ledger".to_string(),
                ));
            }
        }

        if self.from_ledger.is_some_and(|ledger| ledger < 0)
            || self.to_ledger.is_some_and(|ledger| ledger < 0)
        {
            return Err(ApiError::BadRequest(
                "ledger filters must be non-negative".to_string(),
            ));
        }

        let offset = (page - 1).checked_mul(page_size).ok_or_else(|| {
            ApiError::BadRequest("page and page_size produce an invalid offset".to_string())
        })?;

        let event_type = self
            .event_type
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        Ok((
            EventFilters {
                from_ledger: self.from_ledger,
                to_ledger: self.to_ledger,
                event_type,
                limit: page_size,
                offset,
            },
            page,
            page_size,
        ))
    }
}

async fn correlation_id_middleware(mut request: Request<Body>, next: Next) -> Response {
    let header_name = correlation_header();
    let correlation_id = request
        .headers()
        .get(&header_name)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(next_correlation_id);

    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let span = tracing::info_span!(
        "api_request",
        correlation_id = %correlation_id,
        method = %method,
        path = %path
    );

    request.extensions_mut().insert(correlation_id.clone());
    let mut response = next.run(request).instrument(span).await;

    if let Ok(value) = HeaderValue::from_str(&correlation_id) {
        response.headers_mut().insert(header_name, value);
    }

    tracing::info!(
        correlation_id = %correlation_id,
        method = %method,
        path = %path,
        status = response.status().as_u16(),
        "handled API request"
    );

    response
}

fn correlation_header() -> HeaderName {
    HeaderName::from_static("x-correlation-id")
}

fn next_correlation_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_micros())
        .unwrap_or_default();
    let counter = CORRELATION_COUNTER.fetch_add(1, Ordering::Relaxed);

    format!("soroban-sql-sync-{now}-{counter}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{self, Body},
        http::{Request, StatusCode},
    };
    use chrono::{TimeZone, Utc};
    use serde_json::Value;
    use tower::ServiceExt;

    #[derive(Clone)]
    struct FakeStore {
        latest_ledger: Option<i64>,
        events: Arc<Vec<ContractEvent>>,
    }

    #[async_trait]
    impl EventStore for FakeStore {
        async fn latest_ledger(&self) -> Result<Option<i64>> {
            Ok(self.latest_ledger)
        }

        async fn events_by_contract(
            &self,
            contract_id: &str,
            filters: &EventFilters,
        ) -> Result<Vec<ContractEvent>> {
            Ok(self
                .filtered_events(contract_id, filters)
                .into_iter()
                .skip(filters.offset as usize)
                .take(filters.limit as usize)
                .collect())
        }

        async fn count_events_by_contract(
            &self,
            contract_id: &str,
            filters: &EventFilters,
        ) -> Result<i64> {
            Ok(self.filtered_events(contract_id, filters).len() as i64)
        }
    }

    impl FakeStore {
        fn filtered_events(&self, contract_id: &str, filters: &EventFilters) -> Vec<ContractEvent> {
            let mut events = self
                .events
                .iter()
                .filter(|event| event.contract_id == contract_id)
                .filter(|event| {
                    filters
                        .from_ledger
                        .is_none_or(|from_ledger| event.ledger_seq >= from_ledger)
                })
                .filter(|event| {
                    filters
                        .to_ledger
                        .is_none_or(|to_ledger| event.ledger_seq <= to_ledger)
                })
                .filter(|event| {
                    filters.event_type.as_ref().is_none_or(|event_type| {
                        event.event_type.as_ref() == Some(event_type)
                    })
                })
                .cloned()
                .collect::<Vec<_>>();

            events.sort_by(|left, right| {
                right
                    .ledger_seq
                    .cmp(&left.ledger_seq)
                    .then_with(|| right.id.cmp(&left.id))
            });
            events
        }
    }

    fn test_router(events: Vec<ContractEvent>) -> Router {
        create_router_with_store(Arc::new(FakeStore {
            latest_ledger: Some(1_001),
            events: Arc::new(events),
        }))
    }

    fn event(id: i64, contract_id: &str, ledger_seq: i64, event_type: &str) -> ContractEvent {
        ContractEvent {
            id,
            ledger_seq,
            tx_hash: format!("tx-{id}"),
            contract_id: contract_id.to_string(),
            event_type: Some(event_type.to_string()),
            topics: Some("[\"transfer\"]".to_string()),
            data: Some(serde_json::json!({ "amount": id })),
            created_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        }
    }

    async fn json_body(response: Response) -> Value {
        let bytes = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn health_returns_status_and_correlation_id() {
        let response = test_router(vec![])
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .header("x-correlation-id", "test-request-1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("x-correlation-id")
                .unwrap()
                .to_str()
                .unwrap(),
            "test-request-1"
        );

        let body = json_body(response).await;
        assert_eq!(body["status"], "ok");
        assert_eq!(body["database"], "connected");
        assert_eq!(body["latest_ledger"], 1_001);
    }

    #[tokio::test]
    async fn events_route_filters_and_paginates_large_datasets() {
        let mut events = (1..=1_001)
            .map(|seq| event(seq, "contract-a", seq, "transfer"))
            .collect::<Vec<_>>();
        events.push(event(2_000, "contract-b", 2_000, "transfer"));
        events.push(event(2_001, "contract-a", 2_001, "mint"));

        let response = test_router(events)
            .oneshot(
                Request::builder()
                    .uri("/events/contract-a?event_type=transfer&page=3&page_size=500")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = json_body(response).await;
        assert_eq!(body["contract_id"], "contract-a");
        assert_eq!(body["total"], 1_001);
        assert_eq!(body["page"], 3);
        assert_eq!(body["page_size"], 500);
        assert_eq!(body["events"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn events_route_rejects_invalid_pagination() {
        let response = test_router(vec![])
            .oneshot(
                Request::builder()
                    .uri("/events/contract-a?page=0")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = json_body(response).await;
        assert_eq!(body["code"], "bad_request");
        assert_eq!(body["error"], "page must be greater than or equal to 1");
    }
}
