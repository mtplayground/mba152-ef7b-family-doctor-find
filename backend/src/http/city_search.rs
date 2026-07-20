use axum::{
    extract::{rejection::QueryRejection, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    http::{error::ApiError, AppState},
    services::city_search::{self, CitySearchCriteria, CitySearchResult},
};

const DEFAULT_LIMIT: i64 = 8;
const MAX_LIMIT: u32 = 20;
const MAX_QUERY_CHARS: usize = 80;

#[derive(Debug, Deserialize)]
pub struct CitySearchQuery {
    q: Option<String>,
    query: Option<String>,
    limit: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CitySearchResponse {
    results: Vec<CitySearchResult>,
}

pub async fn search_cities(
    State(state): State<AppState>,
    query: Result<Query<CitySearchQuery>, QueryRejection>,
) -> Result<Json<CitySearchResponse>, ApiError> {
    let Query(query) = query?;
    let criteria = query.into_criteria()?;
    let results = city_search::search_cities(&state.pool, &criteria)
        .await
        .map_err(|source| ApiError::service_unavailable("city search unavailable", source))?;

    Ok(Json(CitySearchResponse { results }))
}

impl CitySearchQuery {
    fn into_criteria(self) -> Result<CitySearchCriteria, ApiError> {
        let raw_query = self
            .q
            .as_deref()
            .or(self.query.as_deref())
            .unwrap_or("")
            .trim()
            .to_string();

        if raw_query.chars().count() > MAX_QUERY_CHARS {
            return Err(ApiError::bad_request(
                "query_too_long",
                format!("query must be {MAX_QUERY_CHARS} characters or fewer"),
            ));
        }

        let limit = match self.limit {
            Some(0) => {
                return Err(ApiError::bad_request(
                    "invalid_limit",
                    "limit must be greater than zero",
                ));
            }
            Some(limit) => i64::from(limit.min(MAX_LIMIT)),
            None => DEFAULT_LIMIT,
        };

        Ok(CitySearchCriteria {
            query: raw_query,
            limit,
        })
    }
}
