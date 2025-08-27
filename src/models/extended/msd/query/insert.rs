use crate::models::extended::msd::types::MSDExtended;
use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError>;
}

#[async_trait]
impl Insert for MSDExtended {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO msd (
                beatmap_id, overall, stream, jumpstream, handstream,
                stamina, jackspeed, chordjack, technical, rate, main_pattern
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            RETURNING id
            "#,
            self.beatmap_id,
            self.overall.as_ref(),
            self.stream.as_ref(),
            self.jumpstream.as_ref(),
            self.handstream.as_ref(),
            self.stamina.as_ref(),
            self.jackspeed.as_ref(),
            self.chordjack.as_ref(),
            self.technical.as_ref(),
            self.rate.as_ref(),
            self.main_pattern.as_deref()
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }
}
