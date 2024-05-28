use std::fmt::Display;

/// Represents status the game is currently in
pub enum GameStatus {
    Playing,
    GameOver,
    Victory,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatus::Playing => write!(f, ""),
            GameStatus::GameOver => write!(f, "Game Over!"),
            GameStatus::Victory => write!(f, "Victory!"),
        }
    }
}
