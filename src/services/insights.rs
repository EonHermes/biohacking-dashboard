use std::collections::HashMap;

use crate::{
    errors::AppError,
    models::{Metric, MetricInsight, TrendDirection},
};

pub async fn calculate_insights(metrics: Vec<Metric>) -> Result<Vec<MetricInsight>, AppError> {
    // Group metrics by type
    let mut grouped: HashMap<String, Vec<f64>> = HashMap::new();
    
    for metric in &metrics {
        grouped
            .entry(metric.metric_type.clone())
            .or_insert_with(Vec::new)
            .push(metric.value);
    }

    let mut insights = Vec::new();

    for (metric_type, values) in grouped {
        if values.is_empty() {
            continue;
        }

        // Calculate statistics
        let avg = values.iter().sum::<f64>() / values.len() as f64;
        let variance = calculate_variance(&values);
        let std_dev = variance.sqrt();
        
        // Determine trend (compare first half vs second half)
        let trend = determine_trend(&values);
        
        // Generate recommendation based on metric type and trend
        let recommendation = generate_recommendation(&metric_type, avg, &trend, values.len());
        
        // Calculate confidence based on sample size
        let confidence = calculate_confidence(values.len());

        insights.push(MetricInsight {
            metric_type,
            average: avg,
            trend,
            recommendation,
            confidence,
        });
    }

    Ok(insights)
}

fn calculate_variance(values: &[f64]) -> f64 {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    values
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64
}

fn determine_trend(values: &[f64]) -> TrendDirection {
    if values.len() < 2 {
        return TrendDirection::Stable;
    }

    let mid = values.len() / 2;
    let first_half_avg: f64 = values[..mid].iter().sum::<f64>() / mid as f64;
    let second_half_avg: f64 = values[mid..].iter().sum::<f64>() / (values.len() - mid) as f64;

    let change_pct = (second_half_avg - first_half_avg).abs() / first_half_avg;
    
    if change_pct < 0.05 {
        TrendDirection::Stable
    } else if second_half_avg > first_half_avg {
        TrendDirection::Up
    } else {
        TrendDirection::Down
    }
}

fn generate_recommendation(metric_type: &str, avg: f64, trend: &TrendDirection, sample_size: usize) -> String {
    match metric_type.to_lowercase().as_str() {
        "sleep_hours" => {
            if avg < 7.0 {
                "Consider increasing sleep duration for better health outcomes.".to_string()
            } else if avg > 9.0 {
                "Your sleep duration is above average. Consider maintaining a consistent schedule.".to_string()
            } else {
                "Great! Your sleep duration is in the healthy range (7-9 hours).".to_string()
            }
        },
        "steps" => {
            if avg < 5000.0 {
                "Try to increase daily steps. Aim for at least 7,000-10,000 steps per day.".to_string()
            } else if avg > 12000.0 {
                "Excellent activity level! Keep up the great work.".to_string()
            } else {
                format!("Your average of {:.0} steps is good. Consider aiming for 10,000.", avg)
            }
        },
        "heart_rate" => {
            if avg < 60.0 {
                "Resting heart rate below 60 may indicate excellent fitness or warrant medical consultation.".to_string()
            } else if avg > 100.0 {
                "Elevated heart rate detected. Consider consulting a healthcare professional.".to_string()
            } else {
                format!("Your average heart rate of {:.0} bpm is within normal range.", avg)
            }
        },
        "water_intake" => {
            if avg < 2000.0 {
                "Increase water intake. Aim for at least 2-3 liters per day.".to_string()
            } else {
                format!("Great hydration! Your average of {:.0}ml is excellent.", avg)
            }
        },
        _ => {
            match trend {
                TrendDirection::Up => format!("{} is trending upward. Monitor if this aligns with your goals.", metric_type),
                TrendDirection::Down => format!("{} is trending downward. Consider if adjustments are needed.", metric_type),
                TrendDirection::Stable => format!("{} has remained stable at an average of {:.2}.", metric_type, avg),
            }
        }
    }
}

fn calculate_confidence(sample_size: usize) -> f64 {
    // Simple confidence based on sample size (0-1 scale)
    let min_samples = 3;
    let max_samples = 30;
    
    if sample_size < min_samples {
        0.3 + (sample_size as f64 / min_samples as f64) * 0.2
    } else if sample_size > max_samples {
        0.95
    } else {
        0.5 + ((sample_size - min_samples) as f64 / (max_samples - min_samples) as f64) * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_variance() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let variance = calculate_variance(&values);
        assert!((variance - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_determine_trend_up() {
        let values = vec![1.0, 1.5, 2.0, 3.0, 4.0, 5.0];
        let trend = determine_trend(&values);
        assert_eq!(trend, TrendDirection::Up);
    }

    #[test]
    fn test_determine_trend_stable() {
        let values = vec![5.0, 5.1, 4.9, 5.0, 5.05, 4.95];
        let trend = determine_trend(&values);
        assert_eq!(trend, TrendDirection::Stable);
    }

    #[test]
    fn test_calculate_confidence() {
        assert!(calculate_confidence(1) < 0.5);
        assert!(calculate_confidence(15) > 0.7);
        assert!(calculate_confidence(50) > 0.9);
    }
}
