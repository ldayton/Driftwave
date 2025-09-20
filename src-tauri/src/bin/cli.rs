#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use driftwave::core::fmod::FmodPlayer;
use driftwave::core::player::Player;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Driftwave CLI - Audio Player");

    // Initialize FMOD player
    let mut player = FmodPlayer::new();

    println!("Initializing FMOD...");
    if let Err(e) = player.init() {
        eprintln!("Failed to initialize FMOD: {}", e);
        return;
    }
    println!("FMOD initialized successfully");

    // Load the audio file
    let audio_path = Path::new("../assets/אני פורים.wav");
    println!("Loading audio file: {:?}", audio_path);

    let mut sound = match player.load(audio_path) {
        Ok(s) => {
            println!("Audio file loaded successfully");
            s
        }
        Err(e) => {
            eprintln!("Failed to load audio file: {}", e);
            return;
        }
    };

    // Play the sound
    println!("Playing audio...");
    let mut playback = match player.play(&mut sound, None) {
        Ok(p) => {
            println!("Audio started playing");
            p
        }
        Err(e) => {
            eprintln!("Failed to play audio: {}", e);
            return;
        }
    };

    // Keep the program running while audio plays
    println!("Press Ctrl+C to stop...");
    loop {
        match player.is_playing(&mut playback) {
            Ok(true) => {
                thread::sleep(Duration::from_millis(100));
            }
            Ok(false) => {
                println!("Audio finished playing");
                break;
            }
            Err(e) => {
                eprintln!("Error checking playback status: {}", e);
                break;
            }
        }
    }

    // Clean up
    if let Err(e) = player.close() {
        eprintln!("Error closing FMOD: {}", e);
    }

    println!("Done!");
}
