#![allow(dead_code)]

use std::{error::Error, fmt, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::{
    sync::Mutex,
    time::{sleep, Instant},
};

use crate::{config::AppConfig, db::DbPool};

const SEARCH_PATH: &str = "search";
const COUNTRY_CODES: &str = "ca";
const SEARCH_LIMIT: u8 = 1;
const MIN_REQUEST_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct NominatimClient {
    http: reqwest::Client,
    base_url: String,
    last_request_at: Arc<Mutex<Option<Instant>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeocodeResult {
    pub query: String,
    pub display_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub provider: &'static str,
    pub provider_place_id: Option<i64>,
    pub provider_class: Option<String>,
    pub provider_type: Option<String>,
    pub importance: Option<f64>,
}

#[derive(Debug)]
pub enum GeocodeError {
    HttpClient(reqwest::Error),
    Database(sqlx::Error),
}

#[derive(Debug, FromRow)]
struct GeocodeCacheRecord {
    query_text: String,
    display_name: String,
    latitude: f64,
    longitude: f64,
    provider_place_id: Option<i64>,
    provider_class: Option<String>,
    provider_type: Option<String>,
    importance: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct NominatimPlace {
    place_id: Option<i64>,
    lat: String,
    lon: String,
    display_name: String,
    class: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    importance: Option<f64>,
}

#[derive(Debug, Serialize)]
struct SearchParams<'a> {
    q: &'a str,
    format: &'static str,
    limit: u8,
    countrycodes: &'static str,
    addressdetails: u8,
}

impl NominatimClient {
    pub fn from_config(config: &AppConfig) -> Result<Self, GeocodeError> {
        Self::new(
            config.nominatim_base_url.clone(),
            config.nominatim_user_agent.clone(),
        )
    }

    pub fn new(base_url: String, user_agent: String) -> Result<Self, GeocodeError> {
        let http = reqwest::Client::builder()
            .user_agent(user_agent)
            .build()
            .map_err(GeocodeError::HttpClient)?;

        Ok(Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
            last_request_at: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn resolve(
        &self,
        pool: &DbPool,
        query: impl AsRef<str>,
    ) -> Result<Option<GeocodeResult>, GeocodeError> {
        let query = query.as_ref().trim();
        if query.is_empty() {
            return Ok(None);
        }

        let query_key = normalize_query_key(query);
        if let Some(cached_result) = self.cached_result(pool, &query_key).await? {
            return Ok(Some(cached_result));
        }

        let Some(result) = self.fetch_from_nominatim(query).await else {
            return Ok(None);
        };

        if let Err(source) = self.store_cached_result(pool, &query_key, &result).await {
            tracing::warn!(error = ?source, query_key, "failed to store geocode cache result");
        }

        Ok(Some(result))
    }

    async fn cached_result(
        &self,
        pool: &DbPool,
        query_key: &str,
    ) -> Result<Option<GeocodeResult>, GeocodeError> {
        let record = sqlx::query_as::<_, GeocodeCacheRecord>(
            r#"
            SELECT
                query_text,
                display_name,
                latitude,
                longitude,
                provider_place_id,
                provider_class,
                provider_type,
                importance
            FROM geocode_cache
            WHERE query_key = $1
            "#,
        )
        .bind(query_key)
        .fetch_optional(pool)
        .await
        .map_err(GeocodeError::Database)?;

        let Some(record) = record else {
            return Ok(None);
        };

        sqlx::query(
            r#"
            UPDATE geocode_cache
            SET last_used_at = NOW()
            WHERE query_key = $1
            "#,
        )
        .bind(query_key)
        .execute(pool)
        .await
        .map_err(GeocodeError::Database)?;

        Ok(Some(record.into_result()))
    }

    async fn fetch_from_nominatim(&self, query: &str) -> Option<GeocodeResult> {
        let url = format!("{}/{}", self.base_url, SEARCH_PATH);
        let params = SearchParams {
            q: query,
            format: "jsonv2",
            limit: SEARCH_LIMIT,
            countrycodes: COUNTRY_CODES,
            addressdetails: 1,
        };

        self.wait_for_request_slot().await;

        let response = match self.http.get(url).query(&params).send().await {
            Ok(response) => response,
            Err(source) => {
                tracing::warn!(error = ?source, "nominatim request failed");
                return None;
            }
        };

        let status = response.status();
        if !status.is_success() {
            tracing::warn!(status = status.as_u16(), "nominatim returned non-success status");
            return None;
        }

        let places = match response.json::<Vec<NominatimPlace>>().await {
            Ok(places) => places,
            Err(source) => {
                tracing::warn!(error = ?source, "failed to decode nominatim response");
                return None;
            }
        };

        places
            .into_iter()
            .find_map(|place| place.into_result(query.to_string()))
    }

    async fn wait_for_request_slot(&self) {
        let mut last_request_at = self.last_request_at.lock().await;

        if let Some(last_request_at) = *last_request_at {
            let elapsed = last_request_at.elapsed();
            if elapsed < MIN_REQUEST_INTERVAL {
                sleep(MIN_REQUEST_INTERVAL - elapsed).await;
            }
        }

        *last_request_at = Some(Instant::now());
    }

    async fn store_cached_result(
        &self,
        pool: &DbPool,
        query_key: &str,
        result: &GeocodeResult,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO geocode_cache (
                query_key,
                query_text,
                display_name,
                latitude,
                longitude,
                provider_place_id,
                provider_class,
                provider_type,
                importance
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (query_key) DO UPDATE
            SET
                query_text = EXCLUDED.query_text,
                display_name = EXCLUDED.display_name,
                latitude = EXCLUDED.latitude,
                longitude = EXCLUDED.longitude,
                provider_place_id = EXCLUDED.provider_place_id,
                provider_class = EXCLUDED.provider_class,
                provider_type = EXCLUDED.provider_type,
                importance = EXCLUDED.importance,
                last_used_at = NOW()
            "#,
        )
        .bind(query_key)
        .bind(&result.query)
        .bind(&result.display_name)
        .bind(result.latitude)
        .bind(result.longitude)
        .bind(result.provider_place_id)
        .bind(&result.provider_class)
        .bind(&result.provider_type)
        .bind(result.importance)
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl GeocodeCacheRecord {
    fn into_result(self) -> GeocodeResult {
        GeocodeResult {
            query: self.query_text,
            display_name: self.display_name,
            latitude: self.latitude,
            longitude: self.longitude,
            provider: "nominatim",
            provider_place_id: self.provider_place_id,
            provider_class: self.provider_class,
            provider_type: self.provider_type,
            importance: self.importance,
        }
    }
}

impl NominatimPlace {
    fn into_result(self, query: String) -> Option<GeocodeResult> {
        let latitude = self.lat.parse::<f64>().ok()?;
        let longitude = self.lon.parse::<f64>().ok()?;

        if !((-90.0..=90.0).contains(&latitude) && (-180.0..=180.0).contains(&longitude)) {
            return None;
        }

        Some(GeocodeResult {
            query,
            display_name: self.display_name,
            latitude,
            longitude,
            provider: "nominatim",
            provider_place_id: self.place_id,
            provider_class: self.class,
            provider_type: self.kind,
            importance: self.importance,
        })
    }
}

impl fmt::Display for GeocodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpClient(source) => write!(formatter, "failed to initialize geocoder: {source}"),
            Self::Database(source) => write!(formatter, "geocode cache database error: {source}"),
        }
    }
}

impl Error for GeocodeError {}

fn normalize_query_key(query: &str) -> String {
    query.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_query_key_collapses_whitespace_and_case() {
        assert_eq!(
            normalize_query_key("  100 Sample Street   Toronto "),
            "100 sample street toronto"
        );
    }

    #[test]
    fn nominatim_place_rejects_invalid_coordinates() {
        let place = NominatimPlace {
            place_id: Some(1),
            lat: "95".to_string(),
            lon: "-79.3832".to_string(),
            display_name: "Toronto, Ontario, Canada".to_string(),
            class: None,
            kind: None,
            importance: None,
        };

        assert_eq!(place.into_result("Toronto".to_string()), None);
    }
}
