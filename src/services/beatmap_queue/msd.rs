use super::processor::BeatmapProcessor;
use crate::models::extended::msd::MSDExtended;
use crate::services::beatmap_queue::processor::BeatmapProcessor as Processor;
use crate::services::msd_calculator::calculate_etterna_rating;
use anyhow::Result;
use minacalc_rs::Note;

impl BeatmapProcessor {
    pub async fn calculate_msd(&self, notes: Vec<Note>) -> Result<Vec<MSDExtended>> {
        if let Some(calc) = Processor::get_calc() {
            let rating =
                calculate_etterna_rating(&notes, calc).map_err(|e| anyhow::anyhow!("{}", e))?;
            Ok(rating
                .msds
                .iter()
                .enumerate()
                .map(|(i, msd)| {
                    let rate = 0.7 + 0.1 * i as f32;
                    MSDExtended::from(*msd, rate)
                })
                .collect::<Vec<_>>())
        } else {
            Err(anyhow::anyhow!("Calc not initialized"))
        }
    }
}
