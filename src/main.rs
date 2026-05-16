#![allow(dead_code, unused_imports)]

mod api;
mod config;
mod ingest;
mod storage;
mod types;

use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Stellar Wave — Soroban SQL Sync initializing...");

    // Load configuration from environment
    let config = config::Config::from_env()?;
    info!("Config loaded: {:?}", config);

    // Initialize database connection pool
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    info!("Database connection pool initialized");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| {
            error!("Migration failed: {}", e);
            anyhow::anyhow!("Database migrations failed: {}", e)
        })?;

    info!("Database migrations completed successfully");

    let app = api::create_router(db_pool.clone());
    let listener = TcpListener::bind(config.api_addr).await?;

    info!("REST API listening on http://{}", config.api_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
