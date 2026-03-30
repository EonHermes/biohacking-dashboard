use axum::Json;
use serde_json::json;

use crate::models::ApiResponse;

pub async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(json!({
        "status": "healthy",
        "service": "biohacking-dashboard",
        "version": env!("CARGO_PKG_VERSION")
    })))
}
