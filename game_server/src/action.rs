use engine::game::{player::Player, pointer::{BoardPointer, CardPointer, ChipPointer, PilePointer}, view::{BoardView, CardView, ChipView, PileView}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Action {
    CardMoved {
        source: CardPointer,
        destination: CardPointer,
    },
    CardCreated {
        destination: CardPointer,
        card: CardView,
    },
    CardChanged {
        target: CardPointer,
        new_card: CardView,
    },
    GameInfo {
        your_number: i32,
        players_count: usize,
        battlefields_count: usize,
    },
    ViewPile {
        target: PilePointer,
        pile: PileView,
    },
    ViewCard {
        target: CardPointer,
        card: CardView,
    },
    ViewBoard {
        target: BoardPointer,
        board: BoardView,
    },
    ViewChip {
        target: ChipPointer,
        chip: ChipView,
    },
    ChipCreated {
        destination: ChipPointer,
        chip: ChipView,
    },
    ChipChanged {
        target: ChipPointer,
        new_chip: ChipView,
    },
    NextTurn (Player),
    BackgroundRequest,
}
