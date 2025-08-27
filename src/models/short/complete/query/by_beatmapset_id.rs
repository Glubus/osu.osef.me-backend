use crate::models::short::beatmap::BeatmapShort;
use crate::models::short::complete::types::BeatmapCompleteShort;
use crate::models::short::msd::MSDShort;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_beatmapset_id(
    pool: &PgPool,
    beatmapset_id: i32,
) -> Result<Vec<BeatmapCompleteShort>, SqlxError> {
    let query = sqlx::query!(
        r#"
        SELECT 
            b.id, b.osu_id, b.difficulty, b.difficulty_rating, b.mode, b.status,
            m.id as "msd_id?", m.overall, m.main_pattern
        FROM beatmap b
        LEFT JOIN msd m ON b.id = m.beatmap_id
        WHERE b.beatmapset_id = $1
        "#,
        beatmapset_id
    )
    .fetch_all(pool)
    .await?;

    Ok(query
        .into_iter()
        .map(|r| BeatmapCompleteShort {
            beatmap: Some(BeatmapShort {
                id: Some(r.id),
                osu_id: r.osu_id,
                difficulty: r.difficulty,
                difficulty_rating: r.difficulty_rating,
                mode: r.mode,
                status: r.status,
            }),
            msd: r.msd_id.map(|id| MSDShort {
                id: Some(id),
                overall: r.overall,
                main_pattern: r.main_pattern,
            }),
        })
        .collect())
}
