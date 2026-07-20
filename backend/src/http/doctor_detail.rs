use axum::{
    extract::{rejection::PathRejection, Path, State},
    Json,
};
use chrono::Utc;
use serde::Deserialize;

use crate::{
    http::{error::ApiError, AppState},
    services::doctor_detail::{self, DoctorDetail},
};

#[derive(Debug, Deserialize)]
pub struct DoctorDetailPath {
    doctor_id: i64,
}

pub async fn get_doctor_detail(
    State(state): State<AppState>,
    path: Result<Path<DoctorDetailPath>, PathRejection>,
) -> Result<Json<DoctorDetail>, ApiError> {
    let Path(path) = path?;
    let doctor_id = validate_doctor_id(path.doctor_id)?;

    let detail = doctor_detail::get_doctor_detail(&state.pool, doctor_id, Utc::now())
        .await
        .map_err(|source| ApiError::service_unavailable("doctor detail unavailable", source))?;

    let Some(detail) = detail else {
        return Err(ApiError::resource_not_found(
            "doctor_not_found",
            format!("No doctor listing found for id {doctor_id}"),
        ));
    };

    Ok(Json(detail))
}

fn validate_doctor_id(doctor_id: i64) -> Result<i64, ApiError> {
    if doctor_id <= 0 {
        return Err(ApiError::bad_request(
            "invalid_doctor_id",
            "doctor_id must be greater than zero",
        ));
    }

    Ok(doctor_id)
}
