pub mod city_search;
pub mod doctor_listings;
pub mod error;
pub mod health;
pub mod validation;

use axum::{routing::get, Router};

#[derive(Clone)]
pub struct AppState {
    pub pool: crate::db::DbPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health::health_check))
        .route("/api/health", get(health::health_check))
        .route("/api/cities/search", get(city_search::search_cities))
        .route(
            "/api/cities/{city_slug}/doctors",
            get(doctor_listings::list_by_city),
        )
        .fallback(error::not_found)
        .with_state(state)
}

async fn root() -> &'static str {
    "Family Doctor Finder backend"
}
