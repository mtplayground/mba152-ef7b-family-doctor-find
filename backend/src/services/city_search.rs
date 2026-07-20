#![allow(dead_code)]

use serde::Serialize;
use sqlx::FromRow;

use crate::db::DbPool;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitySearchCriteria {
    pub query: String,
    pub limit: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CitySearchResult {
    pub kind: String,
    pub label: String,
    pub city_id: i64,
    pub city_name: String,
    pub city_slug: String,
    pub province_code: String,
    pub area_id: Option<i64>,
    pub area_name: Option<String>,
    pub area_slug: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

pub async fn search_cities(
    pool: &DbPool,
    criteria: &CitySearchCriteria,
) -> Result<Vec<CitySearchResult>, sqlx::Error> {
    let contains_pattern = contains_like_pattern(&criteria.query);
    let prefix_pattern = prefix_like_pattern(&criteria.query);

    sqlx::query_as::<_, CitySearchResult>(
        r#"
        SELECT
            kind,
            label,
            city_id,
            city_name,
            city_slug,
            province_code,
            area_id,
            area_name,
            area_slug,
            latitude,
            longitude
        FROM (
            SELECT
                'city'::TEXT AS kind,
                cities.name || ', ' || cities.province_code AS label,
                cities.id AS city_id,
                cities.name AS city_name,
                cities.slug AS city_slug,
                cities.province_code AS province_code,
                NULL::BIGINT AS area_id,
                NULL::TEXT AS area_name,
                NULL::TEXT AS area_slug,
                cities.latitude AS latitude,
                cities.longitude AS longitude
            FROM cities
            WHERE $1 = ''
                OR cities.name ILIKE $2 ESCAPE E'\\'
                OR cities.slug ILIKE $2 ESCAPE E'\\'

            UNION ALL

            SELECT
                'area'::TEXT AS kind,
                city_areas.name || ', ' || cities.name || ', ' || cities.province_code AS label,
                cities.id AS city_id,
                cities.name AS city_name,
                cities.slug AS city_slug,
                cities.province_code AS province_code,
                city_areas.id AS area_id,
                city_areas.name AS area_name,
                city_areas.slug AS area_slug,
                COALESCE(city_areas.latitude, cities.latitude) AS latitude,
                COALESCE(city_areas.longitude, cities.longitude) AS longitude
            FROM city_areas
            JOIN cities ON cities.id = city_areas.city_id
            WHERE $1 = ''
                OR city_areas.name ILIKE $2 ESCAPE E'\\'
                OR city_areas.slug ILIKE $2 ESCAPE E'\\'
                OR cities.name ILIKE $2 ESCAPE E'\\'
                OR cities.slug ILIKE $2 ESCAPE E'\\'
                OR city_areas.name || ' ' || cities.name ILIKE $2 ESCAPE E'\\'
        ) matches
        ORDER BY
            CASE
                WHEN city_name ILIKE $3 ESCAPE E'\\' THEN 0
                WHEN area_name ILIKE $3 ESCAPE E'\\' THEN 1
                WHEN label ILIKE $3 ESCAPE E'\\' THEN 2
                ELSE 3
            END,
            CASE kind
                WHEN 'city' THEN 0
                ELSE 1
            END,
            city_name,
            area_name NULLS FIRST
        LIMIT $4
        "#,
    )
    .bind(&criteria.query)
    .bind(contains_pattern)
    .bind(prefix_pattern)
    .bind(criteria.limit)
    .fetch_all(pool)
    .await
}

fn contains_like_pattern(query: &str) -> String {
    format!("%{}%", escape_like(query))
}

fn prefix_like_pattern(query: &str) -> String {
    format!("{}%", escape_like(query))
}

fn escape_like(value: &str) -> String {
    value.chars().fold(String::new(), |mut escaped, character| {
        if matches!(character, '%' | '_' | '\\') {
            escaped.push('\\');
        }
        escaped.push(character);
        escaped
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn like_patterns_escape_wildcards() {
        assert_eq!(contains_like_pattern(r"tor_%\west"), r"%tor\_\%\\west%");
        assert_eq!(prefix_like_pattern("north"), "north%");
    }
}
