use crate::ffi::fmod_sys;
use crate::player::{PlaybackState, Player, PlayerError};

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
    type PlaybackListener = ();

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
                fmod_sys::FMOD_INIT_NORMAL,
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

    fn play(&mut self, sound: &mut FmodSound) -> Result<FmodPlayback, PlayerError> {
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
            Ok(FmodPlayback { ptr: channel })
        }
    }

    fn play_range(
        &mut self,
        _sound: &mut Self::Sound,
        _start_frame: u64,
        _end_frame: u64,
    ) -> Result<Self::Playback, PlayerError> {
        Err(PlayerError {
            message: "play_range not implemented".to_string(),
        })
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

    fn add_playback_listener(&mut self) -> Result<(), PlayerError> {
        Err(PlayerError {
            message: "add_playback_listener not implemented".to_string(),
        })
    }

    fn remove_playback_listener(&mut self) -> Result<(), PlayerError> {
        Err(PlayerError {
            message: "remove_playback_listener not implemented".to_string(),
        })
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
}
