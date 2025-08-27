use crate::models::extended::complete::query::{find_by_beatmapset_id, find_by_beatmapset_osu_id};
use crate::models::extended::complete::types::{
    BeatmapCompleteExtended, BeatmapsetCompleteExtended,
};
use sqlx::PgPool;

impl BeatmapCompleteExtended {
    pub async fn find_by_beatmapset_id(
        pool: &PgPool,
        beatmapset_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_by_beatmapset_id(pool, beatmapset_id).await
    }
}

impl BeatmapsetCompleteExtended {
    pub async fn find_by_beatmapset_osu_id(
        pool: &PgPool,
        beatmapset_osu_id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        find_by_beatmapset_osu_id(pool, beatmapset_osu_id).await
    }
}
