use crate::dsp;
use crate::ffi::fmod_sys;
use crate::player::{PlaybackListener, PlaybackState, Player, PlayerError};

use std::ffi::CString;
use std::path::Path;
use std::ptr;

pub struct FmodPlayer {
    system: *mut fmod_sys::FMOD_SYSTEM,
}

impl FmodPlayer {
    pub fn new() -> Self {
        FmodPlayer {
            system: ptr::null_mut(),
        }
    }
}

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

    fn load(&mut self, path: &Path) -> Result<FmodSound, PlayerError> {
        unsafe {
            let mut sound: *mut fmod_sys::FMOD_SOUND = ptr::null_mut();
            let filename = match path.to_str() {
                Some(s) => CString::new(s).map_err(|_| PlayerError {
                    message: "Path contains null byte".to_string(),
                })?,
                None => {
                    return Err(PlayerError {
                        message: "Invalid path encoding".to_string(),
                    })
                }
            };
            let result = fmod_sys::FMOD_System_CreateSound(
                self.system,
                filename.as_ptr(),
                fmod_sys::FMOD_DEFAULT | fmod_sys::FMOD_ACCURATETIME,
                ptr::null_mut(),
                &mut sound,
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to load sound: {}", result),
                });
            }
            Ok(FmodSound { ptr: sound })
        }
    }

    fn play(
        &mut self,
        sound: &mut FmodSound,
        listener: Option<Self::PlaybackListener>,
    ) -> Result<FmodPlayback, PlayerError> {
        unsafe {
            let mut channel: *mut fmod_sys::FMOD_CHANNEL = ptr::null_mut();
            let result = fmod_sys::FMOD_System_PlaySound(
                self.system,
                sound.ptr,
                ptr::null_mut(),
                0, // not paused
                &mut channel,
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to play sound: {}", result),
                });
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

            Ok(FmodPlayback {
                ptr: channel,
                dsp,
                callback_data,
            })
        }
    }

    fn play_range(
        &mut self,
        sound: &mut Self::Sound,
        start_frame: u64,
        end_frame: u64,
    ) -> Result<Self::Playback, PlayerError> {
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
                    message: format!("Failed to set start position: {}", result),
                });
            }
            let mut parent_clock: u64 = 0;
            let result =
                fmod_sys::FMOD_Channel_GetDSPClock(channel, ptr::null_mut(), &mut parent_clock);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to get DSP clock: {}", result),
                });
            }
            let duration_frames = end_frame - start_frame;
            let stop_clock = parent_clock + duration_frames;
            let result = fmod_sys::FMOD_Channel_SetDelay(
                channel, 0, // start immediately
                stop_clock, 1, // stopchannels
            );
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to set delay: {}", result),
                });
            }
            let result = fmod_sys::FMOD_Channel_SetPaused(channel, 0);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to unpause channel: {}", result),
                });
            }
            Ok(FmodPlayback {
                ptr: channel,
                dsp: ptr::null_mut(),
                callback_data: ptr::null_mut(),
            })
        }
    }

    fn pause(&mut self, _playback: &mut Self::Playback) -> Result<Self::Playback, PlayerError> {
        Err(PlayerError {
            message: "pause not implemented".to_string(),
        })
    }

    fn resume(&mut self, _playback: &mut Self::Playback) -> Result<Self::Playback, PlayerError> {
        Err(PlayerError {
            message: "resume not implemented".to_string(),
        })
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

            if is_playing == 0 {
                return Ok(PlaybackState::Stopped);
            }

            let mut is_paused: i32 = 0;
            let result = fmod_sys::FMOD_Channel_GetPaused(playback.ptr, &mut is_paused);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to get paused state: {}", result),
                });
            }

            if is_paused != 0 {
                Ok(PlaybackState::Paused)
            } else {
                Ok(PlaybackState::Playing)
            }
        }
    }

    fn close(&mut self) -> Result<(), PlayerError> {
        unsafe {
            let result = fmod_sys::FMOD_System_Release(self.system);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to close FMOD system: {}", result),
                });
            }
            Ok(())
        }
    }
}

pub struct FmodSound {
    ptr: *mut fmod_sys::FMOD_SOUND,
}

pub struct FmodPlayback {
    ptr: *mut fmod_sys::FMOD_CHANNEL,
    dsp: *mut fmod_sys::FMOD_DSP,
    callback_data: *mut dsp::DspCallbackData,
}

impl Drop for FmodPlayback {
    fn drop(&mut self) {
        unsafe {
            if !self.dsp.is_null() {
                fmod_sys::FMOD_DSP_Release(self.dsp);
            }
            if !self.callback_data.is_null() {
                drop(Box::from_raw(self.callback_data));
            }
        }
    }
}
