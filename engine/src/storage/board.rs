use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{game::{Game, pointer::ChipPointer, position}, storage::chip::Chip};

pub struct Board {
    pub raw: RawBoard,
    pub img_url: Mutex<Box<str>>,
    pub chips: Mutex<Vec<Arc<Chip>>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RawBoard {
    pub height: usize,
    pub width: usize,
    pub img_url: Box<str>,
}

impl Board {
    pub fn new_empty(raw: RawBoard) -> Self {
        Self {
            img_url: Mutex::new(raw.img_url.clone()),
            chips: Mutex::new(vec![]),
            raw,
        }
    }
    
    pub async fn chips_size(&self) -> usize {
        self.chips.lock().await.len()
    }
}

pub struct ChipOnBoard {
    pub board: Arc<Board>,
    pub index: i32,
}

impl ChipOnBoard {
    pub async fn from_pointer(game: &Game, pointer: &ChipPointer) -> Self {
        Self {
            board: game.board(&pointer.board).await.unwrap(),
            index: pointer.index,
        }
    }
    pub async fn chip(&self) -> Option<Arc<Chip>> {
        let chips = self.board.chips.lock().await;
        let index = position(self.index, chips.len());
        chips.get(index).cloned()
    }
    pub async fn insert(&self, chip: Arc<Chip>) {
        let mut chips = self.board.chips.lock().await;
        let index = position(self.index, chips.len());
        chips.insert(index, chip);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardChange {
    #[serde(skip_serializing_if="Option::is_none")]
    img_url: Option<Box<str>>,
}

impl BoardChange {
    pub async fn apply_to(&self, board: &Board) {
        if let Some(img_url) = self.img_url.clone() {
            *board.img_url.lock().await = img_url;
        }
    }
}
