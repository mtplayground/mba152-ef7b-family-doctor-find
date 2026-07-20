mod config;
mod db;
mod http;

use axum::Router;
use http::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .init();

    let config = config::AppConfig::from_env()?;
    tracing::info!(
        bind_address = %config.bind_address,
        allowed_cors_origin = ?config.allowed_cors_origin.as_deref(),
        osm_tile_url_template = %config.osm_tile_url_template.as_str(),
        nominatim_base_url = %config.nominatim_base_url.as_str(),
        nominatim_user_agent = %config.nominatim_user_agent.as_str(),
        rate_limit_window_secs = config.rate_limit_window_secs,
        rate_limit_max_requests = config.rate_limit_max_requests,
        "configuration loaded",
    );

    let pool = db::connect(&config.database_url).await?;
    db::run_migrations(&pool).await?;

    let listener = tokio::net::TcpListener::bind(config.bind_address).await?;

    tracing::info!(address = %config.bind_address, "backend listening");

    axum::serve(listener, app(pool)).await?;

    Ok(())
}

fn app(pool: db::DbPool) -> Router {
    http::router(AppState { pool })
}
