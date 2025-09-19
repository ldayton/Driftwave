mod ffi {
    pub mod fmod_sys;
}
use ffi::fmod_sys;

use std::ffi::CString;
use std::ptr;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Create FMOD system
        let mut system: *mut fmod_sys::FMOD_SYSTEM = ptr::null_mut();
        let result = fmod_sys::FMOD_System_Create(&mut system, fmod_sys::FMOD_VERSION as u32);
        if result != fmod_sys::FMOD_RESULT_FMOD_OK {
            panic!("Failed to create FMOD system: {}", result);
        }

        // Initialize FMOD system
        let result = fmod_sys::FMOD_System_Init(system, 512, fmod_sys::FMOD_INIT_NORMAL, ptr::null_mut());
        if result != fmod_sys::FMOD_RESULT_FMOD_OK {
            panic!("Failed to initialize FMOD system: {}", result);
        }

        // Load sound file
        let mut sound: *mut fmod_sys::FMOD_SOUND = ptr::null_mut();
        let filename = CString::new("assets/אני פורים.wav")?;
        let result = fmod_sys::FMOD_System_CreateSound(
            system,
            filename.as_ptr(),
            fmod_sys::FMOD_DEFAULT,
            ptr::null_mut(),
            &mut sound,
        );
        if result != fmod_sys::FMOD_RESULT_FMOD_OK {
            panic!("Failed to load sound: {}", result);
        }

        // Play the sound
        let mut channel: *mut fmod_sys::FMOD_CHANNEL = ptr::null_mut();
        let result = fmod_sys::FMOD_System_PlaySound(
            system,
            sound,
            ptr::null_mut(),
            0, // not paused
            &mut channel,
        );
        if result != fmod_sys::FMOD_RESULT_FMOD_OK {
            panic!("Failed to play sound: {}", result);
        }

        // Wait for playback to finish
        let mut is_playing: i32 = 1;
        while is_playing != 0 {
            fmod_sys::FMOD_Channel_IsPlaying(channel, &mut is_playing);
            fmod_sys::FMOD_System_Update(system);
            thread::sleep(Duration::from_millis(10));
        }

        // Clean up
        fmod_sys::FMOD_Sound_Release(sound);
        fmod_sys::FMOD_System_Release(system);
    }

    Ok(())
}
