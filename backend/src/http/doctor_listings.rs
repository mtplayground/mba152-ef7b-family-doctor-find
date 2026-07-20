use axum::{
    extract::{rejection::PathRejection, rejection::QueryRejection, Path, Query, State},
    Json,
};
use chrono::Utc;
use serde::Deserialize;

use crate::{
    http::{error::ApiError, AppState},
    services::doctor_listings::{self, CityDoctorListings, DoctorListingCriteria},
};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: u32 = 100;
const MAX_SLUG_CHARS: usize = 120;

#[derive(Debug, Deserialize)]
pub struct DoctorListingsPath {
    city_slug: String,
}

#[derive(Debug, Deserialize)]
pub struct DoctorListingsQuery {
    area: Option<String>,
    limit: Option<u32>,
}

pub async fn list_by_city(
    State(state): State<AppState>,
    path: Result<Path<DoctorListingsPath>, PathRejection>,
    query: Result<Query<DoctorListingsQuery>, QueryRejection>,
) -> Result<Json<CityDoctorListings>, ApiError> {
    let Path(path) = path?;
    let Query(query) = query?;
    let criteria = into_criteria(path, query)?;
    let city_slug = criteria.city_slug.clone();

    let result = doctor_listings::list_doctors_by_city(&state.pool, criteria, Utc::now())
        .await
        .map_err(|source| ApiError::service_unavailable("doctor listings unavailable", source))?;

    let Some(result) = result else {
        return Err(ApiError::resource_not_found(
            "city_not_found",
            format!("No city found for slug {city_slug:?}"),
        ));
    };

    Ok(Json(result))
}

fn into_criteria(
    path: DoctorListingsPath,
    query: DoctorListingsQuery,
) -> Result<DoctorListingCriteria, ApiError> {
    let city_slug = clean_slug(path.city_slug, "city_slug")?;
    let area_slug = query
        .area
        .map(|area| clean_slug(area, "area"))
        .transpose()?;
    let limit = match query.limit {
        Some(0) => {
            return Err(ApiError::bad_request(
                "invalid_limit",
                "limit must be greater than zero",
            ));
        }
        Some(limit) => limit.min(MAX_LIMIT) as usize,
        None => DEFAULT_LIMIT,
    };

    Ok(DoctorListingCriteria {
        city_slug,
        area_slug,
        limit,
    })
}

fn clean_slug(slug: String, field: &'static str) -> Result<String, ApiError> {
    let slug = slug.trim().to_string();

    if slug.is_empty() {
        return Err(ApiError::bad_request(
            "invalid_slug",
            format!("{field} must not be empty"),
        ));
    }

    if slug.chars().count() > MAX_SLUG_CHARS {
        return Err(ApiError::bad_request(
            "invalid_slug",
            format!("{field} must be {MAX_SLUG_CHARS} characters or fewer"),
        ));
    }

    if !is_slug(&slug) {
        return Err(ApiError::bad_request(
            "invalid_slug",
            format!("{field} must use lowercase letters, numbers, and hyphens"),
        ));
    }

    Ok(slug)
}

fn is_slug(value: &str) -> bool {
    let mut previous_was_hyphen = false;
    let mut saw_character = false;

    for character in value.chars() {
        let valid_character = character.is_ascii_lowercase() || character.is_ascii_digit();
        if valid_character {
            previous_was_hyphen = false;
            saw_character = true;
            continue;
        }

        if character == '-' && saw_character && !previous_was_hyphen {
            previous_was_hyphen = true;
            continue;
        }

        return false;
    }

    saw_character && !previous_was_hyphen
}
