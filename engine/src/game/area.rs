use std::sync::Arc;
use crate::{game::{background::PlayerBackground, deck::Deck, hand::Hand, pile::Pile, player::Player, visibility::Visibility}, storage::rules::Rules};

pub struct PlayerArea {
    pub hand: Arc<Hand>,
    pub main_deck: Arc<Deck>,
    pub trash_deck: Arc<Deck>,
    pub mana_deck: Arc<Deck>,
    pub mana_pool: Arc<Deck>,
    pub heroes: Arc<Pile>,
    pub special_zone: Arc<Pile>,
    pub base: Arc<Pile>,
}

impl PlayerArea {
    pub fn new(background: PlayerBackground, rules: &Rules) -> Self {
        Self {
            heroes: Arc::new(Pile::new_with_cards(background.heroes_clone(), true, Visibility::Public, false, background.player.clone())),
            hand: Arc::new(Hand::new_empty(background.player.clone())),
            main_deck: Arc::new(Deck::from_cards(
                background.main_deck_clone(),
                rules.is_main_deck_shuffled,
                background.player.clone(),
                Visibility::Secret,
                rules.only_raw_card_in_main_deck
            )),
            mana_deck: Arc::new(Deck::from_cards(
                background.mana_deck_clone(),
                rules.is_mana_deck_shuffled,
                background.player.clone(),
                Visibility::Secret,
                rules.only_raw_card_in_mana_deck,
            )),
            mana_pool: Arc::new(Deck::new_empty(
                background.player.clone(),
                Visibility::Public,
                rules.only_raw_card_in_mana_pool,
            )),
            trash_deck: Arc::new(Deck::new_empty(
                background.player.clone(),
                Visibility::Public,
                rules.only_raw_card_in_trash_deck,
            )),
            base: Arc::new(Pile::new_with_cards(
                background.base_clone(),
                false,
                Visibility::Public,
                false,
                background.player.clone()
            )),
            special_zone: Arc::new(Pile::new_with_cards(
                background.special_zone_clone(),
                rules.is_special_zone_shuffled,
                Visibility::Public,
                rules.only_raw_card_in_special_zone,
                background.player
            )),
        }
    }
}

pub struct FightArea {
    pub sides: Vec<Arc<Pile>>,
}

impl FightArea {
    pub fn new(location_cards: Arc<Pile>, players: Vec<Player>) -> Self {
        let mut sides = vec![location_cards];
        sides.append(&mut players.into_iter().map(|pl| Arc::new(Pile::new_empty(Visibility::Public, false, pl))).collect::<Vec<_>>());
        Self {
            sides,
        }
    }
}
