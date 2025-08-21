use crate::models::beatmap::pending_beatmap::PendingBeatmap;
use crate::services::beatmap_processor::BeatmapProcessor;
use anyhow::Result;

impl BeatmapProcessor {
    pub async fn add_checksums(&self, checksums: Vec<String>) -> Result<()> {
        let checksums_to_add: Vec<String> = checksums.into_iter().take(50).collect();
        if let Some(db) = &self.db {
            for checksum in checksums_to_add {
                let _ = PendingBeatmap::insert(db.get_pool(), &checksum).await?;
            }
        }
        Ok(())
    }
}
