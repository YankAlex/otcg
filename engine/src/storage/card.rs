use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

use crate::{game::{player::{self, Player}, visibility::Visibility}, storage::Error};

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
}

#[derive(Debug)]
pub struct Card {
    pub raw: RawCard,
    pub r#type: Box<str>,
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
    pub rarity: Box<str>,
    pub art_url: Box<str>,
    pub card_picture_url: Box<str>,
    pub nature: Box<str>,
    pub back_side_url: Box<str>,
}

impl Card {
    pub async fn can_be_viewed_by(&self, player: &Player) -> bool {
        match player {
            &player::ADMIN => true,
            player if *player == *self.owner.lock().await && self.visibility.lock().await.can_be_viewed_by_owner() => true,
            _ if self.visibility.lock().await.can_be_viewed_by_not_owner() => true,
            _ => false,
        }
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
            r#type: raw_card.r#type.clone(),
            rarity: raw_card.rarity.clone(),
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
            art_url: raw_card.art_url.clone(),
            card_picture_url: raw_card.card_picture_url.clone(),
            nature: raw_card.nature.clone(),
            tapped: Mutex::new(false), 
            back_side_url: raw_card.back_side_url.clone(),
        }
    }
}

