// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger::Env;

mod models;
mod modules;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("debug")).init();
    log::info!("mirrorOS initialization started");

    let context: tauri::Context<tauri::utils::assets::EmbeddedAssets> = tauri::generate_context!();
    log::info!("application context generated successfully");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            modules::email::command::get_unseen,
        ])
        .run(context)
        .expect("error while running tauri application");
}
