use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coordinates {
    x: f32,
    y: f32,
}

impl Coordinates {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
