use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Metric {
    pub id: Uuid,
    pub metric_type: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateMetric {
    #[validate(length(min = 1, max = 50))]
    pub metric_type: String,
    pub value: f64,
    #[validate(length(min = 1, max = 20))]
    pub unit: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricInsight {
    pub metric_type: String,
    pub average: f64,
    pub trend: TrendDirection,
    pub recommendation: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrendDirection {
    Up,
    Down,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    pub total_metrics: i64,
    pub metric_types: Vec<String>,
    pub date_range: DateRange,
    pub insights: Vec<MetricInsight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
