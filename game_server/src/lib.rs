use std::{path::PathBuf, sync::Arc};
use rand::seq::SliceRandom;
use tokio::sync::Mutex;
use engine::{game::{Game, player::{self, Player}, pointer::CardPointer}, storage::{Library, card::Card, rules::Rules}};
use axum::extract::ws::WebSocket;
use futures_util::future::join_all;
use engine::game::view;

use action::Action;
use view::CardView;
use client::Client;

mod action;
mod messages;
mod client;

pub struct Config {
    pub game_name: Box<str>,
    pub library_path: PathBuf,
}

pub struct Server {
    game: Arc<Game>,
    clients: Mutex<Vec<Arc<Client>>>,
}

impl Server {
    async fn create_game(players: Vec<Arc<Client>>, rules: Rules, library: Library) -> Game {
        let players_background = join_all(
            players.into_iter().map(
                async |pl| pl.recieve_player_background(&library).await
            ).collect::<Vec<_>>()
        ).await;
        Game::new(players_background, rules, library)
    }

    pub async fn from_clients_ws(mut players_ws: Vec<WebSocket>, watchers_ws: Vec<WebSocket>, config: Config) -> Self {
        players_ws.shuffle(&mut rand::rng());

        let players: Vec<Arc<Client>> = players_ws.into_iter().enumerate().map(|(i, ws)| Arc::new(Client::new(ws, Player(i as i32 + 1)))).collect();
        let watchers: Vec<Arc<Client>> = watchers_ws.into_iter().map(|ws| Arc::new(Client::new(ws, player::WATCHER))).collect();
        let mut clients = vec![];
        clients.append(&mut players.clone());
        clients.append(&mut watchers.clone());
        let library = Library::new(&config.library_path);
        let rules = Rules::new(&config.game_name);
        Self {
            game: Arc::new(Self::create_game(players.clone(), rules, library).await),
            clients: Mutex::new(clients),
        }
    }
    
    pub async fn add_watcher(self: Arc<Self>, watcher: WebSocket) {
        let client = Arc::new(Client::new(watcher, player::WATCHER));
        tokio::spawn(client.clone().start_player(self.clone()));
        self.clients.lock().await.push(client);
    }

    pub async fn notify_clients_about_next_turn(&self) {
        let action = Arc::new(Action::NextTurn (self.game.active_player.lock().await.clone()));
        let clients = self.clients.lock().await.clone();
        join_all(clients.iter().map(async |pl| pl.notify_about_action(action.clone()).await).collect::<Vec<_>>()).await;
    }

    pub async fn notify_clients_about_create(&self, card: Arc<Card>, destination: &CardPointer) {
        let clients = self.clients.lock().await.clone();
        join_all(clients.iter().map(async |pl| {
            let action = Arc::new(Action::CardCreated {
                card: CardView::from_card(card.clone(), &pl.player).await,
                destination: destination.clone(),
            });
            pl.notify_about_action(action).await;
        }).collect::<Vec<_>>()).await;
    }

    pub async fn notify_clients_about_move(&self, source: &CardPointer, destination: &CardPointer) {
        let action = Arc::new(Action::CardMoved {
            source: source.clone(),
            destination: destination.clone(),
        });
        log::warn!("notifying about move");
        let clients = self.clients.lock().await.clone();
        join_all(clients.iter().map(async |pl| pl.notify_about_action(action.clone()).await).collect::<Vec<_>>()).await;
    }
    
    pub async fn notify_clients_about_change(&self, target: &CardPointer, new_card: Arc<Card>) {
        let clients = self.clients.lock().await.clone();
        join_all(clients.iter().map(async |pl| {
            let action = Arc::new(Action::CardChanged {
                target: target.clone(),
                new_card: CardView::from_card(new_card.clone(), &pl.player).await,
            });
            pl.notify_about_action(action).await;
        }).collect::<Vec<_>>()).await;
    }

    pub async fn start(self: Arc<Self>) {
        let clients = self.clients.lock().await.clone();
        join_all(clients.iter().cloned().map(async |pl| {
            tokio::spawn(pl.start_player(self.clone())).await
        }).collect::<Vec<_>>()).await;
    }
}
