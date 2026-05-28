use std::{fs, path::Path};
use serde_json::{from_str, Value, value::Map};
use tokio::sync::Mutex;

use crate::storage::card::RawCard;

pub mod card;
pub mod rules;

pub struct Library {
    objects: Mutex<Map<String, Value>>,
}

impl Library {
    pub fn new(path: &Path) -> Self {
        let string = fs::read_to_string(path).unwrap();
        let value: Value = from_str(&string).unwrap();
        Self {
            objects: Mutex::new(value.as_object().unwrap().clone()),
        }
    }
    pub async fn get_raw_card_by_name(&self, name: &str) -> Result<RawCard, Error> {
        let locked_objects = self.objects.lock().await;
        let Some(value) = locked_objects.get(name) else {
            return Err(Error::DontExist(name.into()));
        };
        RawCard::from_value(value.clone())
    }
}

#[derive(Debug)]
pub enum Error {
    DontExist(Box<str>),
    WrongValue(Value, Box<str>),
}

