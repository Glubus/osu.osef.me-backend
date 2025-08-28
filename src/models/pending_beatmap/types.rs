use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct PendingBeatmap {
    pub id: i32,
    pub hash: String,
    pub osu_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}
