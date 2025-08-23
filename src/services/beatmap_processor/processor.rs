use crate::helpers::beatmap::osu_file_from_url;
use crate::helpers::beatmap_processor::is_allowed_beatmap;
use crate::models::beatmap::beatmap::Beatmap;
use crate::models::beatmap::beatmapset::Beatmapset;
use crate::models::beatmap::pending_beatmap::PendingBeatmap;
use crate::models::ratings::msd::MSD;
use crate::services::beatmap_processor::BeatmapProcessor;
use crate::services::beatmap_processor::{CALC};
use crate::services::etterna_rating::calculate_etterna_rating;
use crate::services::etterna_rating::osu_to_notes;
use crate::services::osu_api::OsuApiService;
use minacalc_rs::{Note};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tracing::{error, info};

static PROCESSING_THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

impl BeatmapProcessor {
    pub fn start_processing_thread(&mut self) {
        // Vérifier si le thread est déjà en cours
        if PROCESSING_THREAD_RUNNING.load(Ordering::Relaxed) {
            error!("Thread de traitement déjà en cours, impossible de relancer");
            return;
        }

        // Marquer le thread comme démarré
        PROCESSING_THREAD_RUNNING.store(true, Ordering::Relaxed);

        // Démarrer le thread
        self.spawn_processing_thread();
    }

    fn spawn_processing_thread(&self) {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Thread de traitement des beatmaps démarré");
                // Safety Measure :
                let processor = BeatmapProcessor::instance();
                let maybe_pending = processor.pending_beatmap().await;

                if let Ok(Some(pending)) = maybe_pending {
                    if let Some(db_ref) = processor.db.as_ref() {
                        if let Err(e) =
                            PendingBeatmap::delete_by_id(db_ref.get_pool(), pending.id).await
                        {
                            error!(
                                "Impossible de supprimer pending_beatmap id={}: {}",
                                pending.id, e
                            );
                        }
                    }
                }

                loop {
                    let processor = BeatmapProcessor::instance();
                    let maybe_pending = processor.pending_beatmap().await;

                    if let Ok(Some(pending)) = maybe_pending {
                        info!("Traitement du beatmap pending: {}", pending.hash);

                        let result = processor
                            .process_single_checksum(pending.hash.clone())
                            .await;

                        // Toujours supprimer le pending, même en cas d'erreur
                        if let Some(db_ref) = processor.db.as_ref() {
                            if let Err(e) =
                                PendingBeatmap::delete_by_id(db_ref.get_pool(), pending.id).await
                            {
                                error!(
                                    "Impossible de supprimer pending_beatmap id={}: {}",
                                    pending.id, e
                                );
                            }
                        }

                        match result {
                            Ok(_) => {
                                info!("Beatmap traité avec succès: {}", pending.hash);
                                // Break de 500ms quand on traite avec succès
                                tokio::time::sleep(Duration::from_millis(500)).await;
                            }
                            Err(e) => {
                                error!(
                                    "Erreur lors du traitement du checksum {}: {}",
                                    pending.hash, e
                                );
                                // Pas de break en cas d'erreur, on continue directement
                            }
                        }
                    } else {
                        // Pas de pending, break de 10 secondes
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                }
            });
        });
    }

    pub async fn calculate_msd(&self, notes: Vec<Note>) -> Result<MSD, anyhow::Error> {
        unsafe {
            let calc_ptr = &raw const CALC;

            if let Some(calc) = (*calc_ptr).as_ref() {
                let calculate =
                    calculate_etterna_rating(&notes, calc).map_err(|e| anyhow::anyhow!("{}", e))?;
                let msd = MSD::from(calculate.msds[3]);
                Ok(msd)
            } else {
                Err(anyhow::anyhow!("Calc non initialisé"))
            }
        }
    }

    pub async fn process_single_checksum(&self, checksum: String) -> Result<(), anyhow::Error> {
        let db_ref = self
            .db
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;

        if self.is_already_processed(checksum.clone()).await? {
            return Err(anyhow::anyhow!(
                "Checksum has already been processed: {}",
                checksum
            ));
        }

        let osu_api = OsuApiService::instance();
        let beatmap_extended = osu_api.beatmap_by_checksum(checksum.clone()).await?;

        if !is_allowed_beatmap(beatmap_extended.mode, beatmap_extended.cs).await {
            return Err(anyhow::anyhow!("Beatmap not allowed"));
        }

        let beatmapset = Beatmapset::from(*beatmap_extended.mapset.clone().unwrap());
        let mut beatmap = Beatmap::from(beatmap_extended);

        let osu_file = osu_file_from_url(&beatmap.file_path)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        let notes = osu_to_notes(&osu_file).map_err(|e| anyhow::anyhow!("{}", e))?;

        let mut msd: MSD = self.calculate_msd(notes).await?;

        let beatmapset_id = match tokio::time::timeout(
            std::time::Duration::from_secs(30), // 30 secondes de timeout
            beatmapset.insert_into_db(db_ref.get_pool()),
        )
        .await
        {
            Ok(Ok(id)) => id,
            Ok(Err(e)) => {
                return Err(anyhow::anyhow!("Failed to insert beatmapset: {}", e));
            }
            Err(_) => {
                return Err(anyhow::anyhow!("Timeout inserting beatmapset"));
            }
        };

        beatmap.beatmapset_id = Some(beatmapset_id);
        let beatmap_id = match beatmap.insert_into_db(db_ref.get_pool()).await {
            Ok(id) => id,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to insert beatmap: {}", e));
            }
        };

        msd.beatmap_id = Some(beatmap_id);
        match msd.insert_into_db(db_ref.get_pool()).await {
            Ok(_) => (),
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to insert MSD: {}", e));
            }
        }

        if let Some(db_ref) = self.db.as_ref() {
            if let Err(e) = PendingBeatmap::delete_by_hash(db_ref.get_pool(), &checksum).await {
                error!(
                    "Impossible de supprimer pending_beatmap hash={}: {}",
                    checksum, e
                );
            }
        }

        Ok(())
    }
}
