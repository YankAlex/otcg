use std::{fs, path::Path};
use serde_json::{from_str, Value, value::Map};
use tokio::sync::Mutex;

use crate::storage::{card::RawCard, chip::RawChip};

pub mod card;
pub mod board;
pub mod chip;
pub mod rules;

pub struct Library {
    cards: Mutex<Map<String, Value>>,
    chips: Mutex<Map<String, Value>>,
}

impl Library {
    pub fn new(path: &Path) -> Self {
        match path.is_file() {
            true => {
                let string = fs::read_to_string(path).unwrap();
                let value: Value = from_str(&string).unwrap();
                Self {
                    cards: Mutex::new(value.get("cards").unwrap_or(&Map::new().into()).as_object().unwrap().clone()),
                    chips: Mutex::new(value.get("chips").unwrap_or(&Map::new().into()).as_object().unwrap().clone()),
                }
            },
            false => {
                let files = path.read_dir().unwrap();
                let mut cards = Map::new();
                let mut chips = Map::new();
                for file in files {
                    if let Ok(file_entry) = file {
                        if let Ok(string) = fs::read_to_string(file_entry.path()) {
                            if let Ok(value) = from_str::<Value>(&string) {
                                cards.append(&mut value.get("cards").unwrap_or(&Map::new().into()).as_object().unwrap().clone());
                                chips.append(&mut value.get("chips").unwrap_or(&Map::new().into()).as_object().unwrap().clone());
                            } else {
                                log::info!("{:?}: {:?}", file_entry.path(), from_str::<Value>(&string));
                            }
                        }
                    }
                }
                log::info!("{:?}", cards);
                Self {
                    cards: Mutex::new(cards),
                    chips: Mutex::new(chips),
                }
            }
        }
    }
    pub async fn get_raw_card_by_name(&self, name: &str) -> Result<RawCard, Error> {
        let locked_objects = self.cards.lock().await;
        let Some(value) = locked_objects.get(name) else {
            return Err(Error::DontExist(name.into()));
        };
        RawCard::from_value(value.clone())
    }
    pub async fn get_raw_chip_by_name(&self, name: &str) -> Result<RawChip, Error> {
        let locked_objects = self.chips.lock().await;
        let Some(value) = locked_objects.get(name) else {
            return Err(Error::DontExist(name.into()));
        };
        RawChip::from_value(value.clone())
    }
}

#[derive(Debug)]
pub enum Error {
    DontExist(Box<str>),
    WrongValue(Value, Box<str>),
}

