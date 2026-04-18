use axum::{routing::get, Router};
use sqlx::PgPool;

pub fn create_router(_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
}

async fn health() -> &'static str {
    "ok"
}
