#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
pub struct Clinic {
    pub id: i64,
    pub city_area_id: i64,
    pub name: String,
    pub slug: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub municipality: String,
    pub province_code: String,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
pub struct FamilyDoctor {
    pub id: i64,
    pub clinic_id: i64,
    pub full_name: String,
    pub slug: String,
    pub credentials: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub profile_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewClinic {
    pub city_area_id: i64,
    pub name: String,
    pub slug: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub municipality: String,
    pub province_code: String,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NewFamilyDoctor {
    pub clinic_id: i64,
    pub full_name: String,
    pub slug: String,
    pub credentials: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub profile_url: Option<String>,
}
