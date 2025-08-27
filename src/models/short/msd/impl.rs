use crate::models::short::msd::query::{find_by_beatmap_id, find_by_id};
use crate::models::short::msd::types::MSDShort;
use sqlx::PgPool;

impl MSDShort {
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_beatmap_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_by_beatmap_id(pool, beatmap_id).await
    }
}
