use crate::models::short::beatmapset::query::{find_by_id, find_by_osu_id};
use crate::models::short::beatmapset::types::BeatmapsetShort;
use sqlx::PgPool;

impl BeatmapsetShort {
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_osu_id(pool, osu_id).await
    }
}
