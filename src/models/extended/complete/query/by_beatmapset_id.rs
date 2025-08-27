use crate::models::extended::beatmap::BeatmapExtended;
use crate::models::extended::complete::types::BeatmapCompleteExtended;
use crate::models::extended::msd::MSDExtended;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_beatmapset_id(
    pool: &PgPool,
    beatmapset_id: i32,
) -> Result<Vec<BeatmapCompleteExtended>, SqlxError> {
    let query = sqlx::query!(
        r#"
        SELECT b.id, b.osu_id, b.beatmapset_id, b.difficulty, b.difficulty_rating, 
               b.count_circles, b.count_sliders, b.count_spinners, b.max_combo,
               b.drain_time, b.total_time, b.bpm, b.cs, b.ar, b.od, b.hp, b.mode,
               b.status, b.file_md5, b.file_path, b.created_at, b.updated_at,
               m.id as msd_id, m.beatmap_id, m.overall, m.stream, m.jumpstream,
               m.handstream, m.stamina, m.jackspeed, m.chordjack, m.technical,
               m.rate, m.main_pattern, m.created_at as msd_created_at,
               m.updated_at as msd_updated_at
        FROM beatmap b
        JOIN msd m ON b.id = m.beatmap_id
        WHERE b.beatmapset_id = $1
        "#,
        beatmapset_id
    )
    .fetch_all(pool)
    .await?;

    use std::collections::HashMap;

    let mut by_beatmap: HashMap<i32, BeatmapCompleteExtended> = HashMap::new();

    for r in query {
        let entry = by_beatmap.entry(r.id).or_insert_with(|| BeatmapCompleteExtended {
            beatmap: Some(BeatmapExtended {
                id: r.id,
                osu_id: r.osu_id,
                beatmapset_id: r.beatmapset_id,
                difficulty: r.difficulty.clone(),
                difficulty_rating: r.difficulty_rating.clone(),
                count_circles: r.count_circles,
                count_sliders: r.count_sliders,
                count_spinners: r.count_spinners,
                max_combo: r.max_combo,
                drain_time: r.drain_time,
                total_time: r.total_time,
                bpm: r.bpm.clone(),
                cs: r.cs.clone(),
                ar: r.ar.clone(),
                od: r.od.clone(),
                hp: r.hp.clone(),
                mode: r.mode,
                status: r.status.clone(),
                file_md5: r.file_md5.clone(),
                file_path: r.file_path.clone(),
                created_at: r.created_at,
                updated_at: r.updated_at,
            }),
            msd: Vec::new(),
        });

        entry.msd.push(MSDExtended {
            id: Some(r.msd_id),
            beatmap_id: r.beatmap_id,
            overall: r.overall.clone(),
            stream: r.stream.clone(),
            jumpstream: r.jumpstream.clone(),
            handstream: r.handstream.clone(),
            stamina: r.stamina.clone(),
            jackspeed: r.jackspeed.clone(),
            chordjack: r.chordjack.clone(),
            technical: r.technical.clone(),
            rate: r.rate.clone(),
            main_pattern: r.main_pattern.clone(),
            created_at: r.msd_created_at,
            updated_at: r.msd_updated_at,
        });
    }

    Ok(by_beatmap.into_values().collect())
}
