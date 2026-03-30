use axum::{http::StatusCode, response::{Json, IntoResponse}, extract::rejection::JsonRejection};
use thiserror::Error;

use crate::models::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid metric data: {0}")]
    InvalidMetric(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Insight calculation failed: {0}")]
    InsightError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AppError::InvalidMetric(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err)),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::InsightError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}

pub async fn json_rejection_handler(rejection: JsonRejection) -> impl IntoResponse {
    (StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error("Invalid JSON".to_string())))
}
