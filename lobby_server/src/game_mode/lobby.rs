use std::{collections::{HashMap, VecDeque}, sync::Arc};

use axum::{extract::{Path, State, WebSocketUpgrade, ws::WebSocket}, response::Response};
use game_server::player;
use tokio::sync::Mutex;

use crate::{client::Client, game_parameters::GameParameters};

pub enum Lobby {
    Waiting {
        players: Mutex<VecDeque<Client>>,
        watchers: Mutex<VecDeque<Client>>,
        game_parameters: GameParameters,
    },
    Started {
        game: Arc<game_server::Server>,
    }
}

impl Lobby {
    pub fn new(game_parameters: GameParameters) -> Self {
        Self::Waiting {
            players: Mutex::new(VecDeque::new()),
            watchers: Mutex::new(VecDeque::new()),
            game_parameters,
        }
    }

    pub async fn try_start(&mut self) {
        log::trace!("Trying to start lobby");
        if let Self::Waiting { players, watchers, game_parameters } = self && players.lock().await.len() == game_parameters.players_count {
            let players: Vec<_> = players.lock().await.drain(..).map(|client| client.get_websocket()).collect();
            let watchers: Vec<_> = watchers.lock().await.drain(..).map(|client| client.get_websocket()).collect();
            let game = Arc::new(game_server::Server::from_clients_ws(players, watchers, game_parameters.clone().into()).await);
            tokio::spawn(game.clone().start());
            log::info!("Succesfuly started lobby");
            *self = Self::Started {
                game,
            };
        } else {
            log::warn!("Tried to start lobby, but it already started or players count is'nt sufficient.");
        }
    }
}

pub struct LobbiesState {
    lobbies: Mutex<HashMap<i32, Lobby>>,
    game_parameters: GameParameters,
}

impl LobbiesState {
    pub fn new(game_parameters: GameParameters) -> Self {
        Self {
            lobbies: Mutex::new(HashMap::new()),
            game_parameters,
        }
    }

    pub async fn add_watcher_to_lobby(&self, id: &i32, ws: WebSocket) {
        log::trace!("Trying to add watcher to the lobby.");
        let mut lobbies = self.lobbies.lock().await;
        if let None = lobbies.get(id) {
            lobbies.insert(*id, Lobby::new(self.game_parameters.clone()));
        }
        if let Some(lobby) = lobbies.get(id) {
            match &lobby {
                Lobby::Started { game } => {
                    game.clone().add_client(ws, game_server::player::WATCHER).await;
                    log::info!("Succesfuly added watcher to already started lobby.");
                },
                Lobby::Waiting { watchers, .. } => {
                    watchers.lock().await.push_back(Client::new(ws));
                    log::info!("Added new watcher to lobby.");
                }
            }
        };
    }

    pub async fn add_player_to_lobby(&self, id: &i32, mut ws: WebSocket) {
        log::trace!("Trying to add player to the lobby.");
        let mut lobbies = self.lobbies.lock().await;
        if let None = lobbies.get(id) {
            lobbies.insert(*id, Lobby::new(self.game_parameters.clone()));
        }
        if let Some(lobby) = lobbies.get_mut(id) {
            match &lobby {
                Lobby::Started { game } => {
                    let game = game.clone();
                    tokio::spawn(async move {
                        let _ = ws.send("\"choose_player\"".into()).await;
                        let Some(Ok(player)) = ws.recv().await else {
                            log::warn!("Client closed connection before choosed player or can't read message from ws.");
                            return;
                        };
                        let Ok(text) = player.to_text() else {
                            log::warn!("Client send wrong format message when choose player.");
                            return;
                        };
                        let Ok(number) = text.parse::<i32>() else {
                            log::warn!("Client send not i32 message when choose player.");
                            return;
                        };

                        game.clone().add_client(ws, player::Player(number)).await;
                        log::warn!("Succesfuly added player to already started lobby.");
                    });
                },
                Lobby::Waiting { players, .. } => {
                    players.lock().await.push_back(Client::new(ws));
                    log::info!("Succesfuly added new player to lobby.");
                    lobby.try_start().await;
                }
            }
        };
    }
}

pub async fn lobbies_player_websocket_upgrader(Path(lobby_id): Path<i32>, ws: WebSocketUpgrade, State(state): State<Arc<LobbiesState>>) -> Response {
    ws.on_upgrade(move |web_socket| async move {
        state.add_player_to_lobby(&lobby_id, web_socket).await;
    })
}

pub async fn lobbies_watcher_websocket_upgrader(Path(lobby_id): Path<i32>, ws: WebSocketUpgrade, State(state): State<Arc<LobbiesState>>) -> Response {
    ws.on_upgrade(move |web_socket| async move {
        state.add_watcher_to_lobby(&lobby_id, web_socket).await;
    })
}
