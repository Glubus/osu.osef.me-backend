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


// status moved to services/status
