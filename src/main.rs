mod ffi;
mod fmod;
mod player;

use crate::fmod::FmodPlayer;
use crate::player::Player;
use std::error::Error;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut player = FmodPlayer::new();
    player.init()?;

    let mut sound = player.load(Path::new("assets/אני פורים.wav"))?;
    let mut playback = player.play(&mut sound)?;

    while player.is_playing(&mut playback)? {
        thread::sleep(Duration::from_millis(50));
    }

    player.close()?;
    Ok(())
}
