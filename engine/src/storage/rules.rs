use crate::game::player::{self, Player};

pub struct Rules {
    pub fight_areas_count: usize,
    pub only_raw_card_in_hand: bool,
    pub only_raw_card_in_main_deck: bool,
    pub only_raw_card_in_mana_deck: bool,
    pub only_raw_card_in_mana_pool: bool,
    pub only_raw_card_in_trash_deck: bool,
    pub only_raw_card_in_special_zone: bool,
}

impl Rules {
    pub fn new(name: &str) -> Self {
        match name {
            "riftbound" => Self {
                fight_areas_count: 2,
                only_raw_card_in_hand: true,
                only_raw_card_in_main_deck: true,
                only_raw_card_in_mana_deck: true,
                only_raw_card_in_mana_pool: true,
                only_raw_card_in_trash_deck: true,
                only_raw_card_in_special_zone: true,
            },
            _ => Self {
                fight_areas_count: 1,
                only_raw_card_in_hand: false,
                only_raw_card_in_main_deck: false,
                only_raw_card_in_mana_deck: false,
                only_raw_card_in_mana_pool: true,
                only_raw_card_in_trash_deck: false,
                only_raw_card_in_special_zone: false,
            }
        }
    }
    pub fn rights_to_touch_ones_pile(&self, player: &Player, pile_owner: &Player, card_owner: &Player) -> bool {
        if *player == player::WATCHER {
            return false;
        }
        *player == player::ADMIN || *pile_owner == player::ADMIN && card_owner == player || player == pile_owner
    }
}
