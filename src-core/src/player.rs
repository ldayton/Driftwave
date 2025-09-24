use async_trait::async_trait;
use std::fmt;

#[async_trait(?Send)]
pub trait Player {
    type Sound;
    type Playback;
    type PlaybackListener;

    fn init(&mut self) -> Result<(), PlayerError>;

    async fn load(&mut self, source: &str) -> Result<Self::Sound, PlayerError>;

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
        listener: Option<Self::PlaybackListener>,
    ) -> Result<Self::Playback, PlayerError>;

    fn pause(&mut self, playback: &mut Self::Playback) -> Result<u64, PlayerError>;

    fn get_metadata(&mut self, sound: &mut Self::Sound) -> Result<Metadata, PlayerError>;

    fn get_state(&mut self, playback: &mut Self::Playback) -> Result<PlaybackState, PlayerError>;

    fn is_playing(&mut self, playback: &mut Self::Playback) -> Result<bool, PlayerError> {
        Ok(matches!(self.get_state(playback)?, PlaybackState::Playing))
    }
}

pub trait PlaybackListener: Send {
    fn on_progress(&mut self, position_frames: u64);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Playing,
    NotPlaying,
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

pub struct Metadata {
    pub sample_rate: u32,
    pub channel_count: u32,
    pub frame_count: u64,
}
