use std::io::Error;
use std::path::Path;

pub trait Player {
    fn init(&mut self) -> Result<PlayerHandle, Error>;

    fn load(&mut self, player_handle: &PlayerHandle, path: &Path) -> Result<AudioHandle, Error>;

    fn play(&mut self, player_handle: &PlayerHandle, audio_handle: &AudioHandle) -> Result<PlaybackHandle, Error>;

    fn pause(&mut self, player_handle: &PlayerHandle, playback_handle: &PlaybackHandle) -> Option<Error>;

    fn close(&mut self, player_handle: &PlayerHandle) -> Option<Error>;
}

pub struct FmodPlayer {}

impl Player for FmodPlayer {
    fn init(&mut self) -> Result<PlayerHandle, Error> {
        Ok(PlayerHandle {})
    }

    fn load(&mut self, _player_handle: &PlayerHandle, _path: &Path) -> Result<AudioHandle, Error> {
        Ok(AudioHandle {})
    }

    fn play(&mut self, _player_handle: &PlayerHandle, _audio_handle: &AudioHandle) -> Result<PlaybackHandle, Error> {
        Ok(PlaybackHandle {})
    }

    fn pause(&mut self, _player_handle: &PlayerHandle, _playback_handle: &PlaybackHandle) -> Option<Error> {
        None
    }

    fn close(&mut self, _handle: &PlayerHandle) -> Option<Error> {
        None
    }
}

pub struct PlayerHandle {}

pub struct AudioHandle {}

pub struct PlaybackHandle {}
