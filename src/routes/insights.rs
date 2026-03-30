use axum::Json;

use crate::{
    models::{ApiResponse, HealthSummary},
    services::{insights as insights_service, metrics as metrics_service},
};

pub async fn get_insights() -> Result<Json<ApiResponse<Vec<crate::models::MetricInsight>>>, (axum::http::StatusCode, Json<ApiResponse<()>>)> {
    let metrics = match metrics_service::get_metrics(None, None, None).await {
        Ok(m) => m,
        Err(e) => return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(e.to_string()))
        )),
    };

    match insights_service::calculate_insights(metrics).await {
        Ok(insights) => Ok(Json(ApiResponse::ok(insights))),
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(e.to_string()))
        )),
    }
}

pub async fn get_health_summary() -> Result<Json<ApiResponse<HealthSummary>>, (axum::http::StatusCode, Json<ApiResponse<()>>)> {
    let metrics = match metrics_service::get_metrics(None, None, None).await {
        Ok(m) => m,
        Err(e) => return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(e.to_string()))
        )),
    };

    let total = metrics.len() as i64;
    let metric_types = match metrics_service::get_metric_types().await {
        Ok(t) => t,
        Err(e) => return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(e.to_string()))
        )),
    };

    let insights = match insights_service::calculate_insights(metrics.clone()).await {
        Ok(i) => i,
        Err(e) => return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(e.to_string()))
        )),
    };

    // Calculate date range
    let (start, end) = if metrics.is_empty() {
        let now = chrono::Utc::now();
        (now, now)
    } else {
        let timestamps: Vec<_> = metrics.iter().map(|m| m.timestamp).collect();
        let start = *timestamps.iter().min().unwrap();
        let end = *timestamps.iter().max().unwrap();
        (start, end)
    };

    let summary = HealthSummary {
        total_metrics: total,
        metric_types,
        date_range: crate::models::DateRange { start, end },
        insights,
    };

    Ok(Json(ApiResponse::ok(summary)))
}
