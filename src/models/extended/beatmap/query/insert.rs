use crate::models::extended::beatmap::types::BeatmapExtended;
use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError>;
}

#[async_trait]
impl Insert for BeatmapExtended {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO beatmap (
                osu_id, beatmapset_id, difficulty, difficulty_rating,
                count_circles, count_sliders, count_spinners, max_combo,
                drain_time, total_time, bpm, cs, ar, od, hp, mode,
                status, file_md5, file_path
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)
            RETURNING id
            "#,
            self.osu_id,
            self.beatmapset_id,
            self.difficulty,
            self.difficulty_rating,
            self.count_circles,
            self.count_sliders,
            self.count_spinners,
            self.max_combo,
            self.drain_time,
            self.total_time,
            self.bpm,
            self.cs,
            self.ar,
            self.od,
            self.hp,
            self.mode,
            self.status,
            self.file_md5,
            self.file_path
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }
}
