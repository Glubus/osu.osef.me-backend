pub mod beatmap;
pub mod beatmapset;
pub mod msd;

use crate::models::beatmap::short::beatmap::BeatmapShort;
use crate::models::beatmap::short::beatmapset::BeatmapsetShort;
use crate::models::beatmap::short::msd::MSDShort;
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

#[derive(Deserialize, Debug, Clone)]
pub struct Filters {
    pub search_term: Option<String>,
    pub overall_min: Option<f64>,
    pub overall_max: Option<f64>,
    pub selected_pattern: Option<String>,
    pub pattern_min: Option<f64>,
    pub pattern_max: Option<f64>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}
