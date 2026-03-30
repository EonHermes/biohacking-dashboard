CREATE TABLE IF NOT EXISTS metrics (
    id TEXT PRIMARY KEY,
    metric_type TEXT NOT NULL CHECK(length(metric_type) > 0 AND length(metric_type) <= 50),
    value REAL NOT NULL,
    unit TEXT NOT NULL CHECK(length(unit) > 0 AND length(unit) <= 20),
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    source TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_metrics_type ON metrics(metric_type);
CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics(timestamp);
CREATE INDEX IF NOT EXISTS idx_metrics_type_timestamp ON metrics(metric_type, timestamp);
