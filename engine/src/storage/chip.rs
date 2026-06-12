use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{game::{coordinates::Coordinates, player::Player, viewable::Viewable, visibility::Visibility}, storage::Error};

pub struct Chip {
    pub raw: RawChip,
    pub health: Mutex<i32>,
    pub coordinates: Mutex<Coordinates>,
    pub owner: Mutex<Player>,
    pub visibility: Mutex<Visibility>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawChip {
    name: Box<str>,
    art_url: Box<str>,
    health: i32,
    colors: Vec<Box<str>>,
}

impl RawChip {
    pub fn from_value(value: serde_json::Value) -> Result<Self, Error> {
        serde_json::from_value(value.clone()).map_err(|err| {
            Error::WrongValue(value, format!("{err}").into())
        })
    }
}

impl Chip {
    pub fn new_from_raw(raw: RawChip, owner: Player, visibility: Visibility) -> Self {
        Self {
            health: Mutex::new(raw.health),
            coordinates: Mutex::new(Coordinates::new(0.0, 0.0)),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ChipChange {
    #[serde(skip_serializing_if="Option::is_none")]
    visibility: Option<Visibility>,
    #[serde(skip_serializing_if="Option::is_none")]
    health: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    coordinates: Option<Coordinates>,
}

impl ChipChange {
    pub async fn apply_to(&self, chip: &Chip) {
        if let Some(value) = &self.visibility {
            *chip.visibility.lock().await = value.clone();
        }
        if let Some(value) = &self.health {
            *chip.health.lock().await = *value;
        }
        if let Some(value) = &self.coordinates {
            *chip.coordinates.lock().await = value.clone();
        }
    }
}
