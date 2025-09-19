mod player;
use player::{FmodPlayer, Player};

use std::io::Error;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let mut player = FmodPlayer {};
    let player_handle = player.init()?;
    let audio_handle = player.load(&player_handle, Path::new("sample.wav"))?;
    let playback_handle = player.play(&player_handle, &audio_handle)?;

    thread::sleep(Duration::from_secs(5));

    player.pause(&player_handle, &playback_handle);
    player.close(&player_handle);
    Ok(())
}
