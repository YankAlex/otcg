use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
