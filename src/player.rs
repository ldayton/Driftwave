use std::fmt;
use std::path::Path;

pub trait Player {
    type Sound;
    type Playback;

    fn init(&mut self) -> Result<(), PlayerError>;

    fn load(&mut self, path: &Path) -> Result<Self::Sound, PlayerError>;

    fn play(&mut self, sound: &mut Self::Sound) -> Result<Self::Playback, PlayerError>;

    fn is_playing(&mut self, playback: &mut Self::Playback) -> Result<bool, PlayerError>;

    fn close(&mut self) -> Result<(), PlayerError>;
}

#[derive(Debug)]
pub struct PlayerError {
    pub message: String,
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PlayerError {}
