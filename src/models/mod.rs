// Re-export all model modules here
// Example:
// pub mod user;
// pub mod product;
use serde::Deserialize;
pub mod extended;
pub mod failed_query;
pub mod help;
pub mod pending_beatmap;
pub mod short;


#[derive(Debug, Clone, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum MsdPattern {
    Stream,
    Jumpstream,
    Handstream,
    Stamina,
    Jackspeed,
    Chordjack,
    Technical,
}

impl MsdPattern {
    /// Retourne le nom de colonne sécurisé pour ce pattern
    pub fn as_column_name(&self) -> &'static str {
        match self {
            MsdPattern::Stream => "stream",
            MsdPattern::Jumpstream => "jumpstream",
            MsdPattern::Handstream => "handstream",
            MsdPattern::Stamina => "stamina",
            MsdPattern::Jackspeed => "jackspeed",
            MsdPattern::Chordjack => "chordjack",
            MsdPattern::Technical => "technical",
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Filters {
    pub search_term: Option<String>,
    pub overall_min: Option<f64>,
    pub overall_max: Option<f64>,
    pub selected_pattern: Option<MsdPattern>,
    pub pattern_min: Option<f64>,
    pub pattern_max: Option<f64>,
    pub bpm_min: Option<f64>,
    pub bpm_max: Option<f64>,
    pub total_time_min: Option<i32>,
    pub total_time_max: Option<i32>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}


// status moved to services/status
