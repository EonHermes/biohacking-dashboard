use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::AppError,
    models::{CreateMetric, Metric},
};

use super::database::get_pool;

pub async fn create_metric(metric: CreateMetric) -> Result<Metric, AppError> {
    metric.validate().map_err(|e| {
        AppError::Validation(format!("Validation failed: {}", e))
    })?;

    let pool = get_pool();
    let id = Uuid::new_v4();
    let timestamp = metric.timestamp.unwrap_or_else(Utc::now);

    sqlx::query_as::<_, Metric>(
        r#"
        INSERT INTO metrics (id, metric_type, value, unit, timestamp, source, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(&metric.metric_type)
    .bind(metric.value)
    .bind(&metric.unit)
    .bind(timestamp)
    .bind(&metric.source)
    .bind(Utc::now())
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn get_metrics(
    metric_type: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Metric>, AppError> {
    let pool = get_pool();
    
    let query = if metric_type.is_some() {
        format!(
            "SELECT * FROM metrics WHERE metric_type = ? ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            limit.unwrap_or(100),
            offset.unwrap_or(0)
        )
    } else {
        format!(
            "SELECT * FROM metrics ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            limit.unwrap_or(100),
            offset.unwrap_or(0)
        )
    };

    if let Some(mt) = metric_type {
        sqlx::query_as::<_, Metric>(&query)
            .bind(&mt)
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)
    } else {
        sqlx::query_as::<_, Metric>(&query)
            .fetch_all(pool)
            .await
            .map_err(AppError::Database)
    }
}

pub async fn get_metric_types() -> Result<Vec<String>, AppError> {
    let pool = get_pool();
    
    sqlx::query_scalar("SELECT DISTINCT metric_type FROM metrics")
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)
}
