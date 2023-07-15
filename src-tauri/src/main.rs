// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod globals;
mod modules;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::fetch_emails,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
