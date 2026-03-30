use axum::{extract::Query, Json};
use serde::Deserialize;

use crate::{
    errors::AppError,
    models::{CreateMetric, Metric, ApiResponse},
    services::metrics as metrics_service,
};

#[derive(Debug, Deserialize)]
pub struct ListMetricsParams {
    #[serde(default)]
    metric_type: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    100
}

pub async fn list_metrics(
    Query(params): Query<ListMetricsParams>,
) -> Result<Json<ApiResponse<Vec<Metric>>>, AppError> {
    let metrics = metrics_service::get_metrics(params.metric_type, Some(params.limit), Some(params.offset)).await?;
    Ok(Json(ApiResponse::ok(metrics)))
}

pub async fn create_metric(
    Json(metric): Json<CreateMetric>,
) -> Result<Json<ApiResponse<Metric>>, (axum::http::StatusCode, Json<ApiResponse<()>>)> {
    match metrics_service::create_metric(metric).await {
        Ok(created) => Ok(Json(ApiResponse::ok(created))),
        Err(e) => Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(e.to_string()))
        )),
    }
}

pub async fn get_metric_types() -> Result<Json<ApiResponse<Vec<String>>>, AppError> {
    let types = metrics_service::get_metric_types().await?;
    Ok(Json(ApiResponse::ok(types)))
}
