use engine::game::{player::Player, pointer::{CardPointer, PilePointer}, view::{CardView, PileView}};
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
        fight_areas_count: usize,
    },
    PlayersCount (usize),
    FightAreasCount (usize),
    ViewPile {
        target: PilePointer,
        pile: PileView,
    },
    ViewCard {
        target: CardPointer,
        card: CardView,
    },
    NextTurn (Player),
    BackgroundRequest,
}
