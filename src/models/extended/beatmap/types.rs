use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use rosu_v2::model::beatmap::BeatmapExtended as BmExtended;
use serde::{Deserialize, Serialize};

use crate::helpers::{
    beatmap::{build_file_path, rank_status_to_string},
    common::from_f32,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeatmapExtended {
    pub id: i32,
    pub osu_id: Option<i32>,
    pub beatmapset_id: Option<i32>,
    pub difficulty: String,
    pub difficulty_rating: BigDecimal,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub max_combo: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: BigDecimal,
    pub cs: BigDecimal,
    pub ar: BigDecimal,
    pub od: BigDecimal,
    pub hp: BigDecimal,
    pub mode: i32,
    pub status: String,
    pub file_md5: String,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<BmExtended> for BeatmapExtended {
    fn from(b: BmExtended) -> Self {
        Self {
            id: 0,
            osu_id: Some(b.map_id as i32),
            beatmapset_id: None,
            difficulty: b.version,
            difficulty_rating: from_f32(b.stars),
            count_circles: b.count_circles as i32,
            count_sliders: b.count_sliders as i32,
            count_spinners: b.count_spinners as i32,
            max_combo: b.max_combo.unwrap_or(0) as i32,
            drain_time: b.seconds_drain as i32,
            total_time: b.seconds_total as i32,
            bpm: from_f32(b.bpm),
            cs: from_f32(b.cs),
            ar: from_f32(b.ar),
            od: from_f32(b.od),
            hp: from_f32(b.hp),
            mode: b.mode as i32,
            status: rank_status_to_string(&b.status),
            file_md5: b.checksum.unwrap_or_default(),
            file_path: build_file_path(b.map_id),
            created_at: None,
            updated_at: None,
        }
    }
}
