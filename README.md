# Biohacking Dashboard 💪

Track health metrics with ML-powered insights. A privacy-first health tracking dashboard that helps you optimize your well-being through data-driven recommendations.

![Status](https://img.shields.io/badge/status-WIP-blue)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-green)

## 🌟 Features

- **Multi-Metric Tracking**: Log sleep, steps, heart rate, water intake, and custom metrics
- **ML-Powered Insights**: Automatic trend analysis and personalized recommendations
- **Privacy-First**: All data stored locally, no cloud dependencies
- **RESTful API**: Clean JSON API for easy integration
- **Real-time Analytics**: Calculate health summaries and insights on-demand
- **Extensible**: Easy to add new metric types and insight algorithms

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+
- SQLite (included via sqlx)

### Installation

```bash
# Clone the repository
git clone https://github.com/EonHermes/biohacking-dashboard.git
cd biohacking-dashboard

# Set environment variables (optional)
export HOST=0.0.0.0
export PORT=3000
export DATABASE_URL="sqlite:biohacking.db?mode=rwc"

# Build and run
cargo run --release
```

## 📖 API Documentation

### Base URL
`http://localhost:3000`

### Endpoints

#### Health Check
```bash
GET /health
```

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "service": "biohacking-dashboard",
    "version": "0.1.0"
  }
}
```

#### List Metrics
```bash
GET /metrics?metric_type=sleep_hours&limit=50&offset=0
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "metric_type": "sleep_hours",
      "value": 7.5,
      "unit": "hours",
      "timestamp": "2024-03-30T08:00:00Z",
      "source": "apple_watch",
      "created_at": "2024-03-30T08:05:00Z"
    }
  ]
}
```

#### Create Metric
```bash
POST /metrics/create
Content-Type: application/json

{
  "metric_type": "steps",
  "value": 8542,
  "unit": "steps",
  "timestamp": "2024-03-30T23:59:59Z",
  "source": "fitbit"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "metric_type": "steps",
    "value": 8542,
    "unit": "steps",
    "timestamp": "2024-03-30T23:59:59Z",
    "source": "fitbit",
    "created_at": "2024-03-31T00:00:00Z"
  }
}
```

#### Get Metric Types
```bash
GET /metrics/types
```

**Response:**
```json
{
  "success": true,
  "data": ["sleep_hours", "steps", "heart_rate", "water_intake"]
}
```

#### Get Insights
```bash
GET /insights
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "metric_type": "sleep_hours",
      "average": 7.2,
      "trend": "stable",
      "recommendation": "Great! Your sleep duration is in the healthy range (7-9 hours).",
      "confidence": 0.85
    }
  ]
}
```

#### Get Health Summary
```bash
GET /insights/summary
```

**Response:**
```json
{
  "success": true,
  "data": {
    "total_metrics": 150,
    "metric_types": ["sleep_hours", "steps", "heart_rate"],
    "date_range": {
      "start": "2024-03-01T00:00:00Z",
      "end": "2024-03-30T23:59:59Z"
    },
    "insights": [...]
  }
}
```

## 🔬 ML Insights Engine

The dashboard uses statistical analysis to provide actionable insights:

### Trend Detection
- Compares first half vs second half of data points
- Classifies as **Up**, **Down**, or **Stable** (±5% threshold)
- Confidence scoring based on sample size

### Recommendations
Pre-built recommendations for common metrics:
- **Sleep Hours**: Optimal range 7-9 hours
- **Steps**: Target 10,000 daily steps
- **Heart Rate**: Normal range 60-100 bpm
- **Water Intake**: Aim for 2-3 liters/day

### Confidence Scoring
- < 3 samples: 30-50% confidence
- 3-30 samples: 50-95% confidence (linear scaling)
- > 30 samples: 95% confidence

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Run specific test suite
cargo test insights::tests
```

**Test Coverage:**
- ✅ Metric creation and retrieval
- ✅ Trend detection algorithms
- ✅ Variance calculations
- ✅ Confidence scoring
- ✅ Recommendation generation

## 🏗️ Architecture

```
biohacking-dashboard/
├── src/
│   ├── main.rs          # Application entry point
│   ├── config.rs        # Configuration management
│   ├── models.rs        # Data structures
│   ├── errors.rs        # Error handling
│   ├── routes/          # HTTP route handlers
│   │   ├── mod.rs
│   │   ├── metrics.rs
│   │   ├── insights.rs
│   │   └── health.rs
│   └── services/        # Business logic
│       ├── mod.rs
│       ├── database.rs  # Database operations
│       ├── metrics.rs   # Metric CRUD
│       └── insights.rs  # ML analysis
├── migrations/          # Database migrations
├── Cargo.toml
└── README.md
```

## 🔧 Tech Stack

- **Backend**: Rust with Axum framework
- **Database**: SQLite via sqlx (async, type-safe)
- **ML/Stats**: statrs for statistical analysis
- **Validation**: validator crate
- **Logging**: tracing + tracing-subscriber
- **Serialization**: serde + serde_json

## 🌐 Future Enhancements

- [ ] React frontend dashboard
- [ ] Wearable integrations (Apple Health, Google Fit, Fitbit)
- [ ] Mobile app (Tauri desktop + React Native mobile)
- [ ] Advanced ML models (time-series forecasting)
- [ ] Data export (CSV, JSON, FHIR)
- [ ] Multi-user support with authentication
- [ ] Real-time WebSocket updates

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 👤 Author

**EonHermes**

Building privacy-first health tools with Rust 🦀

---

*Your health data stays on your device. No cloud, no tracking, just you and your data.*
