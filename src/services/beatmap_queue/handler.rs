use crate::services::beatmap_queue::processor::BeatmapProcessor;
use crate::models::extended::beatmap::BeatmapExtended;
use crate::models::extended::beatmapset::BeatmapsetExtended;
use crate::models::pending_beatmap::PendingBeatmap;
use crate::services::msd_calculator::osu_to_notes;
use crate::helpers::beatmap::osu_file_from_url;
use crate::helpers::beatmap::is_allowed_beatmap;
use crate::models::extended::msd::MSDExtended;
use crate::services::osu_api::OsuApiService;
use crate::models::failed_query::FailedQuery;
use anyhow::Result;
use tracing::{info, error};

pub async fn handle_pending(pending: &PendingBeatmap) -> Result<()> {
    let processor = BeatmapProcessor::instance();
    let db_ref = processor.db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"));
    let _ = PendingBeatmap::delete_by_hash(db_ref?.get_pool(), &pending.hash).await.map_err(|e| error!("Failed to delete pending beatmap: {}", e));
    match processor.process_single_checksum(pending.hash.clone()).await {
        Ok(_) => {
            info!("Beatmap processed with success: {}", pending.hash);
            Ok(())
        }
        Err(e) => {
            let db_ref = processor.db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
            let _ = FailedQuery::insert(db_ref.get_pool(), &pending.hash).await.map_err(|e| error!("Failed to insert failed query: {}", e));
            error!("Error processing checksum {}: {}", pending.hash, e);
            Err(e)
        }
    }
}

impl BeatmapProcessor {
    pub async fn process_single_checksum(&self, checksum: String) -> Result<()> {
        if self.is_already_processed(checksum.clone()).await? {
            return Err(anyhow::anyhow!("Checksum has already been processed: {}", checksum));
        }

        let osu_api = OsuApiService::instance();
        let beatmap_extended = osu_api.beatmap_by_checksum(checksum.clone()).await?;
        if !is_allowed_beatmap(beatmap_extended.mode, beatmap_extended.cs).await 
        {
            return Err(anyhow::anyhow!("Beatmap not allowed"));
        }

        let mut beatmapset = BeatmapsetExtended::from(*beatmap_extended.mapset.clone().unwrap());
        let mut beatmap = BeatmapExtended::from(beatmap_extended);

        let osu_file = osu_file_from_url(&beatmap.file_path).await.map_err(|e| anyhow::anyhow!("Failed to get osu file: {}", e))?;
        let notes = osu_to_notes(&osu_file).map_err(|e| anyhow::anyhow!("Failed to convert osu file to notes: {}", e))?;
        let mut msd: Vec<MSDExtended> = self.calculate_msd(notes).await?;

        self.insert_into_db(&mut beatmapset, &mut beatmap, &mut msd).await?;
        Ok(())
    }

    pub async fn is_already_processed(&self, checksum: String) -> Result<bool> {
        let db_ref = self.db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;

        // Vérifier si un échec a été enregistré
        if FailedQuery::exists_by_hash(db_ref.get_pool(), &checksum).await? {
            return Ok(true);
        }

        // Vérifier si déjà traité
        if BeatmapExtended::exists_by_checksum(db_ref.get_pool(), &checksum).await? {
            return Ok(true);
        }

        Ok(false)
    }

    /// Insère beatmapset, beatmap et MSD dans la DB
    pub async fn insert_into_db(
        &self,
        beatmapset: &mut BeatmapsetExtended,
        beatmap: &mut BeatmapExtended,
        msd: &mut Vec<MSDExtended>,
    ) -> Result<()> {
        let db_ref = self.db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;

        let beatmapset_id = beatmapset.insert_into_db(db_ref.get_pool()).await?;
        beatmap.beatmapset_id = Some(beatmapset_id);

        let beatmap_id = beatmap.insert_into_db(db_ref.get_pool()).await?;

        for msd in msd.iter_mut() {
            msd.beatmap_id = Some(beatmap_id);
            msd.insert_into_db(db_ref.get_pool()).await?;
        }

        Ok(())
    }
}
