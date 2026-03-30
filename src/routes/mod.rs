use axum::Router;

pub mod metrics;
pub mod insights;
pub mod health;

pub fn metrics_routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(metrics::list_metrics))
        .route("/create", axum::routing::post(metrics::create_metric))
        .route("/types", axum::routing::get(metrics::get_metric_types))
}

pub fn insights_routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(insights::get_insights))
        .route("/summary", axum::routing::get(insights::get_health_summary))
}

pub fn health_routes() -> Router {
    Router::new()
        .route("/health", axum::routing::get(health::health_check))
}
