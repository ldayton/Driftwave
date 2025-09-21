use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use tauri::{Manager, path::BaseDirectory};

use crate::core::fmod::FmodPlayer;
use crate::core::player::{PlaybackState, Player};

#[tauri::command]
pub fn log_to_file(message: String) -> Result<(), String> {
    // Get the executable directory
    let log_path = if let Ok(current_dir) = env::current_dir() {
        current_dir.join("driftwave.log")
    } else {
        // Fallback to relative path
        std::path::PathBuf::from("driftwave.log")
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("Failed to open log at {:?}: {}", log_path, e))?;

    writeln!(file, "{}", message).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn play_audio(
    app_handle: tauri::AppHandle,
    state: tauri::State<AudioState>,
) -> Result<String, String> {
    // Log the call
    let _ = log_to_file(format!("[BACKEND] play_audio command called"));

    // Get the resource path for the bundled audio file
    let resource_path = app_handle
        .path()
        .resolve("assets/אני פורים.wav", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let _ = log_to_file(format!("[BACKEND] Using audio path: {:?}", resource_path));

    // Load the sound if not already loaded
    let mut player = state.player.lock().map_err(|e| e.to_string())?;
    let mut current_sound = state.current_sound.lock().map_err(|e| e.to_string())?;
    let mut current_playback = state.current_playback.lock().map_err(|e| e.to_string())?;

    // Stop current playback if any
    if let Some(ref mut playback) = *current_playback {
        let _ = player.pause(playback);
    }

    // Load sound if not already loaded or if path changed
    if current_sound.is_none() {
        let sound = player.load(&resource_path).map_err(|e| e.to_string())?;
        *current_sound = Some(sound);
    }

    // Play from the beginning
    if let Some(ref mut sound) = *current_sound {
        let playback = player
            .play_from(sound, 0, None)
            .map_err(|e| e.to_string())?;
        *current_playback = Some(playback);
        Ok("Audio playback started".to_string())
    } else {
        Err("Failed to load sound".to_string())
    }
}

#[tauri::command]
pub fn stop_audio(state: tauri::State<AudioState>) -> Result<String, String> {
    // Log the call
    let _ = log_to_file(format!("[BACKEND] stop_audio command called"));

    let mut player = state.player.lock().map_err(|e| e.to_string())?;
    let mut current_playback = state.current_playback.lock().map_err(|e| e.to_string())?;

    if let Some(ref mut playback) = *current_playback {
        player.pause(playback).map_err(|e| e.to_string())?;
        *current_playback = None;
        Ok("Audio stopped".to_string())
    } else {
        Ok("No audio playing".to_string())
    }
}

// Audio state with FMOD player
pub struct AudioState {
    player: Mutex<FmodPlayer>,
    current_sound: Mutex<Option<<FmodPlayer as Player>::Sound>>,
    current_playback: Mutex<Option<<FmodPlayer as Player>::Playback>>,
}

impl AudioState {
    pub fn new() -> Result<Self, String> {
        let mut player = FmodPlayer::new();
        player.init().map_err(|e| e.to_string())?;

        Ok(Self {
            player: Mutex::new(player),
            current_sound: Mutex::new(None),
            current_playback: Mutex::new(None),
        })
    }
}
