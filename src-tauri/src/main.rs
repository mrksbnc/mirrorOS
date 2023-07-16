// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod modules;

fn main() {
    let context: tauri::Context<tauri::utils::assets::EmbeddedAssets> = tauri::generate_context!();
    println!("* Application context generated successfully");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            modules::email::command::get_unseen,
        ])
        .run(context)
        .expect("error while running tauri application");
}
