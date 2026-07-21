pub mod city_search;
pub mod confirm_accepting;
pub mod doctor_detail;
pub mod doctor_listings;
pub mod error;
pub mod health;
pub mod rate_limit;
pub mod status_change;
pub mod validation;

use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
pub struct AppState {
    pub pool: crate::db::DbPool,
    pub rate_limiter: rate_limit::RateLimiter,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/health", get(health::health_check))
        .route("/api/cities/search", get(city_search::search_cities))
        .route(
            "/api/cities/{city_slug}/doctors",
            get(doctor_listings::list_by_city),
        )
        .route(
            "/api/doctors/{doctor_id}",
            get(doctor_detail::get_doctor_detail),
        )
        .route(
            "/api/doctors/{doctor_id}/confirm-accepting",
            axum::routing::post(confirm_accepting::confirm_accepting),
        )
        .route(
            "/api/doctors/{doctor_id}/status-change",
            axum::routing::post(status_change::report_status_change),
        )
        .fallback_service(static_frontend())
        .with_state(state)
}

fn static_frontend() -> ServeDir<ServeFile> {
    ServeDir::new("frontend/dist").fallback(ServeFile::new("frontend/dist/index.html"))
}

