use crate::helpers::beatmap::{build_file_path, rank_status_to_string};
use crate::helpers::common::from_f32;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use rosu_v2::model::beatmap::BeatmapExtended as BmExtended;
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, PgPool};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeatmapExtended {
    pub id: i32,
    pub osu_id: Option<i32>,
    pub beatmapset_id: Option<i32>,
    pub difficulty: String,
    pub difficulty_rating: BigDecimal,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub max_combo: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: BigDecimal,
    pub cs: BigDecimal,
    pub ar: BigDecimal,
    pub od: BigDecimal,
    pub hp: BigDecimal,
    pub mode: i32,
    pub status: String,
    pub file_md5: String,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError>;
}

#[async_trait]
impl Insert for BeatmapExtended {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO beatmap (
                osu_id, beatmapset_id, difficulty, difficulty_rating, 
                count_circles, count_sliders, count_spinners, max_combo,
                drain_time, total_time, bpm, cs, ar, od, hp, mode, 
                status, file_md5, file_path
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
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
        );

        let result = query.fetch_one(pool).await?;
        Ok(result.id)
    }
}

impl From<BmExtended> for BeatmapExtended {
    fn from(beatmap: BmExtended) -> Self {
        Self {
            id: 0,
            beatmapset_id: None,
            osu_id: Some(beatmap.map_id as i32),
            difficulty: beatmap.version,
            difficulty_rating: from_f32(beatmap.stars),
            count_circles: beatmap.count_circles as i32,
            count_sliders: beatmap.count_sliders as i32,
            count_spinners: beatmap.count_spinners as i32,
            max_combo: beatmap.max_combo.unwrap_or(0) as i32,
            drain_time: beatmap.seconds_drain as i32,
            total_time: beatmap.seconds_total as i32,
            bpm: from_f32(beatmap.bpm),
            cs: from_f32(beatmap.cs),
            ar: from_f32(beatmap.ar),
            od: from_f32(beatmap.od),
            hp: from_f32(beatmap.hp),
            mode: beatmap.mode as i32,
            status: rank_status_to_string(&beatmap.status),
            file_md5: beatmap.checksum.unwrap_or_default(),
            file_path: build_file_path(beatmap.map_id),
            created_at: None,
            updated_at: None,
        }
    }
}

impl BeatmapExtended {
    /// Insère un beatmap dans la base de données
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        self.insert(pool).await
    }

    /// Récupère un beatmap par son ID
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, SqlxError> {
        let query = sqlx::query_as!(
            Self,
            r#"
            SELECT 
                id, osu_id, beatmapset_id, difficulty, difficulty_rating,
                count_circles, count_sliders, count_spinners, max_combo,
                drain_time, total_time, bpm, cs, ar, od, hp, mode,
                status, file_md5, file_path, created_at, updated_at
            FROM beatmap 
            WHERE id = $1
            "#,
            id
        );

        query.fetch_optional(pool).await
    }

    /// Vérifie si un beatmap existe déjà par son file_md5 (checksum)
    pub async fn exists_by_checksum(pool: &PgPool, checksum: &str) -> Result<bool, SqlxError> {
        let query = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM beatmap WHERE file_md5 = $1
            ) as exists
            "#,
            checksum
        );

        let result = query.fetch_one(pool).await?;
        Ok(result.exists.unwrap_or(false))
    }

    pub async fn get_beatmapset_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Option<i32>, SqlxError> {
        let query = sqlx::query!(
            r#"
            SELECT beatmapset_id FROM beatmap WHERE id = $1
            "#,
            beatmap_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(query.map(|r| r.beatmapset_id.unwrap_or(0)))
    }
}
