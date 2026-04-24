use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{AppState, controllers::search_controller::find_nearby_drivers};

// Search routes module
// This module defines all routes related to search functionality
// Route path: base_url/api/v1/search-service/*
pub fn create_search_routes() -> Router<Arc<AppState>> {
    Router::new().route("/search", get(find_nearby_drivers))
}
