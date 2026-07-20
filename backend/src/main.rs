mod db;

use axum::{extract::State, routing::get, Router};
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    pool: db::DbPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL").map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("DATABASE_URL must be set for PostgreSQL access: {err}"),
        )
    })?;
    let pool = db::connect(&database_url).await?;
    db::run_migrations(&pool).await?;

    let bind_address =
        std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let address: SocketAddr = bind_address.parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!(%address, "backend listening");

    axum::serve(listener, app(pool)).await?;

    Ok(())
}

fn app(pool: db::DbPool) -> Router {
    Router::new()
        .route("/", get(root))
        .with_state(AppState { pool })
}

async fn root(State(state): State<AppState>) -> &'static str {
    let _database_pool_is_open = !state.pool.is_closed();

    "Family Doctor Finder backend"
}
