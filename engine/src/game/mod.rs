use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use area::PlayerArea;

use crate::{game::{area::Battlefield, background::PlayerBackground, pile::{CardInPile, Pile}, player::Player, pointer::{BoardPointer, CardPointer, PilePointer, PileType}, visibility::Visibility}, storage::{Library, board::Board, card::Card, rules::Rules}};

pub mod area;
pub mod player;
pub mod pile;
// pub mod hand;
// pub mod deck;
pub mod visibility;
pub mod coordinates;
pub mod viewable;
pub mod background;
pub mod pointer;
pub mod view;

pub struct Game {
    player_areas: Vec<Arc<PlayerArea>>,
    pub players_count: usize,
    pub rules: Arc<Rules>,
    pub battlefields: Mutex<Vec<Arc<Battlefield>>>,
    pub active_player: Mutex<Player>,
    pub card_library: Arc<Library>,
    pub piles: HashMap<Box<str>, Arc<Pile>>,
    pub boards: HashMap<Box<str>, Arc<Board>>,
}

impl Game {
    pub fn get_player_area(&self, player: &Player) -> Option<Arc<PlayerArea>> {
        match &player {
            Player(i) if *i > 0 => Some(self.player_areas[*i as usize - 1].clone()),
            Player(_) => None,
        }
    }

    pub fn new(players_backgrund: Vec<PlayerBackground>, rules: Rules, card_library: Library) -> Self {
        let players: Vec<_> = players_backgrund.iter().map(|pl_bkg| pl_bkg.player.clone()).collect();
        Self {
            active_player: Mutex::new(players_backgrund[0].player.clone()),
            players_count: players_backgrund.len(),
            player_areas: players_backgrund.into_iter().map(|background| Arc::new(PlayerArea::new(background, &rules))).collect(),
            battlefields: {
                let mut battlefields = vec![];
                for _ in 0..rules.battlefields_count {
                    battlefields.push(Arc::new(Battlefield::new(Arc::new(Pile::new_empty(pile::PileConfig {
                        default_visibility: Visibility::Public,
                        owner: player::ADMIN,
                        only_raw_cards: false,
                        shuffled: false,
                    })), players.clone())));
                }
                Mutex::new(battlefields)
            },
            piles: rules.piles(player::ADMIN).into_iter().map(
                |(name, config)| (
                    name.clone(),
                    Arc::new(Pile::new_empty(config)),
                 )
            ).collect(),
            boards: rules.boards().into_iter().map(
                |(name, config)| (
                    name.clone(),
                    Arc::new(Board::new_empty(config)),
                 )
            ).collect(),
            rules: Arc::new(rules),
            card_library: Arc::new(card_library),
        }
    }

    pub async fn pile(&self, pile: &PilePointer) -> Arc<Pile> {
        match &pile.r#type {
            PileType::Name(name) if pile.player == 0 => self.piles.get(name).unwrap().clone(),
            PileType::Name(name) => self.get_player_area(&Player(pile.player)).unwrap().piles.get(name).unwrap().clone(),
            PileType::Battlefield(index) => self.battlefields.lock().await[*index as usize].sides[pile.player as usize].clone(),
        }
    }
    pub async fn board(&self, board: &BoardPointer) -> Arc<Board> {
        match board {
            BoardPointer::Name(name) => self.boards.get(name).unwrap().clone()
        }
    }
    pub async fn card(&self, card: &CardPointer) -> Option<Arc<Card>> {
        CardInPile::from_pointer(&self, card).await.card().await
    }
    pub async fn next_active_player(&self) {
        let mut active_player = self.active_player.lock().await;
        active_player.0 = (active_player.0 % self.players_count as i32) + 1;
    }
}

