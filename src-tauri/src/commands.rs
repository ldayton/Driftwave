use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

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
pub fn play_audio() -> Result<String, String> {
    // Log the call
    let _ = log_to_file(format!("[BACKEND] play_audio command called"));

    // Use macOS's built-in afplay command as a temporary solution
    let audio_path = "../assets/אני פורים.wav";

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
