use crate::models::beatmap::beatmap::Beatmap;
use crate::models::beatmap::beatmapset::Beatmapset;
use crate::models::beatmap::failed_query::FailedQuery;
use crate::models::beatmap::pending_beatmap::PendingBeatmap;
use crate::models::ratings::msd::MSD;
use crate::services::beatmap_processor::BeatmapProcessor;
use anyhow::Result;
use std::time::Duration;
use tracing::{error, info};

impl BeatmapProcessor {
    pub async fn add_checksums(checksums: Vec<String>) -> Result<()> {
        let processor = BeatmapProcessor::instance();
        let checksums_to_add: Vec<String> = checksums.into_iter().take(50).collect();
        if let Some(db) = &processor.db {
            for checksum in checksums_to_add {
                let _ = PendingBeatmap::insert(db.get_pool(), &checksum).await?;
            }
        }
        if let Some(processor_arc) = BeatmapProcessor::instance_mut().as_ref() {
            let mut processor = processor_arc.lock().unwrap();
            processor.start_processing_thread();
        }
        Ok(())
    }

    pub async fn pending_beatmap(&self) -> Result<Option<PendingBeatmap>> {
        if let Some(db) = &self.db {
            match PendingBeatmap::oldest(db.get_pool()).await {
                Ok(p) => Ok(p),
                Err(e) => {
                    error!(
                        "Erreur lors de la récupération du plus ancien pending_beatmap: {}",
                        e
                    );
                    Err(anyhow::anyhow!(
                        "Erreur lors de la récupération du plus ancien pending_beatmap: {}",
                        e
                    ))
                }
            }
        } else {
            Err(anyhow::anyhow!("Database not initialized"))
        }
    }

    pub async fn process_queue(&mut self) -> Result<()> {
        loop {
            // Récupérer le plus ancien pending_beatmap
            let maybe_pending = self.pending_beatmap().await?;

            let pending = match maybe_pending {
                Some(p) => p,
                None => {
                    self.is_processing = false;
                    return Ok(());
                }
            };

            info!("Traitement du checksum: {}", pending.hash);

            // Traiter le checksum
            let result = self.process_single_checksum(pending.hash.clone()).await;
            match result {
                Ok(_) => {
                    if let Some(db_ref) = self.db.as_ref() {
                        if let Err(e) =
                            PendingBeatmap::delete_by_id(db_ref.get_pool(), pending.id).await
                        {
                            return Err(anyhow::anyhow!(
                                "Impossible de supprimer pending_beatmap id={}: {}",
                                pending.id,
                                e
                            ));
                        }
                    }
                    // pause only if success
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                Err(e) => {
                    error!("Erreur lors du traitement du checksum {}: {}", pending.hash, e);
                }
            }
        }
    }

    pub async fn is_already_processed(&self, checksum: String) -> Result<bool> {
        if FailedQuery::exists_by_hash(self.db.as_ref().unwrap().get_pool(), &checksum).await? {
            return Ok(true);
        }

        // Vérifier si déjà traité
        if Beatmap::exists_by_checksum(self.db.as_ref().unwrap().get_pool(), &checksum).await? {
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn insert_into_db(
        &self,
        beatmapset: &mut Beatmapset,
        beatmap: &mut Beatmap,
        msd: &mut MSD,
    ) -> Result<()> {
        let db_ref = self
            .db
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
        let beatmapset_id = beatmapset.insert_into_db(db_ref.get_pool()).await?;
        beatmap.beatmapset_id = Some(beatmapset_id);
        let beatmap_id = beatmap.insert_into_db(db_ref.get_pool()).await?;
        msd.beatmap_id = Some(beatmap_id);
        msd.insert_into_db(db_ref.get_pool()).await?;
        Ok(())
    }
}
