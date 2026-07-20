use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::http::HeaderMap;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<RateLimitState>>,
    submission_window: Duration,
    submission_max_requests: u32,
    repeat_window: Duration,
    repeat_max_requests: u32,
}

#[derive(Debug, Default)]
struct RateLimitState {
    buckets: HashMap<RateLimitKey, RateLimitBucket>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RateLimitKey {
    client_id: String,
    scope: String,
}

#[derive(Debug, Clone)]
struct RateLimitBucket {
    window_started: Instant,
    window: Duration,
    count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimitError {
    pub retry_after_secs: u64,
}

impl RateLimiter {
    pub fn new(
        submission_window_secs: u64,
        submission_max_requests: u32,
        repeat_window_secs: u64,
        repeat_max_requests: u32,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RateLimitState::default())),
            submission_window: Duration::from_secs(submission_window_secs),
            submission_max_requests,
            repeat_window: Duration::from_secs(repeat_window_secs),
            repeat_max_requests,
        }
    }

    pub async fn check_submission(&self, client_id: &str) -> Result<(), RateLimitError> {
        self.check(
            RateLimitKey {
                client_id: client_id.to_string(),
                scope: "submissions".to_string(),
            },
            self.submission_window,
            self.submission_max_requests,
        )
        .await
    }

    pub async fn check_listing_report(
        &self,
        client_id: &str,
        doctor_id: i64,
    ) -> Result<(), RateLimitError> {
        self.check(
            RateLimitKey {
                client_id: client_id.to_string(),
                scope: format!("doctor:{doctor_id}:reports"),
            },
            self.repeat_window,
            self.repeat_max_requests,
        )
        .await
    }

    async fn check(
        &self,
        key: RateLimitKey,
        window: Duration,
        max_requests: u32,
    ) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let mut state = self.inner.lock().await;

        state
            .buckets
            .retain(|_, bucket| {
                now.saturating_duration_since(bucket.window_started) < bucket.window
            });

        let bucket = state.buckets.entry(key).or_insert(RateLimitBucket {
            window_started: now,
            window,
            count: 0,
        });

        if now.saturating_duration_since(bucket.window_started) >= bucket.window {
            bucket.window_started = now;
            bucket.window = window;
            bucket.count = 0;
        }

        if bucket.count >= max_requests {
            let elapsed = now.saturating_duration_since(bucket.window_started);
            let retry_after_secs = bucket
                .window
                .as_secs()
                .saturating_sub(elapsed.as_secs())
                .max(1);
            return Err(RateLimitError { retry_after_secs });
        }

        bucket.count = bucket.count.saturating_add(1);
        Ok(())
    }
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "rate limit exceeded; retry after {} seconds",
            self.retry_after_secs
        )
    }
}

impl std::error::Error for RateLimitError {}

pub fn client_identifier(headers: &HeaderMap) -> String {
    forwarded_client(headers).unwrap_or_else(|| "unknown-client".to_string())
}

fn forwarded_client(headers: &HeaderMap) -> Option<String> {
    const HEADER_NAMES: [&str; 3] = ["cf-connecting-ip", "x-real-ip", "x-forwarded-for"];

    HEADER_NAMES.iter().find_map(|name| {
        headers
            .get(*name)
            .and_then(|value| value.to_str().ok())
            .and_then(first_non_empty_header_value)
    })
}

fn first_non_empty_header_value(value: &str) -> Option<String> {
    value
        .split(',')
        .map(str::trim)
        .find(|part| !part.is_empty())
        .map(|part| part.chars().take(128).collect())
}
