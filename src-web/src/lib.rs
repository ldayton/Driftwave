mod player;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AudioBuffer};

#[wasm_bindgen]
pub struct Driftwave {
    context: AudioContext,
    buffer: Option<AudioBuffer>,
}

#[wasm_bindgen]
impl Driftwave {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Driftwave, JsValue> {
        console_error_panic_hook::set_once();
        let context = AudioContext::new()?;
        Ok(Driftwave {
            context,
            buffer: None,
        })
    }

    pub fn set_buffer(&mut self, buffer: AudioBuffer) {
        self.buffer = Some(buffer);
    }

    pub fn decode_audio_data(&self, array_buffer: &js_sys::ArrayBuffer) -> js_sys::Promise {
        self.context.decode_audio_data(array_buffer).unwrap()
    }

    pub fn play(&self) -> Result<(), JsValue> {
        if let Some(buffer) = &self.buffer {
            let source = self.context.create_buffer_source()?;
            source.set_buffer(Some(buffer));
            source.connect_with_audio_node(&self.context.destination())?;
            source.start()?;
        }
        Ok(())
    }
}
