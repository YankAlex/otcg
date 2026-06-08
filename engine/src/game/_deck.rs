use std::sync::Arc;

use crate::{game::{pile::{CardInPile, Pile}, player::Player, visibility::Visibility}, storage::card::Card};

pub struct Deck {
    pub pile: Arc<Pile>,
    pub owner: Player,
}

impl Deck {
    pub fn from_cards(cards: Vec<Arc<Card>>, shuffled: bool, owner: Player, visibility: Visibility, only_raw_cards: bool) -> Self {
        Deck {
            pile: Arc::new(Pile::new_with_cards(
                cards,
                shuffled,
                visibility,
                only_raw_cards, // depends on rules
                owner.clone(),
            )),
            owner,
        }
    }
    pub fn new_empty(owner: Player, visibility: Visibility, only_raw_cards: bool) -> Self {
        Deck {
            pile: Arc::new(Pile::new_empty(
                visibility,
                only_raw_cards,
                owner.clone()
            )),
            owner,
        }
    }
    pub async fn cards(&self) -> Vec<Arc<Card>>{
        self.pile.cards().await
    }
    pub async fn size(&self) -> usize {
        self.pile.size().await
    }
    pub async fn is_empty(&self) -> bool {
        self.size().await == 0
    }
    pub fn top(&self) -> CardInPile {
        CardInPile::top_of(self.pile.clone())
    }
    pub async fn top_card(&self) -> Option<Arc<Card>> {
        self.top().card().await
    }
    pub async fn take_top(&self) -> Option<Arc<Card>> {
        self.top().take().await
    }
    pub async fn put_top(&self, card: Arc<Card>) {
        self.top().insert(card).await
    }
    pub async fn take_top_to(&self, pile: Arc<Pile>) {
        let top_card = CardInPile::new(self.pile.clone(), 0);
        let destination = CardInPile::new(pile, 0);
        top_card.move_to(destination).await;
    }
} 
