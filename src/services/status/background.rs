use crate::config::Config;
use crate::db::DatabaseManager;
use tokio::time::{Duration, interval};

use super::compute::process_metrics_and_store;

const HISTORY_INTERVAL_SECONDS: u64 = 300;

pub async fn start_background_metrics_task(_db: DatabaseManager, config: Config) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(HISTORY_INTERVAL_SECONDS));
        tokio::time::sleep(Duration::from_secs(5)).await;
        loop {
            interval.tick().await;
            process_metrics_and_store(&config).await;
        }
    });
}
