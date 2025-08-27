use crate::models::short::beatmapset::BeatmapsetShort;
use crate::models::short::complete::query::by_beatmapset_id::find_by_beatmapset_id;
use crate::models::short::complete::types::BeatmapsetCompleteShort;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_beatmapset_osu_id(
    pool: &PgPool,
    beatmapset_osu_id: i32,
) -> Result<Option<BeatmapsetCompleteShort>, SqlxError> {
    let beatmapset = BeatmapsetShort::find_by_osu_id(pool, beatmapset_osu_id).await?;
    let beatmapset_id = beatmapset.as_ref().map(|b| b.id.unwrap_or(0)).unwrap_or(0);
    let beatmap = find_by_beatmapset_id(pool, beatmapset_id).await?;

    Ok(Some(BeatmapsetCompleteShort {
        beatmapset,
        beatmap,
    }))
}
