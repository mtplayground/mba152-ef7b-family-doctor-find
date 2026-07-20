use std::{env, error::Error, fmt, net::SocketAddr};

const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0:8080";
const DEFAULT_OSM_TILE_URL_TEMPLATE: &str = "https://tile.openstreetmap.org/{z}/{x}/{y}.png";
const DEFAULT_NOMINATIM_BASE_URL: &str = "https://nominatim.openstreetmap.org";
const DEFAULT_NOMINATIM_USER_AGENT: &str = "family-doctor-finder/0.1";
const DEFAULT_RATE_LIMIT_WINDOW_SECS: u64 = 60;
const DEFAULT_RATE_LIMIT_MAX_REQUESTS: u32 = 30;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_address: SocketAddr,
    pub allowed_cors_origin: Option<String>,
    pub osm_tile_url_template: String,
    pub nominatim_base_url: String,
    pub nominatim_user_agent: String,
    pub rate_limit_window_secs: u64,
    pub rate_limit_max_requests: u32,
}

#[derive(Debug)]
pub enum ConfigError {
    Missing {
        key: &'static str,
    },
    Invalid {
        key: &'static str,
        value: String,
        reason: &'static str,
    },
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = postgres_url("DATABASE_URL", required_env("DATABASE_URL")?)?;
        let bind_address = parse_socket_addr(
            "BIND_ADDRESS",
            env_or_default("BIND_ADDRESS", DEFAULT_BIND_ADDRESS),
        )?;
        let allowed_cors_origin = optional_http_url("ALLOWED_CORS_ORIGIN")?;
        let osm_tile_url_template = http_url(
            "OSM_TILE_URL_TEMPLATE",
            env_or_default("OSM_TILE_URL_TEMPLATE", DEFAULT_OSM_TILE_URL_TEMPLATE),
        )?;
        let nominatim_base_url = http_url(
            "NOMINATIM_BASE_URL",
            env_or_default("NOMINATIM_BASE_URL", DEFAULT_NOMINATIM_BASE_URL),
        )?;
        let nominatim_user_agent = non_empty_env_or_default(
            "NOMINATIM_USER_AGENT",
            DEFAULT_NOMINATIM_USER_AGENT,
        )?;
        let rate_limit_window_secs = positive_u64(
            "RATE_LIMIT_WINDOW_SECS",
            env::var("RATE_LIMIT_WINDOW_SECS").ok(),
            DEFAULT_RATE_LIMIT_WINDOW_SECS,
        )?;
        let rate_limit_max_requests = positive_u32(
            "RATE_LIMIT_MAX_REQUESTS",
            env::var("RATE_LIMIT_MAX_REQUESTS").ok(),
            DEFAULT_RATE_LIMIT_MAX_REQUESTS,
        )?;

        Ok(Self {
            database_url,
            bind_address,
            allowed_cors_origin,
            osm_tile_url_template,
            nominatim_base_url,
            nominatim_user_agent,
            rate_limit_window_secs,
            rate_limit_max_requests,
        })
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { key } => write!(formatter, "{key} must be set"),
            Self::Invalid { key, value, reason } => {
                write!(formatter, "{key} has invalid value {value:?}: {reason}")
            }
        }
    }
}

impl Error for ConfigError {}

fn required_env(key: &'static str) -> Result<String, ConfigError> {
    let value = env::var(key).map_err(|_| ConfigError::Missing { key })?;
    non_empty(key, value)
}

fn env_or_default(key: &'static str, default: &'static str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn non_empty_env_or_default(
    key: &'static str,
    default: &'static str,
) -> Result<String, ConfigError> {
    non_empty(key, env_or_default(key, default))
}

fn optional_env(key: &'static str) -> Option<String> {
    env::var(key).ok().and_then(|value| {
        if value.trim().is_empty() {
            None
        } else {
            Some(value)
        }
    })
}

fn non_empty(key: &'static str, value: String) -> Result<String, ConfigError> {
    if value.trim().is_empty() {
        Err(ConfigError::Missing { key })
    } else {
        Ok(value)
    }
}

fn parse_socket_addr(key: &'static str, value: String) -> Result<SocketAddr, ConfigError> {
    value.parse::<SocketAddr>().map_err(|_| ConfigError::Invalid {
        key,
        value,
        reason: "expected socket address in host:port form",
    })
}

fn optional_http_url(key: &'static str) -> Result<Option<String>, ConfigError> {
    optional_env(key).map(|value| http_url(key, value)).transpose()
}

fn http_url(key: &'static str, value: String) -> Result<String, ConfigError> {
    let value = non_empty(key, value)?;
    if value.starts_with("http://") || value.starts_with("https://") {
        Ok(value)
    } else {
        Err(ConfigError::Invalid {
            key,
            value,
            reason: "expected http:// or https:// URL",
        })
    }
}

fn postgres_url(key: &'static str, value: String) -> Result<String, ConfigError> {
    let value = non_empty(key, value)?;
    if value.starts_with("postgres://") || value.starts_with("postgresql://") {
        Ok(value)
    } else {
        Err(ConfigError::Invalid {
            key,
            value,
            reason: "expected postgres:// or postgresql:// URL",
        })
    }
}

fn positive_u64(
    key: &'static str,
    value: Option<String>,
    default: u64,
) -> Result<u64, ConfigError> {
    match value {
        Some(raw) if raw.trim().is_empty() => Ok(default),
        Some(raw) => raw.parse::<u64>().ok().filter(|parsed| *parsed > 0).ok_or(
            ConfigError::Invalid {
                key,
                value: raw,
                reason: "expected positive integer",
            },
        ),
        None => Ok(default),
    }
}

fn positive_u32(
    key: &'static str,
    value: Option<String>,
    default: u32,
) -> Result<u32, ConfigError> {
    match value {
        Some(raw) if raw.trim().is_empty() => Ok(default),
        Some(raw) => raw.parse::<u32>().ok().filter(|parsed| *parsed > 0).ok_or(
            ConfigError::Invalid {
                key,
                value: raw,
                reason: "expected positive integer",
            },
        ),
        None => Ok(default),
    }
}
