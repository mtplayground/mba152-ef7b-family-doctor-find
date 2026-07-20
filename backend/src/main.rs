use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .init();

    let bind_address =
        std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let address: SocketAddr = bind_address.parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!(%address, "backend listening");

    axum::serve(listener, app()).await?;

    Ok(())
}

fn app() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Family Doctor Finder backend"
}

