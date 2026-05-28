use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Player(pub i32);

pub const WATCHER: Player = Player(-1);
pub const ADMIN: Player = Player(0);
pub const A: Player = Player(1);
pub const B: Player = Player(2);

