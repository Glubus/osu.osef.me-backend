use crate::models::extended::beatmapset::BeatmapsetExtended;
use crate::models::extended::complete::query::by_beatmapset_id::find_by_beatmapset_id;
use crate::models::extended::complete::types::BeatmapsetCompleteExtended;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_beatmapset_osu_id(
    pool: &PgPool,
    beatmapset_osu_id: i32,
) -> Result<Option<BeatmapsetCompleteExtended>, SqlxError> {
    let beatmapset = BeatmapsetExtended::find_by_osu_id(pool, beatmapset_osu_id).await?;
    let beatmapset_id = beatmapset.as_ref().map(|b| b.id).unwrap_or(0);
    let beatmap = find_by_beatmapset_id(pool, beatmapset_id).await?;

    Ok(Some(BeatmapsetCompleteExtended {
        beatmapset,
        beatmap,
    }))
}
