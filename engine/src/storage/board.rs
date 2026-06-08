use std::sync::Arc;

use tokio::sync::Mutex;

use crate::storage::chip::Chip;

pub struct Board {
    config: BoardConfig,
    img_url: Mutex<Box<str>>,
    pub chips: Mutex<Vec<Arc<Chip>>>,
}

pub struct BoardConfig {
    pub height: usize,
    pub width: usize,
    pub default_img_url: Box<str>,
}

impl Board {
    pub fn new_empty(config: BoardConfig) -> Self {
        Self {
            img_url: Mutex::new(config.default_img_url.clone()),
            config: config,
            chips: Mutex::new(vec![]),
        }
    }
    
    pub async fn chips_size(&self) -> usize {
        self.chips.lock().await.len()
    }
}
