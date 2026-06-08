use tokio::sync::Mutex;

pub struct Chip {
    raw: RawChip,
    health: Mutex<i32>,
    coordinates: Mutex<(i32, i32)>,
}

pub struct RawChip {
    img_url: Box<str>,
    health: i32,
    color: Box<str>,
}

impl Chip {
    pub fn new_from_raw(raw: RawChip) -> Self {
        Self {
            health: Mutex::new(raw.health),
            coordinates: Mutex::new((0, 0)),
            raw,
        }
    }
}


