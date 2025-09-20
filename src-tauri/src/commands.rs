use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use tauri::{Manager, path::BaseDirectory};

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
pub fn play_audio(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Log the call
    let _ = log_to_file(format!("[BACKEND] play_audio command called"));

    // Get the resource path for the bundled audio file
    let resource_path = app_handle
        .path()
        .resolve("assets/אני פורים.wav", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let audio_path = resource_path
        .to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())?;

    let _ = log_to_file(format!("[BACKEND] Using audio path: {}", audio_path));

    let output = Command::new("afplay").arg(audio_path).spawn();

    match output {
        Ok(_) => Ok("Audio playback started".to_string()),
        Err(e) => Err(format!("Failed to start audio: {}", e)),
    }
}

#[tauri::command]
pub fn stop_audio() -> Result<String, String> {
    // Log the call
    let _ = log_to_file(format!("[BACKEND] stop_audio command called"));

    // For now, we'll just return success
    // Proper stop implementation would need persistent player state
    Ok("Audio stopped".to_string())
}

// Simple state without FMOD
pub struct AudioState;

impl AudioState {
    pub fn new() -> Self {
        Self
    }
}
