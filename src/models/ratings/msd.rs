use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Error as SqlxError};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use crate::helpers::common::from_f64;
use chrono::NaiveDateTime;
use minacalc_rs::Ssr;
use crate::helpers::common::from_f32;
use crate::helpers::msd::calculate_main_pattern;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MSD {
    pub id: Option<i32>,
    pub beatmap_id: Option<i32>,
    pub overall: Option<BigDecimal>,
    pub stream: Option<BigDecimal>,
    pub jumpstream: Option<BigDecimal>,
    pub handstream: Option<BigDecimal>,
    pub stamina: Option<BigDecimal>,
    pub jackspeed: Option<BigDecimal>,
    pub chordjack: Option<BigDecimal>,
    pub technical: Option<BigDecimal>,
    pub rate: Option<BigDecimal>,
    pub main_pattern: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[async_trait]
pub trait Insert {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError>;
}

#[async_trait]
impl Insert for MSD {
    async fn insert(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO msd (
                beatmap_id, overall, stream, jumpstream, handstream,
                stamina, jackspeed, chordjack, technical, rate,
                main_pattern
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
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
            self.main_pattern.as_deref(),
        );

        let result = query.fetch_one(pool).await?;
        Ok(result.id)
    }
}
impl From<Ssr> for MSD {
    fn from(ssr: Ssr) -> Self {
        Self {
            id: None,
            beatmap_id: None,
            overall: Some(from_f32(ssr.overall)),
            stream: Some(from_f32(ssr.stream)),
            jumpstream: Some(from_f32(ssr.jumpstream)),
            handstream: Some(from_f32(ssr.handstream)),
            stamina: Some(from_f32(ssr.stamina)),
            jackspeed: Some(from_f32(ssr.jackspeed)),
            chordjack: Some(from_f32(ssr.chordjack)),
            technical: Some(from_f32(ssr.technical)),
            rate: Some(from_f32(1.0)),
            main_pattern: Some(calculate_main_pattern(&ssr)),
            created_at: None,
            updated_at: None,
        }
    }
}

impl MSD {
    /// Crée une nouvelle instance MSD avec des valeurs par défaut
    pub fn new(beatmap_id: i32) -> Self {
        Self {
            id: None,
            beatmap_id: Some(beatmap_id),
            overall: None,
            stream: None,
            jumpstream: None,
            handstream: None,
            stamina: None,
            jackspeed: None,
            chordjack: None,
            technical: None,
            rate: None,
            main_pattern: None,
            created_at: None,
            updated_at: None,
        }
    }

    /// Insère un MSD dans la base de données
    pub async fn insert_into_db(&self, pool: &PgPool) -> Result<i32, SqlxError> {
        self.insert(pool).await
    }

    /// Met à jour les valeurs de difficulté
    pub fn with_difficulty(
        mut self,
        overall: Option<f64>,
        stream: Option<f64>,
        jumpstream: Option<f64>,
        handstream: Option<f64>,
        stamina: Option<f64>,
        jackspeed: Option<f64>,
        chordjack: Option<f64>,
        technical: Option<f64>,
        rate: Option<f64>,
        main_pattern: Option<String>,
    ) -> Self {
        self.overall = overall.map(from_f64);
        self.stream = stream.map(from_f64);
        self.jumpstream = jumpstream.map(from_f64);
        self.handstream = handstream.map(from_f64);
        self.stamina = stamina.map(from_f64);
        self.jackspeed = jackspeed.map(from_f64);
        self.chordjack = chordjack.map(from_f64);
        self.technical = technical.map(from_f64);
        self.rate = rate.map(from_f64);
        self.main_pattern = main_pattern;
        self
    }

    /// Récupère un MSD par son ID
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, SqlxError> {
        let query = sqlx::query_as!(
            MSD,
            r#"
            SELECT 
                id, beatmap_id, overall, stream, jumpstream, handstream,
                stamina, jackspeed, chordjack, technical, rate,
                main_pattern, created_at, updated_at
            FROM msd 
            WHERE id = $1
            "#,
            id
        );

        query.fetch_optional(pool).await
    }

    /// Récupère un MSD par beatmap_id
    pub async fn find_by_beatmap_id(pool: &PgPool, beatmap_id: i32) -> Result<Option<Self>, SqlxError> {
        let query = sqlx::query_as!(
            MSD,
            r#"
            SELECT 
                id, beatmap_id, overall, stream, jumpstream, handstream,
                stamina, jackspeed, chordjack, technical, rate,
                main_pattern, created_at, updated_at
            FROM msd 
            WHERE beatmap_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            beatmap_id
        );

        query.fetch_optional(pool).await
    }


    pub async fn find_by_beatmap_id_and_rate(pool: &PgPool, beatmap_id: i32, rate: f64) -> Result<Option<Self>, SqlxError> {
        let query = sqlx::query_as!(
            MSD,
            r#"
            SELECT * FROM msd WHERE beatmap_id = $1 AND rate = $2
            "#,
            beatmap_id, from_f64(rate)
        );

        query.fetch_optional(pool).await
    }

    /// Récupère tous les MSD pour un beatmap
    pub async fn find_all_by_beatmap_id(pool: &PgPool, beatmap_id: i32) -> Result<Vec<Self>, SqlxError> {
        let query = sqlx::query_as!(
            MSD,
            r#"
            SELECT 
                id, beatmap_id, overall, stream, jumpstream, handstream,
                stamina, jackspeed, chordjack, technical, rate,
                main_pattern, created_at, updated_at
            FROM msd 
            WHERE beatmap_id = $1
            ORDER BY created_at DESC
            "#,
            beatmap_id
        );

        query.fetch_all(pool).await
    }
}
