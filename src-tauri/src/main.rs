#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod commands;
mod core;
mod ffi;

use commands::AudioState;

fn main() {
    let audio_state = AudioState::new().expect("Failed to initialize audio system");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(audio_state)
        .invoke_handler(tauri::generate_handler![
            commands::play_audio,
            commands::stop_audio,
            commands::log_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
