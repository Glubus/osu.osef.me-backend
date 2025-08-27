use crate::models::extended::beatmap::BeatmapExtended;
use crate::models::extended::beatmapset::BeatmapsetExtended;
use crate::models::extended::msd::MSDExtended;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapCompleteExtended {
    pub beatmap: Option<BeatmapExtended>,
    pub msd: Vec<MSDExtended>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapsetCompleteExtended {
    pub beatmapset: Option<BeatmapsetExtended>,
    pub beatmap: Vec<BeatmapCompleteExtended>,
}
