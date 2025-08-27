use chrono::Utc;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::Mutex;

use super::types::{HistoryEntry, PerformanceMetrics};

const MAX_HISTORY_SIZE: usize = 50;
const PERFORMANCE_QUEUE_SIZE: usize = 5;

pub static METRICS_HISTORY: Lazy<Mutex<VecDeque<HistoryEntry>>> =
    Lazy::new(|| Mutex::new(VecDeque::with_capacity(MAX_HISTORY_SIZE)));

pub static PERFORMANCE_QUEUE: Lazy<Mutex<VecDeque<PerformanceMetrics>>> =
    Lazy::new(|| Mutex::new(VecDeque::with_capacity(PERFORMANCE_QUEUE_SIZE)));

pub static LATEST_CACHED_METRICS: Lazy<Mutex<Option<PerformanceMetrics>>> =
    Lazy::new(|| Mutex::new(None));

const HISTORY_INTERVAL_SECONDS: i64 = 300;

pub fn add_history_entry(entry: HistoryEntry) {
    let mut history = METRICS_HISTORY.lock().unwrap();
    if let Some(last_entry) = history.back() {
        let time_diff = entry.timestamp.signed_duration_since(last_entry.timestamp);
        if time_diff.num_seconds() < HISTORY_INTERVAL_SECONDS {
            return;
        }
    }
    if history.len() >= MAX_HISTORY_SIZE {
        history.pop_front();
    }
    history.push_back(entry);
}

pub fn add_performance_metrics(metrics: PerformanceMetrics) {
    let mut queue = PERFORMANCE_QUEUE.lock().unwrap();
    if queue.len() >= PERFORMANCE_QUEUE_SIZE {
        queue.pop_front();
    }
    queue.push_back(metrics);
}

pub fn get_latest_performance_metrics() -> Option<PerformanceMetrics> {
    let cached = LATEST_CACHED_METRICS.lock().unwrap();
    cached.clone()
}

pub fn should_use_cache() -> bool {
    if let Some(metrics) = get_latest_performance_metrics() {
        let now = Utc::now();
        let time_diff = now.signed_duration_since(metrics.timestamp);
        time_diff.num_seconds() < metrics.minimal_waittime as i64
    } else {
        false
    }
}

pub fn get_metrics_with_fallback() -> Option<PerformanceMetrics> {
    if should_use_cache() {
        get_latest_performance_metrics()
    } else {
        get_latest_performance_metrics()
    }
}

pub fn get_history() -> Vec<HistoryEntry> {
    let history = METRICS_HISTORY.lock().unwrap();
    history.iter().cloned().collect()
}
