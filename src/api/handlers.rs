use axum::{extract::{Path, Query, State}, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    api::errors::ApiError,
    storage::{models::ContractEvent, repository::EventRepository},
};

#[derive(Clone)]
struct ApiState {
    pool: PgPool,
}

#[derive(Debug, Deserialize)]
pub struct EventQuery {
    page: Option<i64>,
    page_size: Option<i64>,
    from_ledger: Option<i64>,
    to_ledger: Option<i64>,
    event_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthBody {
    status: &'static str,
    database: &'static str,
    ingestion: &'static str,
    latest_ledger: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct EventsBody {
    events: Vec<ContractEvent>,
    total: i64,
    page: i64,
    page_size: i64,
}

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/events/:contract_id", get(events_by_contract))
        .with_state(ApiState { pool })
}

async fn health(State(state): State<ApiState>) -> Result<Json<HealthBody>, ApiError> {
    let repo = EventRepository::new(state.pool);
    let latest_ledger = repo.get_latest_ledger().await?;

    Ok(Json(HealthBody {
        status: "ok",
        database: "ok",
        ingestion: "ready",
        latest_ledger,
    }))
}

async fn events_by_contract(
    State(state): State<ApiState>,
    Path(contract_id): Path<String>,
    Query(query): Query<EventQuery>,
) -> Result<Json<EventsBody>, ApiError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(50).min(100);

    if page < 1 {
        return Err(ApiError::BadRequest("page must be at least 1".to_string()));
    }
    if page_size < 1 {
        return Err(ApiError::BadRequest("page_size must be at least 1".to_string()));
    }
    if let (Some(from), Some(to)) = (query.from_ledger, query.to_ledger) {
        if from > to {
            return Err(ApiError::BadRequest(
                "from_ledger must be less than or equal to to_ledger".to_string(),
            ));
        }
    }

    let offset = (page - 1) * page_size;
    let repo = EventRepository::new(state.pool);
    let event_type = query.event_type.as_deref();
    let events = repo
        .get_events_by_contract(
            &contract_id,
            page_size,
            offset,
            query.from_ledger,
            query.to_ledger,
            event_type,
        )
        .await?;
    let total = repo
        .count_events_by_contract(&contract_id, query.from_ledger, query.to_ledger, event_type)
        .await?;

    Ok(Json(EventsBody {
        events,
        total,
        page,
        page_size,
    }))
}
