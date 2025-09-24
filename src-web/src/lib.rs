mod player;

use player::WebPlayer;
use driftwave_core::Player;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;

#[wasm_bindgen]
pub struct Driftwave {
    player: WebPlayer,
    current_sound: Option<player::WebSound>,
    current_playback: Option<player::WebPlayback>,
}

#[wasm_bindgen]
impl Driftwave {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Driftwave, JsValue> {
        console_error_panic_hook::set_once();
        let mut player = WebPlayer::new()?;
        player.init().map_err(|e| JsValue::from_str(&e.message))?;

        Ok(Driftwave {
            player,
            current_sound: None,
            current_playback: None,
        })
    }

    pub fn load(&mut self, url: String) -> Promise {
        let mut player = WebPlayer::new().unwrap();

        future_to_promise(async move {
            let sound = player.load(&url).await
                .map_err(|e| JsValue::from_str(&e.message))?;

            Ok(JsValue::from_str("loaded"))
        })
    }

    pub async fn load_async(&mut self, url: String) -> Result<(), JsValue> {
        let sound = self.player.load(&url).await
            .map_err(|e| JsValue::from_str(&e.message))?;
        self.current_sound = Some(sound);
        Ok(())
    }

    pub fn play(&mut self) -> Result<(), JsValue> {
        if let Some(ref mut sound) = self.current_sound {
            let playback = self.player.play_from(sound, 0, None)
                .map_err(|e| JsValue::from_str(&e.message))?;
            self.current_playback = Some(playback);
        }
        Ok(())
    }

    pub fn play_from(&mut self, start_frame: u32) -> Result<(), JsValue> {
        if let Some(ref mut sound) = self.current_sound {
            let playback = self.player.play_from(sound, start_frame as u64, None)
                .map_err(|e| JsValue::from_str(&e.message))?;
            self.current_playback = Some(playback);
        }
        Ok(())
    }

    pub fn play_range(&mut self, start_frame: u32, end_frame: u32) -> Result<(), JsValue> {
        if let Some(ref mut sound) = self.current_sound {
            let playback = self.player.play_range(sound, start_frame as u64, end_frame as u64, None)
                .map_err(|e| JsValue::from_str(&e.message))?;
            self.current_playback = Some(playback);
        }
        Ok(())
    }

    pub fn pause(&mut self) -> Result<u32, JsValue> {
        if let Some(ref mut playback) = self.current_playback {
            let frame = self.player.pause(playback)
                .map_err(|e| JsValue::from_str(&e.message))?;
            Ok(frame as u32)
        } else {
            Ok(0)
        }
    }

    pub fn is_playing(&mut self) -> Result<bool, JsValue> {
        if let Some(ref mut playback) = self.current_playback {
            self.player.is_playing(playback)
                .map_err(|e| JsValue::from_str(&e.message))
        } else {
            Ok(false)
        }
    }

    pub fn get_metadata(&mut self) -> Result<JsValue, JsValue> {
        if let Some(ref mut sound) = self.current_sound {
            let metadata = self.player.get_metadata(sound)
                .map_err(|e| JsValue::from_str(&e.message))?;

            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"sampleRate".into(), &metadata.sample_rate.into())?;
            js_sys::Reflect::set(&obj, &"channelCount".into(), &metadata.channel_count.into())?;
            js_sys::Reflect::set(&obj, &"frameCount".into(), &(metadata.frame_count as f64).into())?;

            Ok(obj.into())
        } else {
            Err(JsValue::from_str("No sound loaded"))
        }
    }
}
