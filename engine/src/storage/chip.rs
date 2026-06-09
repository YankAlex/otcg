use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::game::{coordinates::Coordinates, player::Player, viewable::Viewable, visibility::Visibility};

pub struct Chip {
    pub raw: RawChip,
    pub health: Mutex<i32>,
    pub coordinates: Mutex<Coordinates>,
    pub owner: Mutex<Player>,
    pub visibility: Mutex<Visibility>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawChip {
    img_url: Box<str>,
    health: i32,
    color: Box<str>,
}

impl Chip {
    pub fn new_from_raw(raw: RawChip, owner: Player, visibility: Visibility) -> Self {
        Self {
            health: Mutex::new(raw.health),
            coordinates: Mutex::new(Coordinates::new(0, 0)),
            owner: Mutex::new(owner),
            visibility: Mutex::new(visibility),
            raw,
        }
    }
}

impl Viewable for Chip {
    async fn visibility(&self) -> Visibility {
        self.visibility.lock().await.clone()
    }
    async fn owner(&self) -> Player {
        self.owner.lock().await.clone()
    }
}


