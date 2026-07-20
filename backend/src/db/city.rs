#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
pub struct City {
    pub id: i64,
    pub name: String,
    pub province_code: String,
    pub country_code: String,
    pub slug: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
pub struct CityArea {
    pub id: i64,
    pub city_id: i64,
    pub name: String,
    pub slug: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewCity {
    pub name: String,
    pub province_code: String,
    pub slug: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewCityArea {
    pub city_id: i64,
    pub name: String,
    pub slug: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
