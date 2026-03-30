mod config;
mod handlers;
mod models;
mod routes;
mod services;
mod errors;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "biohacking_dashboard=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;
    
    // Initialize database
    services::database::init_db(&config.database_url).await?;

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .merge(routes::metrics_routes())
        .merge(routes::insights_routes())
        .merge(routes::health_routes());

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("Starting Biohacking Dashboard on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Biohacking Dashboard API - Health metrics tracking with ML insights"
}
