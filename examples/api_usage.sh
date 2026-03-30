#!/bin/bash
# Example usage of Biohacking Dashboard API

BASE_URL="http://localhost:3000"

echo "=== Health Check ==="
curl -s "$BASE_URL/health" | jq .

echo -e "\n=== Create Sleep Metric ==="
curl -s -X POST "$BASE_URL/metrics/create" \
  -H "Content-Type: application/json" \
  -d '{
    "metric_type": "sleep_hours",
    "value": 7.5,
    "unit": "hours",
    "source": "apple_watch"
  }' | jq .

echo -e "\n=== Create Steps Metric ==="
curl -s -X POST "$BASE_URL/metrics/create" \
  -H "Content-Type: application/json" \
  -d '{
    "metric_type": "steps",
    "value": 8542,
    "unit": "steps",
    "source": "fitbit"
  }' | jq .

echo -e "\n=== Create Heart Rate Metric ==="
curl -s -X POST "$BASE_URL/metrics/create" \
  -H "Content-Type: application/json" \
  -d '{
    "metric_type": "heart_rate",
    "value": 72,
    "unit": "bpm",
    "source": "whoop"
  }' | jq .

echo -e "\n=== Create Water Intake Metric ==="
curl -s -X POST "$BASE_URL/metrics/create" \
  -H "Content-Type: application/json" \
  -d '{
    "metric_type": "water_intake",
    "value": 2500,
    "unit": "ml",
    "source": "manual"
  }' | jq .

echo -e "\n=== List All Metrics ==="
curl -s "$BASE_URL/metrics" | jq .

echo -e "\n=== Get Metric Types ==="
curl -s "$BASE_URL/metrics/types" | jq .

echo -e "\n=== Get Insights ==="
curl -s "$BASE_URL/insights" | jq .

echo -e "\n=== Get Health Summary ==="
curl -s "$BASE_URL/insights/summary" | jq .
