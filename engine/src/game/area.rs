use std::{collections::HashMap, sync::Arc};
use crate::{game::{background::PlayerBackground, pile::{Pile, PileConfig}, player::Player, visibility::Visibility}, storage::rules::Rules};

pub struct PlayerArea {
    pub piles: HashMap<Box<str>, Arc<Pile>>,
}

impl PlayerArea {
    pub fn new(background: PlayerBackground, rules: &Rules) -> Self {
        Self {
            piles: rules.piles(background.player.clone()).into_iter().map(
                |(name, config)| (
                    name.clone(),
                    Arc::new(Pile::from_raw_cards(config, background.cards_pile(&name))),
                 )
            ).collect(),
        }
    }
}

pub struct Battlefield {
    pub sides: Vec<Arc<Pile>>,
}

impl Battlefield {
    pub fn new(location_cards: Arc<Pile>, players: Vec<Player>) -> Self {
        let mut sides = vec![location_cards];
        sides.append(&mut players.into_iter().map(|pl| Arc::new(Pile::new_empty(PileConfig {
            shuffled: false,
            only_raw_cards: false,
            owner: pl,
            default_visibility: Visibility::Public,
        }))).collect::<Vec<_>>());
        Self {
            sides,
        }
    }
}
