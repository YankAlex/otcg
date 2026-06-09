use serde::{Deserialize, Serialize};

use crate::game::Game;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PileType {
    Name(Box<str>),
    Battlefield(i32),
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
        let len = game.pile(&self.pile).await.size().await as i32;
        self.index = self.index % (len + 1);
        if self.index < 0 {
            self.index += len + 1;
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum BoardPointer {
    Name(Box<str>),
}

#[derive(Deserialize, Serialize)]
pub struct ChipPointer {
    pub r#board: BoardPointer,
    pub index: i32,
}

impl ChipPointer {
    pub async fn normalize(&mut self, game: &Game) {
        let len = game.board(&self.board).await.chips_size().await as i32;
        self.index = self.index % (len + 1);
        if self.index < 0 {
            self.index += len + 1;
        }
    }
}
