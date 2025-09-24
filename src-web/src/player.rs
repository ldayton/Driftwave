use async_trait::async_trait;
use driftwave_core::{Metadata, PlaybackState, Player, PlayerError, PlaybackListener};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, Request, Response};

pub struct WebPlayer {
    context: AudioContext,
}

pub struct WebSound {
    buffer: AudioBuffer,
    sample_rate: f32,
    channels: u32,
    frame_count: u32,
}

pub struct WebPlayback {
    source: Option<AudioBufferSourceNode>,  // Current source node (None when paused)
    buffer: AudioBuffer,                     // The audio buffer to play
    start_time: f64,                        // When playback started (context time)
    start_frame: u64,                       // Which frame we started from in the audio
    paused_at_frame: Option<u64>,          // Frame position when paused
    sample_rate: f32,
    channels: u32,
    frame_count: u32,
    end_frame: Option<u64>,                 // Optional end frame for range playback
}

impl WebPlayer {
    pub fn new() -> Result<Self, JsValue> {
        let context = AudioContext::new()?;
        Ok(WebPlayer { context })
    }
}

impl Drop for WebPlayer {
    fn drop(&mut self) {
        let _ = self.context.close();
    }
}

impl Drop for WebPlayback {
    fn drop(&mut self) {
        // Try to stop if still playing
        if let Some(ref source) = self.source {
            #[allow(deprecated)]
            let _ = source.stop();
        }
    }
}

impl WebPlayer {
    fn create_and_start_source(
        &mut self,
        buffer: &AudioBuffer,
        start_frame: u64,
        end_frame: Option<u64>,
        sample_rate: f32,
    ) -> Result<AudioBufferSourceNode, PlayerError> {
        let source = self
            .context
            .create_buffer_source()
            .map_err(|e| PlayerError {
                message: format!("Failed to create buffer source: {:?}", e),
            })?;

        source.set_buffer(Some(buffer));
        source
            .connect_with_audio_node(&self.context.destination())
            .map_err(|e| PlayerError {
                message: format!("Failed to connect to destination: {:?}", e),
            })?;

        let start_time = start_frame as f64 / sample_rate as f64;

        if let Some(end) = end_frame {
            let duration = (end - start_frame) as f64 / sample_rate as f64;
            source
                .start_with_when_and_grain_offset_and_grain_duration(0.0, start_time, duration)
                .map_err(|e| PlayerError {
                    message: format!("Failed to start playback: {:?}", e),
                })?;
        } else {
            source
                .start_with_when_and_grain_offset(0.0, start_time)
                .map_err(|e| PlayerError {
                    message: format!("Failed to start playback: {:?}", e),
                })?;
        }

        Ok(source)
    }

    fn play_internal(
        &mut self,
        sound: &mut WebSound,
        start_frame: u64,
        end_frame: Option<u64>,
        _listener: Option<Box<dyn PlaybackListener>>,
    ) -> Result<WebPlayback, PlayerError> {
        let source = self.create_and_start_source(
            &sound.buffer,
            start_frame,
            end_frame,
            sound.sample_rate,
        )?;

        Ok(WebPlayback {
            source: Some(source),
            buffer: sound.buffer.clone(),
            start_time: self.context.current_time(),
            start_frame,
            paused_at_frame: None,
            sample_rate: sound.sample_rate,
            channels: sound.channels,
            frame_count: sound.frame_count,
            end_frame,
        })
    }
}

#[async_trait(?Send)]
impl Player for WebPlayer {
    type Sound = WebSound;
    type Playback = WebPlayback;
    type PlaybackListener = Box<dyn PlaybackListener>;

    fn init(&mut self) -> Result<(), PlayerError> {
        Ok(())
    }

    async fn load(&mut self, source: &str) -> Result<Self::Sound, PlayerError> {
        let request = Request::new_with_str(source).map_err(|e| PlayerError {
            message: format!("Failed to create request: {:?}", e),
        })?;
        let window = web_sys::window().ok_or_else(|| PlayerError {
            message: "No window object available".to_string(),
        })?;
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| PlayerError {
                message: format!("Failed to fetch '{}': {:?}", source, e),
            })?;
        let response: Response = response.dyn_into().map_err(|e| PlayerError {
            message: format!("Invalid response type: {:?}", e),
        })?;
        let array_buffer = JsFuture::from(
            response.array_buffer().map_err(|e| PlayerError {
                message: format!("Failed to get array buffer: {:?}", e),
            })?
        )
        .await
        .map_err(|e| PlayerError {
            message: format!("Failed to read array buffer: {:?}", e),
        })?;

        let array_buffer = js_sys::ArrayBuffer::from(array_buffer);
        let decode_promise = self.context.decode_audio_data(&array_buffer)
            .map_err(|e| PlayerError {
                message: format!("Failed to decode audio data: {:?}", e),
            })?;
        let audio_buffer = JsFuture::from(decode_promise)
            .await
            .map_err(|e| PlayerError {
                message: format!("Failed to decode audio: {:?}", e),
            })?;

        let audio_buffer: AudioBuffer = audio_buffer.dyn_into().map_err(|e| PlayerError {
            message: format!("Invalid audio buffer type: {:?}", e),
        })?;
        let sample_rate = audio_buffer.sample_rate();
        let channels = audio_buffer.number_of_channels();
        let frame_count = audio_buffer.length();

        Ok(WebSound {
            buffer: audio_buffer,
            sample_rate,
            channels,
            frame_count,
        })
    }

    fn play_from(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        listener: Option<Self::PlaybackListener>,
    ) -> Result<Self::Playback, PlayerError> {
        self.play_internal(sound, start_frame, None, listener)
    }

    fn play_range(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        end_frame: u64,
        listener: Option<Self::PlaybackListener>,
    ) -> Result<Self::Playback, PlayerError> {
        self.play_internal(sound, start_frame, Some(end_frame), listener)
    }

    fn pause(&mut self, playback: &mut Self::Playback) -> Result<u64, PlayerError> {
        if let Some(ref source) = playback.source {
            let current_time = self.context.current_time();
            let elapsed_seconds = current_time - playback.start_time;
            let elapsed_frames = (elapsed_seconds * playback.sample_rate as f64) as u64;
            let current_frame = playback.start_frame + elapsed_frames;
            #[allow(deprecated)]
            source.stop().map_err(|e| PlayerError {
                message: format!("Failed to stop playback: {:?}", e),
            })?;
            playback.source = None;
            playback.paused_at_frame = Some(current_frame);
            Ok(current_frame)
        } else {
            Ok(playback.paused_at_frame.unwrap_or(playback.start_frame))
        }
    }

    fn get_metadata(&mut self, sound: &mut Self::Sound) -> Result<Metadata, PlayerError> {
        Ok(Metadata {
            sample_rate: sound.sample_rate as u32,
            channel_count: sound.channels,
            frame_count: sound.frame_count as u64,
        })
    }

    fn get_state(&mut self, playback: &mut Self::Playback) -> Result<PlaybackState, PlayerError> {
        // Check if we have an active source node
        if playback.source.is_some() {
            Ok(PlaybackState::Playing)
        } else {
            Ok(PlaybackState::NotPlaying)
        }
    }
}
