use driftwave_core::{PlaybackListener, PlaybackState, Player, PlayerError};
use std::path::Path;
use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};

pub struct WebPlayer {
    context: AudioContext,
}

pub struct WebSound {
    buffer: AudioBuffer,
}

pub struct WebPlayback {
    source: AudioBufferSourceNode,
    start_time: f64,
    paused_at: Option<f64>,
}

impl WebPlayer {
    pub fn new() -> Result<Self, JsValue> {
        let context = AudioContext::new()?;
        Ok(WebPlayer { context })
    }
}

impl Player for WebPlayer {
    type Sound = WebSound;
    type Playback = WebPlayback;
    type PlaybackListener = ();

    fn init(&mut self) -> Result<(), PlayerError> {
        Ok(())
    }

    fn load(&mut self, path: &Path) -> Result<Self::Sound, PlayerError> {
        Err(PlayerError {
            message: "Web audio loading not yet implemented".to_string(),
        })
    }

    fn play_from(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        _listener: Option<Self::PlaybackListener>,
    ) -> Result<Self::Playback, PlayerError> {
        let source = self
            .context
            .create_buffer_source()
            .map_err(|e| PlayerError {
                message: format!("Failed to create buffer source: {:?}", e),
            })?;

        source.set_buffer(Some(&sound.buffer));
        source
            .connect_with_audio_node(&self.context.destination())
            .map_err(|e| PlayerError {
                message: format!("Failed to connect to destination: {:?}", e),
            })?;

        let sample_rate = self.context.sample_rate();
        let start_time = start_frame as f64 / sample_rate as f64;

        source
            .start_with_when_and_grain_offset(0.0, start_time)
            .map_err(|e| PlayerError {
                message: format!("Failed to start playback: {:?}", e),
            })?;

        Ok(WebPlayback {
            source,
            start_time: self.context.current_time(),
            paused_at: None,
        })
    }

    fn play_range(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        end_frame: u64,
    ) -> Result<Self::Playback, PlayerError> {
        let source = self
            .context
            .create_buffer_source()
            .map_err(|e| PlayerError {
                message: format!("Failed to create buffer source: {:?}", e),
            })?;

        source.set_buffer(Some(&sound.buffer));
        source
            .connect_with_audio_node(&self.context.destination())
            .map_err(|e| PlayerError {
                message: format!("Failed to connect to destination: {:?}", e),
            })?;

        let sample_rate = self.context.sample_rate();
        let start_time = start_frame as f64 / sample_rate as f64;
        let duration = (end_frame - start_frame) as f64 / sample_rate as f64;

        source
            .start_with_when_and_grain_offset_and_grain_duration(0.0, start_time, duration)
            .map_err(|e| PlayerError {
                message: format!("Failed to start playback: {:?}", e),
            })?;

        Ok(WebPlayback {
            source,
            start_time: self.context.current_time(),
            paused_at: None,
        })
    }

    fn pause(&mut self, playback: &mut Self::Playback) -> Result<Self::Playback, PlayerError> {
        playback.source.stop().map_err(|e| PlayerError {
            message: format!("Failed to stop playback: {:?}", e),
        })?;

        playback.paused_at = Some(self.context.current_time());

        Err(PlayerError {
            message: "Pause/resume not fully implemented yet".to_string(),
        })
    }

    fn get_state(&mut self, playback: &mut Self::Playback) -> Result<PlaybackState, PlayerError> {
        if playback.paused_at.is_some() {
            Ok(PlaybackState::Paused)
        } else {
            Ok(PlaybackState::Playing)
        }
    }

    fn close(&mut self) -> Result<(), PlayerError> {
        let _ = self.context.close();
        Ok(())
    }
}
