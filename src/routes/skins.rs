use axum::{
    Router,
    routing::{post, get},
};
use crate::db::DatabaseManager;
use crate::handlers::skins::{create_skin_handler, search_skins};

pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/skins", post(create_skin_handler))
        .route("/skins/search", get(search_skins))
} 
