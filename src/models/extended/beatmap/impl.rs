use crate::models::extended::beatmap::query::{
    Insert, exists_by_checksum, find_by_id, get_beatmapset_id,
};
use crate::models::extended::beatmap::types::BeatmapExtended;
use sqlx::PgPool;

impl BeatmapExtended {
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, sqlx::Error> {
        self.insert(pool).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn exists_by_checksum(pool: &PgPool, checksum: &str) -> Result<bool, sqlx::Error> {
        exists_by_checksum(pool, checksum).await
    }

    pub async fn get_beatmapset_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Option<i32>, sqlx::Error> {
        get_beatmapset_id(pool, beatmap_id).await
    }
}
