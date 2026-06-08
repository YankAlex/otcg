use std::sync::Arc;

use crate::{game::{pile::{CardInPile, Pile}, player::Player, visibility::Visibility}, storage::{card::Card, rules::Rules}};

pub struct Hand {
    pub pile: Arc<Pile>,
}

impl Hand {
    pub fn from_cards(cards: Vec<Arc<Card>>, player: Player, rules: Rules) -> Self {
        Self {
            pile: Arc::new(Pile::new_with_cards(cards, rules.is_hand_shuffled, Visibility::Private, false, player)), // depends on
                                                                                  // gamerules
        }
    }
    pub fn new_empty(player: Player) -> Self {
        Self {
            pile: Arc::new(Pile::new_empty(Visibility::Private, false, player)), // depends on
                                                                                  // gamerules
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
    pub async fn pull_out_at(&self, index: usize) -> Option<Arc<Card>> {
        CardInPile::new(self.pile.clone(), index as i32).take().await
    }
    pub async fn add_card(&self, card: Arc<Card>) {
        CardInPile::top_of(self.pile.clone()).insert(card).await;
    }
    pub async fn pull_out_to(&self, index: usize, pile: Arc<Pile>) {
        let source = CardInPile::new(self.pile.clone(), index as i32);
        let destination = CardInPile::top_of(pile);
        source.move_to(destination).await;
    }
}
