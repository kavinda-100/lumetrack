use std::sync::Arc;

use axum::{Router, routing::get};

pub mod search_routes;
use crate::{
    AppState, controllers::health_check_handler, routes::search_routes::create_search_routes,
};

/// Creates the main API router with all route groups
/// This is where you compose all your route modules together
pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // API v1 routes
        .nest("/api/v1/search-service", api_v1_routes())
        // Specific routes first
        .route("/health", get(health_check_handler))
        // API v2 routes (future expansion)
        // .nest("/api/v2", api_v2_routes(pool))
        .with_state(state)
}

// Helper function to create API v1 routes
fn api_v1_routes() -> Router<Arc<AppState>> {
    Router::new().merge(create_search_routes())
}
