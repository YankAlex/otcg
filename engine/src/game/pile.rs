use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{game::{Game, player::Player, pointer::CardPointer, position, visibility::Visibility}, storage::card::{Card, RawCard}};

#[derive(Debug)]
pub struct PileConfig {
    pub only_raw_cards: bool,
    pub default_visibility: Visibility,
    pub owner: Player,
    pub shuffled: bool,
}

#[derive(Debug)]
pub struct Pile {
    cards: Mutex<Vec<Arc<Card>>>, 
    pub config: PileConfig,
}

impl Pile {
    pub fn new_empty(config: PileConfig) -> Self {
        Pile {
            cards: Mutex::new(Vec::new()),
            config,
        }
    }
    
    pub fn from_raw_cards(config: PileConfig, cards: Vec<RawCard>) -> Self {
        Pile {
            cards: Mutex::new(cards.iter().map(|raw| Arc::new(Card::from_raw(raw, config.owner.clone(), config.default_visibility.clone()))).collect()),
            config,
        }
    }

    pub fn new(config: PileConfig, mut cards: Vec<Arc<Card>>) -> Self {
        if config.shuffled {
            rand::seq::SliceRandom::shuffle(cards.as_mut_slice(), &mut rand::rng());
        }
        Pile {
            config,
            cards: Mutex::new(cards),
        }
    }

    pub async fn cards(&self) -> Vec<Arc<Card>> {
        self.cards.lock().await.clone()
    }

    pub async fn size(&self) -> usize {
        self.cards.lock().await.len()
    }
}

pub struct CardInPile {
    pile: Arc<Pile>,
    index: i32,
}


impl CardInPile {
    pub fn new(pile: Arc<Pile>, index: i32) -> Self {
        Self {
            pile, index,
        }
    }

    pub async fn from_pointer(game: &Game, pointer: &CardPointer) -> Self {
        Self::new(game.pile(&pointer.pile).await, pointer.index)
    }
    
    pub fn top_of(pile: Arc<Pile>) -> Self {
        Self {
            pile, index: 0,
        }
    }

    pub async fn bottom_of(pile: Arc<Pile>) -> Self {
        Self {
            pile: pile.clone(), index: -1,
        }
    }

    pub async fn insert(&self, card: Arc<Card>) {
        let mut cards = self.pile.cards.lock().await;
        let index = position(self.index, cards.len());
        cards.insert(index, card);
    }

    pub async fn take(self) -> Option<Arc<Card>> {
        let mut cards = self.pile.cards.lock().await;
        let index = position(self.index, cards.len());
        if index < cards.len() {
            Some(cards.remove(index))
        } else {
            None
        }
    }

    pub async fn move_to(self: Self, to: Self) -> Option<Arc<Card>> {
        let card = self.take().await;
        match card {
            Some(card) => {
                to.insert(card.clone()).await;
                Some(card)
            },
            None => None,
        }
    }
    
    pub async fn card(&self) -> Option<Arc<Card>> {
        let cards = self.pile.cards.lock().await;
        let index = position(self.index, cards.len());
        cards.get(index).cloned()
    }

    pub fn pile(&self) -> Arc<Pile> {
        self.pile.clone()
    }
}
