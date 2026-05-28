use std::{collections::VecDeque, sync::Arc};

use axum::{extract::{State, WebSocketUpgrade}, response::Response};
use futures_util::future::join_all;
use tokio::sync::Mutex;

use crate::{DURATION_BETWEEN_GAME_GATHERINGS, client::Client, game_parameters::GameParameters};

pub struct QueueState {
    clients: Mutex<VecDeque<Client>>,
    game_parameters: GameParameters,
}

impl QueueState {
    pub fn new(game_parameters: GameParameters) -> Self {
        Self {
            clients: Mutex::new(VecDeque::new()),
            game_parameters,
        }
    }
    pub async fn push(&self, client: Client) {
        self.clients.lock().await.push_front(client);
        log::trace!("Pushed new clinet in queue");
    }
}


pub async fn queue_websocket_upgrader(ws: WebSocketUpgrade, State(state): State<Arc<QueueState>>) -> Response {
    log::trace!("Pushed");
    ws.on_upgrade(|web_socket| async move {
        state.push(Client::new(web_socket)).await;
    })
}


pub async fn trying_gather_players(queue: Arc<QueueState>) {
    loop {
        tokio::time::sleep(DURATION_BETWEEN_GAME_GATHERINGS).await;
        loop {
            let players_count = queue.game_parameters.players_count;
            let mut game_queue = queue.clients.lock().await;
            if game_queue.len() < players_count {
                break;
            }
            let splited_off_elements_count = game_queue.len() - players_count;
            let last_clients = game_queue.split_off(splited_off_elements_count);
            let are_ready = join_all(last_clients.into_iter().map(async |mut client| (client.is_ready_to_start().await, client)).collect::<Vec<_>>()).await;
            let only_ready = are_ready.into_iter().filter_map(|(is_ready, client)| {
                if is_ready {
                    Some(client)
                } else {
                    None
                }
            }).collect::<Vec<_>>();
            if only_ready.len() == players_count {
                let websockets: Vec<_> = only_ready.into_iter().map(|client| client.get_websocket()).collect();
                let game_parameters = queue.game_parameters.clone();
                tokio::spawn(async move {
                    Arc::new(game_server::Server::from_clients_ws(websockets, vec![], game_parameters.clone().into()).await).start().await;
                });
            } else {
                game_queue.append(&mut only_ready.into())
            }
            drop(game_queue);
        }
    }
}
