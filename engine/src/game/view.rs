use std::sync::Arc;

use crate::{game::{coordinates::Coordinates, pile::Pile, player::Player, viewable::Viewable, visibility::Visibility}, storage::{board::{Board, RawBoard}, card::{Card, RawCard}, chip::{Chip, RawChip}}};
use futures_util::future::join_all;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CardView {
    #[serde(skip_serializing_if = "Option::is_none")]
    raw: Option<RawCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cost: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_cost: Option<Vec<Box<str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<Box<str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colors: Option<Vec<Box<str>>>,
    visibility: Visibility,
    tapped: bool,
    owner: Player,
    #[serde(skip_serializing_if = "Option::is_none")]
    art_url: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_picture_url: Option<Box<str>>,
    comments: Box<str>,
    nature: Box<str>,
    visible_to_me: bool,
    back_side_url: Box<str>
}

#[derive(Serialize, Deserialize)]
pub struct CardChange {
    #[serde(skip_serializing_if="Option::is_none")]
    power: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    health: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    cost: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    color_cost: Option<Vec<Box<str>>>,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<Box<str>>,
    #[serde(skip_serializing_if="Option::is_none")]
    tags: Option<Vec<Box<str>>>,
    #[serde(skip_serializing_if="Option::is_none")]
    visibility: Option<Visibility>,
    #[serde(skip_serializing_if="Option::is_none")]
    comments: Option<Box<str>>,
    #[serde(skip_serializing_if="Option::is_none")]
    tapped: Option<bool>,
}

impl CardView {
    pub async fn from_card(card: Arc<Card>, viewer: &Player) -> Self {
        if card.can_be_viewed_by(viewer).await {
            CardView {
                raw: Some(card.raw.clone()),
                r#type: Some(card.raw.r#type.clone()),
                name: Some(card.name.lock().await.clone()),
                power: Some(card.power.lock().await.clone()),
                health: Some(card.health.lock().await.clone()),
                cost: Some(card.cost.lock().await.clone()),
                color_cost: Some(card.color_cost.lock().await.clone()),
                description: Some(card.description.lock().await.clone()),
                tags: Some(card.tags.lock().await.clone()),
                colors: Some(card.colors.lock().await.clone()),
                visibility: card.visibility.lock().await.clone(),
                owner: card.owner.lock().await.clone(),
                rarity: Some(card.raw.rarity.clone()),
                art_url: Some(card.raw.art_url.clone()),
                card_picture_url: Some(card.raw.card_picture_url.clone()),
                comments: card.comments.lock().await.clone(),
                nature: card.raw.nature.clone(),
                tapped: card.tapped.lock().await.clone(),
                back_side_url: card.raw.back_side_url.clone(),
                visible_to_me: true,
            }
        } else {
            CardView {
                raw: None,
                r#type: None, 
                name: None,
                power: None,
                health: None,
                cost: None,
                color_cost: None,
                description: None,
                tags: None,
                colors: None,
                rarity: None,
                art_url: None,
                card_picture_url: None,
                visibility: card.visibility.lock().await.clone(),
                owner: card.owner.lock().await.clone(),
                comments: card.comments.lock().await.clone(),
                nature: card.raw.nature.clone(),
                tapped: card.tapped.lock().await.clone(),
                back_side_url: card.raw.back_side_url.clone(),
                visible_to_me: false,
            }
        }
    }
}

impl CardChange {
    pub fn from_raw_card(card: &RawCard) -> Self {
        Self {
            power: Some(card.power),
            health: Some(card.health),
            cost: Some(card.cost),
            color_cost: Some(card.color_cost.clone()),
            description: Some(card.description.clone()),
            tags: Some(card.tags.clone()),
            comments: None,
            visibility: None,
            tapped: None,
        }
    }

    pub async fn apply_to(&self, card: Arc<Card>) {
        if let Some(val) = &self.tags {
            *card.tags.lock().await = val.clone();
        }
        if let Some(val) = &self.description {
            *card.description.lock().await = val.clone();
        }
        if let Some(val) = &self.cost {
            *card.cost.lock().await = val.clone();
        }
        if let Some(val) = &self.color_cost {
            *card.color_cost.lock().await = val.clone();
        }
        if let Some(val) = &self.health {
            *card.health.lock().await = val.clone();
        }
        if let Some(val) = &self.power {
            *card.power.lock().await = val.clone();
        }
        if let Some(val) = &self.visibility {
            *card.visibility.lock().await = val.clone();
        }
        if let Some(val) = &self.comments {
            *card.comments.lock().await = val.clone();
        }
        if let Some(val) = &self.tapped {
            *card.tapped.lock().await = val.clone();
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PileView {
    cards: Vec<CardView>, 
    only_raw_cards: bool,
    default_visibility: Visibility,
}

impl PileView {
    pub async fn from_pile(pile: Arc<Pile>, viewer: &Player) -> Self {
        Self {
            only_raw_cards: pile.config.only_raw_cards,
            default_visibility: pile.config.default_visibility.clone(),
            cards: join_all(pile.cards().await.into_iter().map(async |card| {CardView::from_card(card, viewer).await}).collect::<Vec<_>>()).await,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChipView {
    raw: Option<RawChip>,
    health: Option<i32>,
    coordinates: Coordinates,
    owner: Player,
}

impl ChipView {
    pub async fn from_chip(chip: Arc<Chip>, viewer: &Player) -> Self {
        if chip.can_be_viewed_by(viewer).await {
            Self {
                raw: Some(chip.raw.clone()),
                health: Some(chip.health.lock().await.clone()),
                coordinates: chip.coordinates.lock().await.clone(),
                owner: chip.owner.lock().await.clone(),
            }
        } else {
            Self {
                raw: None,
                health: None,
                coordinates: chip.coordinates.lock().await.clone(),
                owner: chip.owner.lock().await.clone(),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BoardView {
    raw: RawBoard,
    img_url: Box<str>,
    chips: Vec<ChipView>,
}

impl BoardView {
    pub async fn from_board(board: Arc<Board>, viewer: &Player) -> Self {
        Self {
            raw: board.raw.clone(),
            img_url: board.img_url.lock().await.clone(),
            chips: join_all(board.chips.lock().await.iter().map(async |chip| ChipView::from_chip(chip.clone(), viewer).await)).await,
        }
    }
}

