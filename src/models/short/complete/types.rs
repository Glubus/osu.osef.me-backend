use crate::models::short::beatmap::BeatmapShort;
use crate::models::short::beatmapset::BeatmapsetShort;
use crate::models::short::msd::MSDShort;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapCompleteShort {
    pub beatmap: Option<BeatmapShort>,
    pub msd: Option<MSDShort>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapsetCompleteShort {
    pub beatmapset: Option<BeatmapsetShort>,
    pub beatmap: Vec<BeatmapCompleteShort>,
}
