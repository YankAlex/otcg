use std::{path::PathBuf, sync::Arc, time::Duration};

use axum::{Router, http::Method, routing::{any, get}};
use tower_http::cors::{CorsLayer, self};

use crate::{game_mode::{lobby::{LobbiesState, lobbies_player_websocket_upgrader, lobbies_watcher_websocket_upgrader}, queue::{QueueState, queue_websocket_upgrader, trying_gather_players}}, game_parameters::GameParameters, library::library_router};

mod client;
mod game_mode;
mod game_parameters;
mod library;

const DURATION_BETWEEN_GAME_GATHERINGS: Duration = Duration::from_millis(1000);


#[tokio::main]
async fn main() {
    env_logger::init();

    let address: String = std::env::var("ADDRESS").unwrap_or("0.0.0.0:8126".to_string());
    
    let game_parameters = GameParameters {
        name: std::env::var("GAME").unwrap_or("riftbound".to_string()).into(),
        players_count: usize::from_str_radix(&std::env::var("PLAYERS").unwrap_or("2".to_string()), 10).unwrap_or(2),
        library_path: PathBuf::from(std::env::var("LIBRARY").unwrap_or(".otcglib".to_string())),
    };

    let cors_layer = CorsLayer::new()
        .allow_origin(cors::Any)  // Open access to selected route
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let queue = Arc::new(QueueState::new(game_parameters.clone()));
    let lobbies = Arc::new(LobbiesState::new(game_parameters.clone()));
    let queue_router = Router::new()
        .route("/ws", any(queue_websocket_upgrader))
        .with_state(queue.clone());
    let lobby_router = Router::new()
        .route("/{lobby_id}/ws", any(lobbies_player_websocket_upgrader))
        .route("/{lobby_id}/watcher/ws", any(lobbies_watcher_websocket_upgrader))
        .with_state(lobbies.clone());
    let app = Router::new()
        .route("/library", get(library_router))
        .with_state(game_parameters.clone())
        .nest("/game_queue", queue_router)
        .nest("/lobby", lobby_router)
        .layer(cors_layer);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tokio::spawn(trying_gather_players(queue.clone()));
    log::info!("Succesfully starting the lobby_server.");
    axum::serve(listener, app).await.unwrap();
}
