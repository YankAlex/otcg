use std::{collections::HashMap};

use futures_util::future::join_all;
use serde::{Deserialize, Serialize};

use crate::{game::{player::Player}, storage::{Library, card::{Card, RawCard}}};

pub struct PlayerBackground {
    pub piles: HashMap<Box<str>, Vec<RawCard>>,
    pub player: Player,
}

impl PlayerBackground {
    pub fn cards_pile(&self, name: &str) -> Vec<RawCard> {
        self.piles.get(name).unwrap_or(&vec![]).clone()
    }
    pub async fn load_from_library(library: &Library, names: PlayerBackgroundNames, player: Player) -> Self {
        Self {
            piles: join_all(
                names.piles.into_iter().map(
                    async |(pile_name, cards_names)| (
                        pile_name,
                        join_all(
                            cards_names.iter().map(
                                async |name| library.get_raw_card_by_name(&name).await.unwrap()
                            )
                        ).await
                    )
                )
            ).await.into_iter().collect(),
            player: player,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerBackgroundNames {
    pub piles: HashMap<Box<str>, Vec<Box<str>>>,
}

