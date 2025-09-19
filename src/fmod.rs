use crate::ffi::fmod_sys;
use crate::player::{Player, PlayerError};

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

    fn init(&mut self) -> Result<(), PlayerError> {
        unsafe {
            self.system = ptr::null_mut();
            let result = fmod_sys::FMOD_System_Create(&mut self.system, fmod_sys::FMOD_VERSION);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to create FMOD system: {}", result),
                });
            }
            let result = fmod_sys::FMOD_System_Init(self.system, 512, fmod_sys::FMOD_INIT_NORMAL, ptr::null_mut());
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
                fmod_sys::FMOD_DEFAULT,
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

    fn is_playing(&mut self, playback: &mut Self::Playback) -> Result<bool, PlayerError> {
        unsafe {
            let mut is_playing: i32 = 1;
            let result = fmod_sys::FMOD_Channel_IsPlaying(playback.ptr, &mut is_playing);
            if result != fmod_sys::FMOD_RESULT_FMOD_OK {
                return Err(PlayerError {
                    message: format!("Failed to check playback status: {}", result),
                });
            }
            Ok(is_playing != 0)
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
}
