use crate::models::pending_beatmap::PendingBeatmap;
use crate::services::beatmap_queue::processor::BeatmapProcessor;
use anyhow::Result;
use tracing::error;

impl BeatmapProcessor {
    pub async fn pending_beatmap(&self) -> Result<Option<PendingBeatmap>> {
        if let Some(db) = &self.db {
            match PendingBeatmap::oldest(db.get_pool()).await {
                Ok(p) => Ok(p),
                Err(e) => {
                    error!(
                        "Error retrieving oldest pending_beatmap: {}",
                        e
                    );
                    Err(anyhow::anyhow!(
                        "Error retrieving oldest pending_beatmap: {}",
                        e
                    ))
                }
            }
        } else {
            Err(anyhow::anyhow!("Database not initialized"))
        }
    }
}
