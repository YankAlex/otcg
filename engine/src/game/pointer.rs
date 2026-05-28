use serde::{Deserialize, Serialize};

use crate::game::Game;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PileType {
    Hand,
    MainDeck,
    ManaDeck,
    ManaPool,
    TrashDeck,
    FightArea (i32),
    SpecialZone,
    Heroes,
    SpellQueue,
    Base,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PilePointer {
    pub player: i32,
    pub r#type: PileType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CardPointer {
    pub pile: PilePointer,
    pub index: i32,
}

impl CardPointer {
    pub async fn normalize(&mut self, game: &Game) {
        let len = game.get_pile(&self.pile).await.size().await as i32;
        self.index = self.index % (len + 1);
        if self.index < 0 {
            self.index += len + 1;
        }
    }
}
