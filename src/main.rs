mod dsp;
mod ffi;
mod fmod;
mod player;

use crate::fmod::FmodPlayer;
use crate::player::{PlaybackListener, Player};
use std::error::Error;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct ProgressPrinter;

impl PlaybackListener for ProgressPrinter {
    fn on_progress(&mut self, position_frames: u64) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("Progress: {} frames at {}ms", position_frames, timestamp);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut player = FmodPlayer::new();
    player.init()?;

    let mut sound = player.load(Path::new("assets/אני פורים.wav"))?;
    let listener = Box::new(ProgressPrinter);

    {
        let mut playback = player.play(&mut sound, Some(listener))?;
        while player.is_playing(&mut playback)? {
            thread::sleep(Duration::from_millis(50));
        }
    }

    thread::sleep(Duration::from_millis(500));

    let mut range_playback = player.play_range(&mut sound, 30_000, 53_000)?;
    while player.is_playing(&mut range_playback)? {
        thread::sleep(Duration::from_millis(50));
    }

    player.close()?;
    Ok(())
}
