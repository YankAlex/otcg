use std::collections::HashMap;

use crate::{game::{pile::PileConfig, player::{self, Player}, visibility::Visibility}, storage::board::RawBoard};

pub struct Rules {
    name: Box<str>,
}

impl Rules {
    pub fn new(name: &str) -> Self {
        match name {
            "riftbound" => Self {
                name: name.into(),
            },
            _ => Self {
                name: name.into(),
            }
        }
    }
    pub fn battlefields_count(&self) -> usize {
        match &self.name[..] {
            "riftbound" => 2,
            _ => 1,
        }
    }
    pub fn rights_to_touch_ones_pile(&self, player: &Player, pile_owner: &Player, card_owner: &Player) -> bool {
        if *player == player::WATCHER {
            return false;
        }
        *player == player::ADMIN || *pile_owner == player::ADMIN && card_owner == player || player == pile_owner
    }
    pub fn piles(&self, player: Player) -> HashMap<Box<str>, PileConfig> {
        match &self.name[..] {
            "unmatched" => {
                let mut piles = HashMap::new();
                match player {
                    player::WATCHER => {}
                    player::ADMIN => {},
                    player => {
                        piles.insert("heroes".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player.clone(),
                        });
                        piles.insert("hand".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Private,
                            shuffled: true,
                            owner: player.clone(),
                        });
                        piles.insert("main_deck".into(), PileConfig {
                            only_raw_cards: true,
                            default_visibility: Visibility::Secret,
                            shuffled: true,
                            owner: player.clone(),
                        });
                        piles.insert("additional_deck".into(), PileConfig {
                            only_raw_cards: true,
                            default_visibility: Visibility::Secret,
                            shuffled: true,
                            owner: player.clone(),
                        });
                        piles.insert("trash_deck".into(), PileConfig {
                            only_raw_cards: true,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player.clone(),
                        });
                        piles.insert("base".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player.clone(),
                        });
                    }
                }
                piles 
            },
            _ => {
                let mut piles = HashMap::new();
                match player {
                    player::WATCHER => {}
                    player::ADMIN => {
                        piles.insert("spell_queue".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player::ADMIN
                        });
                    },
                    player => {
                        piles.insert("heroes".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player.clone(),
                        });
                        piles.insert("hand".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Private,
                            shuffled: true,
                            owner: player.clone(),
                        });
                        piles.insert("main_deck".into(), PileConfig {
                            only_raw_cards: true,
                            default_visibility: Visibility::Secret,
                            shuffled: true,
                            owner: player.clone(),
                        });
                        piles.insert("mana_deck".into(), PileConfig {
                            only_raw_cards: true,
                            default_visibility: Visibility::Secret,
                            shuffled: true,
                            owner: player.clone(),
                        });
                        piles.insert("trash_deck".into(), PileConfig {
                            only_raw_cards: true,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player.clone(),
                        });
                        piles.insert("mana_pool".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Public,
                            shuffled: true,
                            owner: player.clone(),
                        });
                        piles.insert("base".into(), PileConfig {
                            only_raw_cards: false,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player.clone(),
                        });
                        piles.insert("special_zone".into(), PileConfig {
                            only_raw_cards: true,
                            default_visibility: Visibility::Public,
                            shuffled: false,
                            owner: player.clone(),
                        });
                    }
                }
                piles 
            }
        }
    }
    pub fn boards(&self) -> HashMap<Box<str>, RawBoard> {
        match &self.name[..] {
            "unmatched" => {
                let mut boards = HashMap::new();
                boards.insert("board".into(), RawBoard {
                    height: 100,
                    width: 100,
                    img_url: "".into(),
                });
                boards
            },
            _ => HashMap::new(),
        }
    }
}
