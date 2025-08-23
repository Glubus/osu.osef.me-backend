use axum::{
    extract::{State, Query},
    http::StatusCode,
    Json,
};
use crate::db::DatabaseManager;
use crate::models::skins::Skin;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateSkinRequest {
    pub name: String,
    pub author: String,
    pub version: String,
    pub download_url: String,
    pub note_type: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct CreateSkinResponse {
    pub id: i32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SearchSkinsQuery {
    pub note_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Option<String>,
}

#[derive(Serialize)]
pub struct SearchSkinsResponse {
    pub skins: Vec<Skin>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// Handler pour créer un nouveau skin
pub async fn create_skin_handler(
    State(db): State<DatabaseManager>,
    Json(payload): Json<CreateSkinRequest>,
) -> Result<Json<CreateSkinResponse>, StatusCode> {
    let pool = db.get_pool();
    
    // Créer le nouveau skin
    let skin = Skin::new(
        payload.name,
        payload.author,
        payload.version,
        payload.download_url,
        payload.note_type,
        payload.tags,
    );

    // Insérer en base de données
    let id = skin.insert_into_db(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateSkinResponse {
        id,
        message: "Skin créé avec succès".to_string(),
    }))
}

/// Handler pour rechercher les skins avec filtres multiples
pub async fn search_skins(
    State(db): State<DatabaseManager>,
    Query(query): Query<SearchSkinsQuery>,
) -> Result<Json<SearchSkinsResponse>, StatusCode> {
    let pool = db.get_pool();
    
    // Valeurs par défaut pour la pagination
    let limit = query.limit.unwrap_or(30);
    let offset = query.offset.unwrap_or(0);
    
    // Rechercher les skins avec tous les filtres
    let skins = Skin::search_by_filters(
        pool,
        query.note_type.as_deref(),
        query.tags.as_deref(),
        query.search.as_deref(),
        limit,
        offset,
        query.order_by.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Compter le total avec les mêmes filtres
    let total = Skin::count_by_filters(
        pool,
        query.note_type.as_deref(),
        query.tags.as_deref(),
        query.search.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SearchSkinsResponse {
        skins,
        total,
        limit,
        offset,
    }))
}



