use super::player::PlaybackListener;
use crate::ffi::fmod_sys;

use std::ffi::c_void;
use std::ptr;

// DSP callback context to pass listener
pub struct DspCallbackData {
    pub listener: Option<Box<dyn PlaybackListener>>,
    pub channel: *mut fmod_sys::FMOD_CHANNEL,
}

// DSP callback that reports playback progress
pub unsafe extern "C" fn progress_dsp_callback(
    dsp_state: *mut fmod_sys::FMOD_DSP_STATE,
    inbuffer: *mut f32,
    outbuffer: *mut f32,
    length: ::core::ffi::c_uint,
    _inchannels: ::core::ffi::c_int,
    outchannels: *mut ::core::ffi::c_int,
) -> fmod_sys::FMOD_RESULT {
    // Get our callback data from the DSP's user data
    if !dsp_state.is_null() {
        let mut userdata: *mut c_void = ptr::null_mut();
        let dsp_instance = (*dsp_state).instance;

        // Get the DSP instance's userdata
        let result =
            fmod_sys::FMOD_DSP_GetUserData(dsp_instance as *mut fmod_sys::FMOD_DSP, &mut userdata);
        if result == fmod_sys::FMOD_RESULT_FMOD_OK && !userdata.is_null() {
            let data = userdata as *mut DspCallbackData;
            let callback_data = &mut *data;

            // Get channel position in PCM samples
            if !callback_data.channel.is_null() {
                let mut position: ::core::ffi::c_uint = 0;
                let result = fmod_sys::FMOD_Channel_GetPosition(
                    callback_data.channel,
                    &mut position,
                    fmod_sys::FMOD_TIMEUNIT_PCM,
                );

                if result == fmod_sys::FMOD_RESULT_FMOD_OK {
                    if let Some(ref mut listener) = callback_data.listener {
                        listener.on_progress(position as u64);
                    }
                }
            }
        }
    }

    // Pass through audio unchanged (this is just for progress tracking)
    if !inbuffer.is_null() && !outbuffer.is_null() {
        let samples = (length as usize) * (*outchannels as usize);
        ptr::copy_nonoverlapping(inbuffer, outbuffer, samples);
    }

    fmod_sys::FMOD_RESULT_FMOD_OK
}
