use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use driftwave_core::{PlaybackListener, PlaybackState, Player, PlayerError};
use std::path::Path;
use std::sync::{Arc, Mutex};
use symphonia::core::audio::AudioBufferRef;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct CpalPlayer {
    host: cpal::Host,
    device: Option<cpal::Device>,
}

pub struct CpalSound {
    samples: Vec<f32>,
    sample_rate: u32,
    channels: u16,
}

// Sound data is just plain data, safe to send between threads
unsafe impl Send for CpalSound {}
unsafe impl Sync for CpalSound {}

pub struct CpalPlayback {
    stream: cpal::Stream,
    state: Arc<Mutex<PlaybackState>>,
}

// CPAL streams are thread-safe
unsafe impl Send for CpalPlayback {}
unsafe impl Sync for CpalPlayback {}

impl Default for CpalPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl CpalPlayer {
    pub fn new() -> Self {
        CpalPlayer {
            host: cpal::default_host(),
            device: None,
        }
    }
}

impl Player for CpalPlayer {
    type Sound = CpalSound;
    type Playback = CpalPlayback;
    type PlaybackListener = Box<dyn PlaybackListener>;

    fn init(&mut self) -> Result<(), PlayerError> {
        self.device = Some(
            self.host
                .default_output_device()
                .ok_or_else(|| PlayerError {
                    message: "No output device available".to_string(),
                })?,
        );
        Ok(())
    }

    fn load(&mut self, path: &Path) -> Result<Self::Sound, PlayerError> {
        // Open the file
        let file = std::fs::File::open(path).map_err(|e| PlayerError {
            message: format!("Failed to open file: {}", e),
        })?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // Probe the media source
        let hint = Hint::new();
        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let probe = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| PlayerError {
                message: format!("Failed to probe file: {}", e),
            })?;

        let mut format = probe.format;

        // Find the first audio track
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| PlayerError {
                message: "No audio tracks found".to_string(),
            })?;

        // Create a decoder
        let dec_opts = DecoderOptions::default();
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .map_err(|e| PlayerError {
                message: format!("Failed to create decoder: {}", e),
            })?;

        let track_id = track.id;

        // Collect all samples
        let mut samples = Vec::new();
        let mut sample_rate = 0;
        let mut channels = 0;

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(_)) => break,
                Err(Error::ResetRequired) => continue,
                Err(e) => {
                    return Err(PlayerError {
                        message: format!("Error reading packet: {}", e),
                    });
                }
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    // Get audio parameters
                    if sample_rate == 0 {
                        sample_rate = audio_buf.spec().rate;
                        channels = audio_buf.spec().channels.count() as u16;
                    }

                    // Convert to f32 samples (interleaved for CPAL)
                    match audio_buf {
                        AudioBufferRef::F32(buf) => {
                            let planes = buf.planes();
                            if planes.planes().len() == 2 {
                                // Stereo - interleave channels
                                let left = planes.planes()[0];
                                let right = planes.planes()[1];
                                for i in 0..left.len() {
                                    samples.push(left[i]);
                                    samples.push(right[i]);
                                }
                            } else if planes.planes().len() == 1 {
                                // Mono
                                samples.extend_from_slice(planes.planes()[0]);
                            } else {
                                // Multi-channel - interleave all channels
                                let num_frames = planes.planes()[0].len();
                                for frame in 0..num_frames {
                                    for plane in planes.planes() {
                                        samples.push(plane[frame]);
                                    }
                                }
                            }
                        }
                        AudioBufferRef::S16(buf) => {
                            let planes = buf.planes();
                            if planes.planes().len() == 2 {
                                // Stereo - interleave channels
                                let left = planes.planes()[0];
                                let right = planes.planes()[1];
                                for i in 0..left.len() {
                                    samples.push(left[i] as f32 / 32768.0);
                                    samples.push(right[i] as f32 / 32768.0);
                                }
                            } else if planes.planes().len() == 1 {
                                // Mono
                                for &sample in planes.planes()[0].iter() {
                                    samples.push(sample as f32 / 32768.0);
                                }
                            } else {
                                // Multi-channel - interleave all channels
                                let num_frames = planes.planes()[0].len();
                                for frame in 0..num_frames {
                                    for plane in planes.planes() {
                                        samples.push(plane[frame] as f32 / 32768.0);
                                    }
                                }
                            }
                        }
                        _ => {
                            // Convert other formats to f32
                            let mut buf = symphonia::core::audio::AudioBuffer::new(
                                audio_buf.frames() as u64,
                                *audio_buf.spec(),
                            );
                            audio_buf.convert(&mut buf);
                            let planes = buf.planes();
                            if planes.planes().len() == 2 {
                                // Stereo - interleave channels
                                let left = planes.planes()[0];
                                let right = planes.planes()[1];
                                for i in 0..left.len() {
                                    samples.push(left[i]);
                                    samples.push(right[i]);
                                }
                            } else if planes.planes().len() == 1 {
                                // Mono
                                samples.extend_from_slice(planes.planes()[0]);
                            } else {
                                // Multi-channel - interleave all channels
                                let num_frames = planes.planes()[0].len();
                                for frame in 0..num_frames {
                                    for plane in planes.planes() {
                                        samples.push(plane[frame]);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(Error::DecodeError(_)) => continue,
                Err(e) => {
                    return Err(PlayerError {
                        message: format!("Decode error: {}", e),
                    });
                }
            }
        }

        Ok(CpalSound {
            samples,
            sample_rate,
            channels,
        })
    }

    fn play_from(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        listener: Option<Self::PlaybackListener>,
    ) -> Result<Self::Playback, PlayerError> {
        let device = self.device.as_ref().ok_or_else(|| PlayerError {
            message: "Device not initialized".to_string(),
        })?;

        // Try to use the audio file's sample rate
        let supported_configs = device.supported_output_configs().map_err(|e| PlayerError {
            message: format!("Failed to get supported configs: {}", e),
        })?;

        // Find a config that matches our audio file's sample rate and channels
        let mut config = None;
        for supported_config in supported_configs {
            if supported_config.channels() == sound.channels
                && supported_config.min_sample_rate().0 <= sound.sample_rate
                && supported_config.max_sample_rate().0 >= sound.sample_rate
            {
                config = Some(
                    supported_config
                        .with_sample_rate(cpal::SampleRate(sound.sample_rate))
                        .config(),
                );
                break;
            }
        }

        // Fall back to default config if no exact match
        let config = match config {
            Some(c) => c,
            None => {
                let default_config = device.default_output_config().map_err(|e| PlayerError {
                    message: format!("Failed to get default config: {}", e),
                })?;
                default_config.config()
            }
        };

        let sample_rate = config.sample_rate.0;
        let channels = config.channels;

        // Create a shared state for playback control
        let state = Arc::new(Mutex::new(PlaybackState::Playing));
        let state_clone = state.clone();

        // Calculate starting position in samples
        let start_sample = (start_frame * sound.channels as u64) as usize;
        let samples = sound.samples.clone();
        let position = Arc::new(Mutex::new(start_sample));
        let position_clone = position.clone();

        // Handle listener callbacks
        let listener = Arc::new(Mutex::new(listener));
        let listener_clone = listener.clone();

        // Create the audio stream
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut pos = position_clone.lock().unwrap();
                    let state = state_clone.lock().unwrap();

                    if *state != PlaybackState::Playing {
                        data.fill(0.0);
                        return;
                    }

                    for sample in data.iter_mut() {
                        if *pos < samples.len() {
                            *sample = samples[*pos];
                            *pos += 1;

                            // Call listener periodically (every 1024 samples)
                            if (*pos).is_multiple_of(1024)
                                && let Some(ref mut listener) = *listener_clone.lock().unwrap()
                            {
                                let frame = *pos / channels as usize;
                                listener.on_progress(frame as u64);
                            }
                        } else {
                            *sample = 0.0;
                            // Playback finished
                            if let Ok(mut s) = state_clone.try_lock() {
                                *s = PlaybackState::Stopped;
                            }
                        }
                    }
                },
                move |err| eprintln!("Stream error: {}", err),
                None,
            )
            .map_err(|e| PlayerError {
                message: format!("Failed to build output stream: {}", e),
            })?;

        stream.play().map_err(|e| PlayerError {
            message: format!("Failed to play stream: {}", e),
        })?;

        Ok(CpalPlayback { stream, state })
    }

    fn play_range(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        end_frame: u64,
    ) -> Result<Self::Playback, PlayerError> {
        let device = self.device.as_ref().ok_or_else(|| PlayerError {
            message: "Device not initialized".to_string(),
        })?;

        // Try to use the audio file's sample rate
        let supported_configs = device.supported_output_configs().map_err(|e| PlayerError {
            message: format!("Failed to get supported configs: {}", e),
        })?;

        // Find a config that matches our audio file's sample rate and channels
        let mut config = None;
        for supported_config in supported_configs {
            if supported_config.channels() == sound.channels
                && supported_config.min_sample_rate().0 <= sound.sample_rate
                && supported_config.max_sample_rate().0 >= sound.sample_rate
            {
                config = Some(
                    supported_config
                        .with_sample_rate(cpal::SampleRate(sound.sample_rate))
                        .config(),
                );
                break;
            }
        }

        // Fall back to default config if no exact match
        let config = match config {
            Some(c) => c,
            None => {
                let default_config = device.default_output_config().map_err(|e| PlayerError {
                    message: format!("Failed to get default config: {}", e),
                })?;
                default_config.config()
            }
        };

        let channels = config.channels;

        // Create a shared state for playback control
        let state = Arc::new(Mutex::new(PlaybackState::Playing));
        let state_clone = state.clone();

        // Calculate starting and ending positions in samples
        let start_sample = (start_frame * sound.channels as u64) as usize;
        let end_sample = (end_frame * sound.channels as u64) as usize;
        let samples = sound.samples.clone();
        let position = Arc::new(Mutex::new(start_sample));
        let position_clone = position.clone();

        // Create the audio stream
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut pos = position_clone.lock().unwrap();
                    let state = state_clone.lock().unwrap();

                    if *state != PlaybackState::Playing {
                        data.fill(0.0);
                        return;
                    }

                    for sample in data.iter_mut() {
                        if *pos < samples.len() && *pos < end_sample {
                            *sample = samples[*pos];
                            *pos += 1;
                        } else {
                            *sample = 0.0;
                            // Playback finished
                            if let Ok(mut s) = state_clone.try_lock() {
                                *s = PlaybackState::Stopped;
                            }
                        }
                    }
                },
                move |err| eprintln!("Stream error: {}", err),
                None,
            )
            .map_err(|e| PlayerError {
                message: format!("Failed to build output stream: {}", e),
            })?;

        stream.play().map_err(|e| PlayerError {
            message: format!("Failed to play stream: {}", e),
        })?;

        Ok(CpalPlayback { stream, state })
    }

    fn pause(&mut self, playback: &mut Self::Playback) -> Result<Self::Playback, PlayerError> {
        playback.stream.pause().map_err(|e| PlayerError {
            message: format!("Failed to pause stream: {}", e),
        })?;

        *playback.state.lock().unwrap() = PlaybackState::Paused;

        Err(PlayerError {
            message: "Pause/resume not fully implemented yet".to_string(),
        })
    }

    fn get_state(&mut self, playback: &mut Self::Playback) -> Result<PlaybackState, PlayerError> {
        Ok(*playback.state.lock().unwrap())
    }

    fn close(&mut self) -> Result<(), PlayerError> {
        self.device = None;
        Ok(())
    }
}
