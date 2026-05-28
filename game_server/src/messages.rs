use engine::game::{pointer::{CardPointer, PilePointer}, view::CardChange};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerMessage {
    MoveCard {
        source: CardPointer,
        destination: CardPointer,
    },
    ChangeCardToRaw {
        target: CardPointer,
    },
    ChangeCard {
        target: CardPointer,
        changes: CardChange,
    },
    CreateCard {
        destination: CardPointer,
        name: Box<str>,
    },
    TurnEnd,
    Surrender,
    ViewPile (PilePointer),
    ViewCard (CardPointer),
    GameInfo,
}

