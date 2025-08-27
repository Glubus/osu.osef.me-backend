use chrono::NaiveDateTime;
use sqlx::{Error as SqlxError, PgPool};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FailedQuery {
    pub id: i32,
    pub hash: String,
    pub created_at: Option<NaiveDateTime>,
}
