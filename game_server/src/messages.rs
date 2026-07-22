use engine::{game::{coordinates::Coordinates, pointer::{BoardPointer, CardPointer, ChipPointer, PilePointer}, view::CardChange}, storage::{board::BoardChange, chip::ChipChange}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerMessage {
    MoveCard {
        source: CardPointer,
        destination: CardPointer,
    },
    ShuffleCardToPile {
        source: CardPointer,
        destination: PilePointer,
    },
    ShufflePile {
        target: PilePointer,
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
    CreateChip {
        destination: ChipPointer,
        coordinates: Coordinates,
        name: Box<str>,
    },
    ChangeChip {
        target: ChipPointer,
        changes: ChipChange,
    },
    ChangeBoard {
        target: BoardPointer,
        changes: BoardChange,
    },
    TurnEnd,
    Surrender,
    ViewPile (PilePointer),
    ViewBoard (BoardPointer),
    ViewCard (CardPointer),
    ViewChip (ChipPointer),
    GameInfo,
}

