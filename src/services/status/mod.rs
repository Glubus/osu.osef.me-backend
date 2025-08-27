pub mod background;
pub mod compute;
pub mod storage;
pub mod types;

// Re-exports to preserve existing API surface
pub use background::start_background_metrics_task;
pub use storage::{get_history, get_metrics_with_fallback};
pub use types::{HistoryEntry, PerformanceMetrics};
