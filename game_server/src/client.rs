use std::sync::Arc;

use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use engine::{game::{background::{PlayerBackground, PlayerBackgroundNames}, pile::CardInPile, player::Player, view::{BoardView, CardChange, CardView, ChipView, PileView}, visibility::Visibility}, storage::{Library, board::ChipOnBoard, card::Card, chip::Chip}};
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde_json::{from_str, to_string_pretty};
use tokio::sync::Mutex;

use crate::{Server, action::Action, messages::PlayerMessage};

#[derive(Clone)]
pub struct Client {
    pub ws_writer: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    pub ws_reader: Arc<Mutex<SplitStream<WebSocket>>>,
    pub player: Player,
}

impl Client {
    pub fn new(socket: WebSocket, player: Player) -> Self {
        let (writer, reader) = socket.split();
        Self {
            ws_reader: Arc::new(Mutex::new(reader)),
            ws_writer: Arc::new(Mutex::new(writer)),
            player: player,
        }
    }

    pub async fn notify_about_action(&self, action: Arc<Action>) {
        let writer = self.ws_writer.clone();
        let json = to_string_pretty(&*action.clone()).unwrap();
        writer.lock().await.send(Message::Text(Utf8Bytes::from(json))).await.unwrap();
    }

    pub async fn recieve_player_background(self: Arc<Self>, library: &Library) -> PlayerBackground {
        self.notify_about_action(Arc::new(Action::BackgroundRequest)).await;
        let mut reader = self.ws_reader.lock().await;
        let player_background_names: PlayerBackgroundNames = serde_json::from_str(
            reader.next().await.unwrap().unwrap().into_text().unwrap().as_str()
        ).unwrap();

        PlayerBackground::load_from_library(&library, player_background_names, self.player.clone()).await
    }

    pub async fn handle_message(&self, message: PlayerMessage, server: Arc<Server>) {
        match message {
            PlayerMessage::MoveCard { source, destination } => {
                let Some(source_card_in_pile) = CardInPile::from_pointer(&server.game, &source).await else {
                    return;
                };
                if let Some(card) = source_card_in_pile.card().await {
                    let card_owner = card.owner.lock().await;
                    if !server.game.rules.rights_to_touch_ones_pile(&self.player, &source_card_in_pile.pile().config.owner, &card_owner) {
                        return;
                    }
                } else {
                    return;
                }
                log::trace!("Player {} moves card: {} --[{}]-> {}",
                    self.player.0,
                    serde_json::to_string_pretty(&source).unwrap(),
                    source_card_in_pile.card().await.unwrap().name.lock().await,
                    serde_json::to_string_pretty(&destination).unwrap()
                );
                let Some(destination_space_in_pile) = CardInPile::from_pointer(&server.game, &destination).await else {
                    return;
                };
                if let Some(_) = source_card_in_pile.move_to(destination_space_in_pile).await {
                    server.notify_clients_about_move(&source, &destination).await;
                }
            },
            PlayerMessage::ChangeChip { target, changes } => {
                let chip_on_board = ChipOnBoard::from_pointer(&server.game, &target).await;

                if let Some(chip) = chip_on_board.chip().await {
                    log::trace!("Player {} changes chip: {:?} to {:?}", self.player.0, target, changes);
                    changes.apply_to(&chip).await;
                    server.notify_clients_about_chip_change(&target, chip).await;
                }
                
            },
            PlayerMessage::ViewChip(pointer) => {
                let chip_on_board = ChipOnBoard::from_pointer(&server.game, &pointer).await;

                if let Some(chip) = chip_on_board.chip().await {
                    log::trace!("Player {} views chip: {:?}", self.player.0, pointer);
                    self.notify_about_action(Arc::new(
                        Action::ViewChip {
                            target: pointer,
                            chip: ChipView::from_chip(chip, &self.player).await, 
                        }
                    )).await;
                }
            },
            PlayerMessage::CreateChip { destination, coordinates, name } => {
                log::trace!("Player {} creates chip: [{}]-> {:?} on ({:?})", self.player.0, name, destination, coordinates);
                let Ok(raw_chip) = server.game.card_library.get_raw_chip_by_name(&name).await else {
                    log::error!("Can't load chip {}", name);
                    return;
                };
                let chip = Arc::new(Chip::new_from_raw(raw_chip, self.player.clone(), Visibility::Public));
                let destination_space_on_board = ChipOnBoard::from_pointer(&server.game, &destination).await;
                destination_space_on_board.insert(chip.clone()).await;
                server.notify_clients_about_chip_create(chip, &destination).await;
            },
            PlayerMessage::ViewBoard(pointer) => {
                let board = server.game.board(&pointer).await.clone().unwrap();

                log::trace!("Player {} views board: {:?}", self.player.0, pointer);
                self.notify_about_action(Arc::new(
                    Action::ViewBoard {
                        target: pointer,
                        board: BoardView::from_board(board, &self.player).await, 
                    }
                )).await;
            },
            PlayerMessage::CreateCard { name, destination } => {
                log::trace!("Player {} creates card: [{}]-> {}", self.player.0, name, serde_json::to_string_pretty(&destination).unwrap());
                let Ok(raw_card) = server.game.card_library.get_raw_card_by_name(&name).await else {
                    log::error!("Can't load card {}", name);
                    return;
                };
                let card = Arc::new(Card::from_raw(&raw_card, self.player.clone(), Visibility::Public));
                let Some(destination_space_in_pile) = CardInPile::from_pointer(&server.game, &destination).await else {
                    return;
                };
                destination_space_in_pile.insert(card.clone()).await;
                server.notify_clients_about_create(card, &destination).await;
            },

            PlayerMessage::ChangeCardToRaw { target } => {
                let Some(target_card_in_pile) = CardInPile::from_pointer(&server.game, &target).await else {
                    return;
                };
                let Some(target_card) = target_card_in_pile.card().await else {
                    return;
                };
                let card_owner = target_card.owner.lock().await.clone();
                if !server.game.rules.rights_to_touch_ones_pile(&self.player, &target_card_in_pile.pile().config.owner, &card_owner) {
                    return;
                }
                log::trace!("Player {} changes to raw card: ->[{}]<- at {}",
                    self.player.0,
                    target_card.name.lock().await,
                    serde_json::to_string_pretty(&target).unwrap()
                );
                let changes = CardChange::from_raw_card(&target_card.raw);
                changes.apply_to(target_card.clone()).await;
                server.notify_clients_about_change(&target, target_card).await;
            },

            PlayerMessage::ChangeCard { target, changes } => {

                let Some(target_card_in_pile) = CardInPile::from_pointer(&server.game, &target).await else {
                    return;
                };
                let Some(target_card) = target_card_in_pile.card().await else {
                    return;
                };
                let card_owner = target_card.owner.lock().await.clone();
                if !server.game.rules.rights_to_touch_ones_pile(&self.player, &target_card_in_pile.pile().config.owner, &card_owner) {
                    return;
                }
                log::trace!("Player {} changes card: @>[{}]<@ at {}",
                    self.player.0,
                    target_card.name.lock().await,
                    serde_json::to_string_pretty(&target).unwrap()
                );
                changes.apply_to(target_card.clone()).await;
                server.notify_clients_about_change(&target, target_card).await;
            },

            PlayerMessage::ViewPile ( target ) => {
                log::trace!("Player {} views pile: # {} #", self.player.0, serde_json::to_string_pretty(&target).unwrap());
                if let Some(pile) = server.game.pile(&target).await {
                    let pile_view = PileView::from_pile(pile, &self.player).await;
                    self.notify_about_action(Arc::new(Action::ViewPile {target: target, pile: pile_view})).await;
                }
            },

            PlayerMessage::ViewCard ( target ) => {
                log::trace!("Player {} views card: [{}]", self.player.0, serde_json::to_string_pretty(&target).unwrap());
                if let Some(card) = server.game.card(&target).await {
                    let card_view = CardView::from_card(card, &self.player).await;
                    self.notify_about_action(Arc::new(Action::ViewCard {target: target, card: card_view})).await;
                }
            },

            PlayerMessage::TurnEnd => {
                log::trace!("Player {} ends turn", self.player.0);
                let active_player = server.game.active_player.lock().await;
                if *active_player == self.player {
                    drop(active_player);
                    server.game.next_active_player().await;
                    server.notify_clients_about_next_turn().await;
                }
            },

            PlayerMessage::GameInfo => {
                log::trace!("Player {} requests game info", self.player.0);
                self.notify_about_action(Arc::new(Action::GameInfo {
                    your_number: self.player.0,
                    players_count: server.game.players_count,
                    battlefields_count: server.game.battlefields.lock().await.len() 
                })).await;
            },

            PlayerMessage::Surrender => {
                log::trace!("Player {} surrends", self.player.0);
            },
        };
    }

    async fn client_listener(self: Arc<Self>, server: Arc<Server>) {
        loop {
            let mut reader = self.ws_reader.lock().await;
            let message: PlayerMessage = from_str(reader.next().await.unwrap().unwrap().to_text().unwrap()).unwrap();
            self.handle_message(message, server.clone()).await
        }
    }

    pub async fn start_player(self: Arc<Self>, server: Arc<Server>) {
        log::info!("Starting handling player {}", self.player.0);
        self.notify_about_action(Arc::new(Action::GameInfo {
            your_number: self.player.0,
            players_count: server.game.players_count,
            battlefields_count: server.game.battlefields.lock().await.len() ,
        })).await;
        self.notify_about_action(Arc::new(Action::NextTurn (
            server.game.active_player.lock().await.clone(),
        ))).await;
        self.clone().client_listener(server).await;
    }
}
