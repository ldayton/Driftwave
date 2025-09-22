use std::fmt;
use std::path::Path;

pub trait Player {
    type Sound;
    type Playback;
    type PlaybackListener;

    fn init(&mut self) -> Result<(), PlayerError>;

    fn load(&mut self, path: &Path) -> Result<Self::Sound, PlayerError>;

    fn play_from(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        listener: Option<Self::PlaybackListener>,
    ) -> Result<Self::Playback, PlayerError>;

    fn play_range(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        end_frame: u64,
    ) -> Result<Self::Playback, PlayerError>;

    fn pause(&mut self, sound: &mut Self::Playback) -> Result<Self::Playback, PlayerError>;

    fn get_state(&mut self, playback: &mut Self::Playback) -> Result<PlaybackState, PlayerError>;

    fn is_playing(&mut self, playback: &mut Self::Playback) -> Result<bool, PlayerError> {
        Ok(matches!(self.get_state(playback)?, PlaybackState::Playing))
    }

    fn is_paused(&mut self, playback: &mut Self::Playback) -> Result<bool, PlayerError> {
        Ok(matches!(self.get_state(playback)?, PlaybackState::Paused))
    }

    fn is_stopped(&mut self, playback: &mut Self::Playback) -> Result<bool, PlayerError> {
        Ok(matches!(self.get_state(playback)?, PlaybackState::Stopped))
    }

    fn close(&mut self) -> Result<(), PlayerError>;
}

pub trait PlaybackListener: Send {
    fn on_progress(&mut self, position_frames: u64);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
    Invalid,
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
