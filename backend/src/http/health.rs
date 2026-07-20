use axum::{extract::State, Json};
use serde::Serialize;

use crate::http::{error::ApiError, AppState};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
    database: &'static str,
}

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, ApiError> {
    sqlx::query("SELECT 1")
        .execute(&state.pool)
        .await
        .map_err(|source| ApiError::service_unavailable("database unavailable", source))?;

    Ok(Json(HealthResponse {
        status: "ok",
        database: "ok",
    }))
}
