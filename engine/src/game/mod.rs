use std::sync::Arc;
use tokio::sync::Mutex;
use area::{PlayerArea, FightArea};

use crate::{game::{background::PlayerBackground, pile::{CardInPile, Pile}, player::Player, pointer::{CardPointer, PilePointer, PileType}, visibility::Visibility}, storage::{Library, card::Card, rules::Rules}};

pub mod area;
pub mod player;
pub mod pile;
pub mod hand;
pub mod deck;
pub mod visibility;
pub mod background;
pub mod pointer;
pub mod view;

pub struct Game {
    player_areas: Vec<Arc<PlayerArea>>,
    pub card_library: Arc<Library>,
    pub fight_areas: Mutex<Vec<Arc<FightArea>>>,
    pub spell_queue: Arc<Pile>,
    pub rules: Arc<Rules>,
    pub players_count: usize,
    pub active_player: Mutex<Player>,
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
            fight_areas: {
                let mut fight_areas = vec![];
                for _ in 0..rules.fight_areas_count {
                    fight_areas.push(Arc::new(FightArea::new(Arc::new(Pile::new_empty(Visibility::Public, true, player::ADMIN)), players.clone())));
                }
                Mutex::new(fight_areas)
            },
            rules: Arc::new(rules),
            spell_queue: Arc::new(Pile::new_empty(Visibility::Public, false, player::ADMIN)),
            card_library: Arc::new(card_library),
        }
    }

    pub async fn get_pile(&self, pile: &PilePointer) -> Arc<Pile> {
        match pile.r#type {
            PileType::Heroes => self.get_player_area(&Player(pile.player)).unwrap().heroes.clone(),
            PileType::Hand => self.get_player_area(&Player(pile.player)).unwrap().hand.pile.clone(),
            PileType::MainDeck => self.get_player_area(&Player(pile.player)).unwrap().main_deck.pile.clone(),
            PileType::ManaDeck => self.get_player_area(&Player(pile.player)).unwrap().mana_deck.pile.clone(),
            PileType::ManaPool => self.get_player_area(&Player(pile.player)).unwrap().mana_pool.pile.clone(),
            PileType::TrashDeck => self.get_player_area(&Player(pile.player)).unwrap().trash_deck.pile.clone(),
            PileType::Base => self.get_player_area(&Player(pile.player)).unwrap().base.clone(),
            PileType::SpecialZone => self.get_player_area(&Player(pile.player)).unwrap().special_zone.clone(),
            PileType::SpellQueue => self.spell_queue.clone(),
            PileType::FightArea(index) => self.fight_areas.lock().await[index as usize].sides[pile.player as usize].clone(),
        }
    }
    pub async fn get_card(&self, card: &CardPointer) -> Option<Arc<Card>> {
        CardInPile::from_pointer(&self, card).await.card().await
    }
    pub async fn next_active_player(&self) {
        let mut active_player = self.active_player.lock().await;
        active_player.0 = (active_player.0 % self.players_count as i32) + 1;
    }
}

