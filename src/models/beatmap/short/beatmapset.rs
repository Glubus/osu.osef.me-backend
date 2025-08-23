use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapsetShort {
    pub id: Option<i32>,
    pub osu_id: i32,
    pub artist: String,
    pub title: String,
    pub creator: String,
    pub cover_url: Option<String>,
}
