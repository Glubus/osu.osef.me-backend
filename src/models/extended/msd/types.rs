use crate::helpers::common::from_f32;
use crate::helpers::msd::calculate_main_pattern;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use minacalc_rs::Ssr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MSDExtended {
    pub id: Option<i32>,
    pub beatmap_id: Option<i32>,
    pub overall: Option<BigDecimal>,
    pub stream: Option<BigDecimal>,
    pub jumpstream: Option<BigDecimal>,
    pub handstream: Option<BigDecimal>,
    pub stamina: Option<BigDecimal>,
    pub jackspeed: Option<BigDecimal>,
    pub chordjack: Option<BigDecimal>,
    pub technical: Option<BigDecimal>,
    pub rate: Option<BigDecimal>,
    pub main_pattern: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl MSDExtended {
    pub fn from(ssr: Ssr, rate: f32) -> Self {
        Self {
            id: None,
            beatmap_id: None,
            overall: Some(from_f32(ssr.overall)),
            stream: Some(from_f32(ssr.stream)),
            jumpstream: Some(from_f32(ssr.jumpstream)),
            handstream: Some(from_f32(ssr.handstream)),
            stamina: Some(from_f32(ssr.stamina)),
            jackspeed: Some(from_f32(ssr.jackspeed)),
            chordjack: Some(from_f32(ssr.chordjack)),
            technical: Some(from_f32(ssr.technical)),
            rate: Some(from_f32(rate)),
            main_pattern: Some(calculate_main_pattern(&ssr)),
            created_at: None,
            updated_at: None,
        }
    }
}
