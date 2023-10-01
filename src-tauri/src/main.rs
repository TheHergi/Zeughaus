// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod db;
mod commands;
mod models;
mod schema;
mod services;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, commands::talent_cmd::create_talent, commands::talent_cmd::get_talent])
        .setup(|_app| {
            // Initialize the database.
            db::init();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
