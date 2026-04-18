//! REST API endpoints for health and event queries.

pub mod errors;
pub mod handlers;

#[allow(unused_imports)]
pub use errors::ApiError;
#[allow(unused_imports)]
pub use handlers::create_router;
