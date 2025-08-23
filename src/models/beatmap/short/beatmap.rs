use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapShort {
    pub id: Option<i32>,
    pub osu_id: i32,
    pub difficulty: String,
    pub difficulty_rating: BigDecimal,
    pub mode: i32,
    pub status: String,
}
