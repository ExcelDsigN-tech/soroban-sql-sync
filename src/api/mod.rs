//! REST API endpoints for health and event queries.

pub mod handlers;
pub mod errors;

pub use handlers::create_router;
pub use errors::ApiError;
