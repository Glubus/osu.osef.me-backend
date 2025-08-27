use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub response_time_ms: u64,
    pub db_connected: bool,
    pub db_response_time_ms: Option<u64>,
    pub status: String,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: DateTime<Utc>,
    pub health_score: u8,
    pub cpu_score: u8,
    pub memory_score: u8,
    pub perf_score: u8,
    pub network_score: u8,
    pub avg_response_time: f64,
    pub system_load: f64,

    pub cpu_usage: f32,
    pub cpu_count: usize,
    pub memory_usage_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub disk_usage_percent: f32,
    pub uptime: u64,
    pub response_time_ms: u64,
    pub db_connected: bool,
    pub db_response_time_ms: Option<u64>,
    pub status: String,

    pub minimal_waittime: u64,
}
