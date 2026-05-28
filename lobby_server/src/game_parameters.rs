use std::path::PathBuf;

#[derive(Clone)]
pub struct GameParameters {
    pub name: Box<str>,
    pub players_count: usize,
    pub library_path: PathBuf,
}

impl Into<game_server::Config> for GameParameters {
    fn into(self) -> game_server::Config {
        game_server::Config {
            game_name: self.name,
            library_path: self.library_path,
        }
    }
}
