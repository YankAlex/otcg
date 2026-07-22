use axum::{Json, extract::State};

use crate::game_parameters::GameParameters;

pub async fn library_router(State(state): State<GameParameters>) -> Json<serde_json::Value> {
    Json::from(engine::storage::Library::new(&state.library_path).cards_json().await)
}
