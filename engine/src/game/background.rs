use std::sync::Arc;

use futures_util::future::join_all;
use serde::{Deserialize, Serialize};

use crate::{game::{player::Player, visibility::Visibility}, storage::{Library, card::{Card, RawCard}}};

pub struct PlayerBackground {
    pub main_deck: Vec<RawCard>,
    pub mana_deck: Vec<RawCard>,
    pub special_zone: Vec<RawCard>,
    pub heroes: Vec<RawCard>,
    pub base: Vec<RawCard>,
    pub player: Player,
}

impl PlayerBackground {
    pub fn main_deck_clone(&self) -> Vec<Arc<Card>> {
        self.main_deck.iter().map(|raw_card| Arc::new(Card::from_raw(raw_card, self.player.clone(), Visibility::Secret))).collect()
    }
    pub fn mana_deck_clone(&self) -> Vec<Arc<Card>> {
        self.mana_deck.iter().map(|raw_card| Arc::new(Card::from_raw(raw_card, self.player.clone(), Visibility::Secret))).collect()
    }
    pub fn base_clone(&self) -> Vec<Arc<Card>> {
        self.base.iter().map(|raw_card| Arc::new(Card::from_raw(raw_card, self.player.clone(), Visibility::Public))).collect()
    }
    pub fn special_zone_clone(&self) -> Vec<Arc<Card>> {
        self.special_zone.iter().map(|raw_card| Arc::new(Card::from_raw(raw_card, self.player.clone(), Visibility::Public))).collect()
    }
    pub fn heroes_clone(&self) -> Vec<Arc<Card>> {
        self.heroes.iter().map(|raw_card| Arc::new(Card::from_raw(raw_card, self.player.clone(), Visibility::Public))).collect()
    }
    pub async fn load_from_library(library: &Library, names: PlayerBackgroundNames, player: Player) -> Self {
        Self {
            main_deck: join_all(
                           names.main_deck.iter().map(
                               async |name| library.get_raw_card_by_name(&name).await.unwrap()
                           ).collect::<Vec<_>>()
                       ).await,
            mana_deck: join_all(
                           names.mana_deck.iter().map(
                               async |name| library.get_raw_card_by_name(&name).await.unwrap()
                           ).collect::<Vec<_>>()
                       ).await,
            special_zone: join_all(
                           names.special_zone.iter().map(
                               async |name| library.get_raw_card_by_name(&name).await.unwrap()
                           ).collect::<Vec<_>>()
                       ).await,
            heroes: join_all(
                           names.heroes.iter().map(
                               async |name| library.get_raw_card_by_name(&name).await.unwrap()
                           ).collect::<Vec<_>>()
                       ).await,
            base: join_all(
                           names.base.iter().map(
                               async |name| library.get_raw_card_by_name(&name).await.unwrap()
                           ).collect::<Vec<_>>()
                       ).await,
            player: player,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerBackgroundNames {
    pub main_deck: Vec<String>,
    pub mana_deck: Vec<String>,
    pub special_zone: Vec<String>,
    pub base: Vec<String>,
    heroes: Vec<String>,
}

