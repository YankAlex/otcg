use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

use crate::{game::{player::Player, viewable::Viewable, visibility::Visibility}, storage::Error};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawCard {
    pub r#type: Box<str>,
    pub name: Box<str>,
    #[serde(default)]
    pub cost: i32,
    #[serde(default)]
    pub color_cost: Vec<Box<str>>,
    #[serde(default)]
    pub power: i32,
    #[serde(default)]
    pub health: i32,
    #[serde(default)]
    pub description: Box<str>,
    #[serde(default)]
    pub tags: Vec<Box<str>>,
    #[serde(default)]
    pub colors: Vec<Box<str>>,
    #[serde(default)]
    pub rarity: Box<str>,
    #[serde(default)]
    pub art_url: Box<str>,
    #[serde(default)]
    pub card_picture_url: Box<str>,
    pub nature: Box<str>,
    #[serde(default)]
    pub back_side_url: Box<str>,
    #[serde(default)]
    pub set: Box<str>,
    #[serde(default)]
    pub set_index: Box<str>,
    #[serde(default="default_language")]
    pub language: Box<str>,
}

fn default_language() -> Box<str> {
    "EN".into()
}

#[derive(Debug)]
pub struct Card {
    pub raw: RawCard,
    pub name: Mutex<Box<str>>,
    pub power: Mutex<i32>,
    pub health: Mutex<i32>,
    pub cost: Mutex<i32>,
    pub color_cost: Mutex<Vec<Box<str>>>,
    pub description: Mutex<Box<str>>,
    pub tags: Mutex<Vec<Box<str>>>,
    pub colors: Mutex<Vec<Box<str>>>,
    pub visibility: Mutex<Visibility>,
    pub owner: Mutex<Player>,
    pub comments: Mutex<Box<str>>,
    pub tapped: Mutex<bool>,
}

impl Viewable for Card {
    async fn owner(&self) -> Player {
        self.owner.lock().await.clone()
    }
    async fn visibility(&self) -> Visibility {
        self.visibility.lock().await.clone()
    }
}

impl RawCard {
    pub fn from_value(value: serde_json::Value) -> Result<Self, Error> {
        serde_json::from_value(value.clone()).map_err(|err| {
            Error::WrongValue(value, format!("{err}").into())
        })
    }
}

impl Card {
    pub fn from_raw(raw_card: &RawCard, owner: Player, visibility: Visibility) -> Self {
        Card {
            raw: raw_card.clone(),
            name: Mutex::new(raw_card.name.clone()),
            power: Mutex::new(raw_card.power.clone()),
            health: Mutex::new(raw_card.health.clone()),
            cost: Mutex::new(raw_card.cost.clone()),
            color_cost: Mutex::new(raw_card.color_cost.clone()),
            description: Mutex::new(raw_card.description.clone()),
            tags: Mutex::new(raw_card.tags.clone()),
            colors: Mutex::new(raw_card.colors.clone()),
            owner: Mutex::new(owner),
            visibility: Mutex::new(visibility),
            comments: Mutex::new("".into()),
            tapped: Mutex::new(false), 
        }
    }
}

