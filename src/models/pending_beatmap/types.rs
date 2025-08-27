use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct PendingBeatmap {
    pub id: i32,
    pub hash: String,
    pub created_at: Option<NaiveDateTime>,
}
