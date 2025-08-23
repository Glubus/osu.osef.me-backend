pub mod beatmap;
pub mod beatmapset;
pub mod msd;

use crate::models::beatmap::extended::beatmap::BeatmapExtended;
use crate::models::beatmap::extended::beatmapset::BeatmapsetExtended;
use crate::models::beatmap::extended::msd::MSDExtended;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Error as SqlxError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapCompleteExtended {
    pub beatmap: Option<BeatmapExtended>,
    pub msd: Option<MSDExtended>,
}


impl BeatmapCompleteExtended {
    pub async fn find_by_beatmapset_id(pool: &PgPool, beatmapset_id: i32) -> Result<Vec<Self>, SqlxError> {
        let query = sqlx::query!(
            r#"
            SELECT b.id, b.osu_id, b.beatmapset_id, b.difficulty, b.difficulty_rating, b.count_circles, 
            b.count_sliders, b.count_spinners, b.max_combo, b.drain_time, b.total_time, 
            b.bpm, b.cs, b.ar, b.od, b.hp, b.mode, b.status, b.file_md5, b.file_path, 
            b.created_at, b.updated_at, m.id as msd_id, m.beatmap_id, m.overall, m.stream, m.jumpstream, 
            m.handstream, m.stamina, m.jackspeed, m.chordjack, m.technical, m.rate, m.main_pattern, m.created_at as msd_created_at,
             m.updated_at as msd_updated_at
            FROM beatmap b
            JOIN msd m ON b.id = m.beatmap_id
            WHERE b.beatmapset_id = $1
            "#,
            beatmapset_id
        ).fetch_all(pool).await?;

        Ok(query.into_iter().map(|r| BeatmapCompleteExtended {
            beatmap: Some(BeatmapExtended {
                id: r.id,
                osu_id: r.osu_id,
                beatmapset_id: r.beatmapset_id,
                difficulty: r.difficulty,
                difficulty_rating: r.difficulty_rating,
                count_circles: r.count_circles,
                count_sliders: r.count_sliders,
                count_spinners: r.count_spinners,
                max_combo: r.max_combo,
                drain_time: r.drain_time,
                total_time: r.total_time,
                bpm: r.bpm,
                cs: r.cs,
                ar: r.ar,
                od: r.od,
                hp: r.hp,
                mode: r.mode,
                status: r.status,
                file_md5: r.file_md5,
                file_path: r.file_path,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }),
            msd: Some(MSDExtended {
                id: Some(r.msd_id),
                beatmap_id: r.beatmap_id,
                overall: r.overall,
                stream: r.stream,
                jumpstream: r.jumpstream,
                handstream: r.handstream,
                stamina: r.stamina,
                jackspeed: r.jackspeed,
                chordjack: r.chordjack,
                technical: r.technical,
                rate: r.rate,
                main_pattern: r.main_pattern,
                created_at: r.msd_created_at,
                updated_at: r.msd_updated_at,
            }),
        }).collect())
    }
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeatmapsetCompleteExtended {
    pub beatmapset: Option<BeatmapsetExtended>,
    pub beatmap: Vec<BeatmapCompleteExtended>,
}


impl BeatmapsetCompleteExtended {
    pub async fn find_by_beatmapset_osu_id(pool: &PgPool, beatmapset_id: i32) -> Result<Option<Self>, SqlxError> {
        let beatmapset = BeatmapsetExtended::find_by_osu_id(pool, beatmapset_id).await?;
        let beatmap = BeatmapCompleteExtended::find_by_beatmapset_id(pool, beatmapset_id).await?;

        Ok(Some(Self { beatmapset, beatmap }))
    }
}