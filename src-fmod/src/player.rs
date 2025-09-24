#[cfg(target_os = "macos")]
#[link(name = "fmod")]
unsafe extern "C" {}

#[cfg(target_os = "linux")]
#[link(name = "fmod")]
unsafe extern "C" {}

#[cfg(target_os = "windows")]
#[link(name = "fmod_vc")]
unsafe extern "C" {}

use crate::dsp;
use crate::ffi::fmod_sys;
use async_trait::async_trait;
use driftwave_core::{Metadata, PlaybackListener, PlaybackState, Player, PlayerError};

use std::ffi::CString;
use std::ptr;

pub struct FmodPlayer {
    system: *mut fmod_sys::FMOD_SYSTEM,
}

// We're using FMOD_INIT_THREAD_UNSAFE - client must handle synchronization
unsafe impl Send for FmodPlayer {}
unsafe impl Sync for FmodPlayer {}

impl Default for FmodPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl FmodPlayer {
    pub fn new() -> Self {
        FmodPlayer {
            system: ptr::null_mut(),
        }
    }

    fn play_internal(
        &mut self,
        sound: &mut FmodSound,
        start_frame: u64,
        end_frame: Option<u64>,
        listener: Option<Box<dyn PlaybackListener>>,
    ) -> Result<FmodPlayback, PlayerError> {
        unsafe {
            let mut channel: *mut fmod_sys::FMOD_CHANNEL = ptr::null_mut();
            let result = fmod_sys::FMOD_System_PlaySound(
                self.system,
                sound.ptr,
                ptr::null_mut(),
                1, // paused
                &mut channel,
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to play sound: {}", result),
                });
            }

            // Set position
            if start_frame > u32::MAX as u64 {
                return Err(PlayerError {
                    message: format!("Start frame {} exceeds u32 max", start_frame),
                });
            }
            let result = fmod_sys::FMOD_Channel_SetPosition(
                channel,
                start_frame as u32,
                fmod_sys::FMOD_TIMEUNIT_PCM,
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to set position: {}", result),
                });
            }

            // Set end delay if we have an end_frame
            if let Some(end) = end_frame {
                let mut parent_clock: u64 = 0;
                let result =
                    fmod_sys::FMOD_Channel_GetDSPClock(channel, ptr::null_mut(), &mut parent_clock);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    return Err(PlayerError {
                        message: format!("Failed to get DSP clock: {}", result),
                    });
                }
                let duration_frames = end - start_frame;
                let stop_clock = parent_clock.saturating_add(duration_frames);
                let result = fmod_sys::FMOD_Channel_SetDelay(
                    channel, 0, // start immediately
                    stop_clock, 1, // stop channels
                );
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    return Err(PlayerError {
                        message: format!("Failed to set delay: {}", result),
                    });
                }
            }

            // If we have a listener, create and attach a DSP for progress callbacks
            let mut dsp: *mut fmod_sys::FMOD_DSP = ptr::null_mut();
            let mut callback_data: *mut dsp::DspCallbackData = ptr::null_mut();

            if let Some(listener_box) = listener {
                // Create DSP description
                let mut dspdesc: fmod_sys::FMOD_DSP_DESCRIPTION = std::mem::zeroed();
                dspdesc.pluginsdkversion = fmod_sys::FMOD_PLUGIN_SDK_VERSION;
                let name = b"Progress Tracker\0";
                ptr::copy_nonoverlapping(
                    name.as_ptr(),
                    dspdesc.name.as_mut_ptr() as *mut u8,
                    name.len(),
                );
                dspdesc.version = 0x00010000;
                dspdesc.numinputbuffers = 1;
                dspdesc.numoutputbuffers = 1;
                dspdesc.read = Some(dsp::progress_dsp_callback);

                // Create the DSP
                let result = fmod_sys::FMOD_System_CreateDSP(self.system, &dspdesc, &mut dsp);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    return Err(PlayerError {
                        message: format!("Failed to create DSP: {}", result),
                    });
                }

                // Create callback data
                callback_data = Box::into_raw(Box::new(dsp::DspCallbackData {
                    listener: Some(listener_box),
                    channel,
                }));

                // Set user data on the DSP
                let result =
                    fmod_sys::FMOD_DSP_SetUserData(dsp, callback_data as *mut std::ffi::c_void);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    // Clean up
                    drop(Box::from_raw(callback_data));
                    fmod_sys::FMOD_DSP_Release(dsp);
                    return Err(PlayerError {
                        message: format!("Failed to set DSP user data: {}", result),
                    });
                }

                // Add DSP to channel
                let result = fmod_sys::FMOD_Channel_AddDSP(channel, 0, dsp);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    // Clean up
                    drop(Box::from_raw(callback_data));
                    fmod_sys::FMOD_DSP_Release(dsp);
                    return Err(PlayerError {
                        message: format!("Failed to add DSP to channel: {}", result),
                    });
                }
            }

            // Unpause after everything is set up
            let result = fmod_sys::FMOD_Channel_SetPaused(channel, 0);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to unpause: {}", result),
                });
            }

            Ok(FmodPlayback {
                ptr: channel,
                dsp,
                callback_data,
            })
        }
    }
}

#[async_trait(?Send)]
impl Player for FmodPlayer {
    type Sound = FmodSound;
    type Playback = FmodPlayback;
    type PlaybackListener = Box<dyn PlaybackListener>;

    fn init(&mut self) -> Result<(), PlayerError> {
        unsafe {
            self.system = ptr::null_mut();
            let result = fmod_sys::FMOD_System_Create(&mut self.system, fmod_sys::FMOD_VERSION);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to create FMOD system: {}", result),
                });
            }
            let result = fmod_sys::FMOD_System_Init(
                self.system,
                2,
                fmod_sys::FMOD_INIT_NORMAL | fmod_sys::FMOD_INIT_THREAD_UNSAFE,
                ptr::null_mut(),
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to initialize FMOD system: {}", result),
                });
            }
            Ok(())
        }
    }

    async fn load(&mut self, source: &str) -> Result<FmodSound, PlayerError> {
        unsafe {
            let mut sound: *mut fmod_sys::FMOD_SOUND = ptr::null_mut();
            let filename = CString::new(source).map_err(|_| PlayerError {
                message: "Source string contains null byte".to_string(),
            })?;
            // FMOD will interpret this as a file path
            let result = fmod_sys::FMOD_System_CreateSound(
                self.system,
                filename.as_ptr(),
                fmod_sys::FMOD_DEFAULT | fmod_sys::FMOD_ACCURATETIME,
                ptr::null_mut(),
                &mut sound,
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to load sound from '{}': {}", source, result),
                });
            }
            Ok(FmodSound { ptr: sound })
        }
    }

    fn play_from(
        &mut self,
        sound: &mut FmodSound,
        start_frame: u64,
        listener: Option<Self::PlaybackListener>,
    ) -> Result<FmodPlayback, PlayerError> {
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
        unsafe {
            // First pause the channel
            let result = fmod_sys::FMOD_Channel_SetPaused(playback.ptr, 1);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to pause channel: {}", result),
                });
            }

            // Get the current position in PCM frames
            let mut position: u32 = 0;
            let result = fmod_sys::FMOD_Channel_GetPosition(
                playback.ptr,
                &mut position,
                fmod_sys::FMOD_TIMEUNIT_PCM,
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to get channel position: {}", result),
                });
            }

            Ok(position as u64)
        }
    }

    fn get_metadata(&mut self, sound: &mut Self::Sound) -> Result<Metadata, PlayerError> {
        unsafe {
            // Get channel count
            let mut sound_type: fmod_sys::FMOD_SOUND_TYPE = 0;
            let mut format: fmod_sys::FMOD_SOUND_FORMAT = 0;
            let mut channels: i32 = 0;
            let mut bits: i32 = 0;
            let result = fmod_sys::FMOD_Sound_GetFormat(
                sound.ptr,
                &mut sound_type,
                &mut format,
                &mut channels,
                &mut bits,
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to get sound format: {}", result),
                });
            }

            // Get sample rate
            let mut sample_rate: f32 = 0.0;
            let mut priority: i32 = 0;
            let result =
                fmod_sys::FMOD_Sound_GetDefaults(sound.ptr, &mut sample_rate, &mut priority);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to get sound defaults: {}", result),
                });
            }

            // Get length in PCM samples
            let mut length: u32 = 0;
            let result =
                fmod_sys::FMOD_Sound_GetLength(sound.ptr, &mut length, fmod_sys::FMOD_TIMEUNIT_PCM);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to get sound length: {}", result),
                });
            }

            Ok(Metadata {
                sample_rate: sample_rate as u32,
                channel_count: channels as u32,
                frame_count: length as u64,
            })
        }
    }

    fn get_state(&mut self, playback: &mut Self::Playback) -> Result<PlaybackState, PlayerError> {
        unsafe {
            let mut is_playing: i32 = 0;
            let result = fmod_sys::FMOD_Channel_IsPlaying(playback.ptr, &mut is_playing);

            if result == fmod_sys::FMOD_RESULT_FMOD_ERR_INVALID_HANDLE
                || result == fmod_sys::FMOD_RESULT_FMOD_ERR_CHANNEL_STOLEN
            {
                return Ok(PlaybackState::Invalid);
            }

            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to get channel state: {}", result),
                });
            }

            if is_playing != 0 {
                Ok(PlaybackState::Playing)
            } else {
                Ok(PlaybackState::NotPlaying)
            }
        }
    }
}

impl Drop for FmodPlayer {
    fn drop(&mut self) {
        if !self.system.is_null() {
            unsafe {
                let result = fmod_sys::FMOD_System_Release(self.system);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    eprintln!("Failed to release FMOD system: {}", result);
                }
            }
        }
    }
}

pub struct FmodSound {
    ptr: *mut fmod_sys::FMOD_SOUND,
}

// We're using FMOD_INIT_THREAD_UNSAFE - client must handle synchronization
unsafe impl Send for FmodSound {}
unsafe impl Sync for FmodSound {}

impl Drop for FmodSound {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let result = fmod_sys::FMOD_Sound_Release(self.ptr);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    eprintln!("Failed to release FMOD sound: {}", result);
                }
            }
        }
    }
}

pub struct FmodPlayback {
    ptr: *mut fmod_sys::FMOD_CHANNEL,
    dsp: *mut fmod_sys::FMOD_DSP,
    callback_data: *mut dsp::DspCallbackData,
}

// We're using FMOD_INIT_THREAD_UNSAFE - client must handle synchronization
unsafe impl Send for FmodPlayback {}
unsafe impl Sync for FmodPlayback {}

impl Drop for FmodPlayback {
    fn drop(&mut self) {
        unsafe {
            // Stop the channel if it's still valid
            if !self.ptr.is_null() {
                let result = fmod_sys::FMOD_Channel_Stop(self.ptr);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK
                    && result != fmod_sys::FMOD_RESULT_FMOD_ERR_INVALID_HANDLE
                    && result != fmod_sys::FMOD_RESULT_FMOD_ERR_CHANNEL_STOLEN
                {
                    eprintln!("Failed to stop FMOD channel: {}", result);
                }
            }

            if !self.dsp.is_null() {
                let result = fmod_sys::FMOD_DSP_Release(self.dsp);
                if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                    eprintln!("Failed to release FMOD DSP: {}", result);
                }
            }

            if !self.callback_data.is_null() {
                drop(Box::from_raw(self.callback_data));
            }
        }
    }
}
