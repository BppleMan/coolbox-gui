// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::Manager;

mod api;
mod cool_data;
mod error;
mod task_data;

#[derive(Clone, Serialize)]
pub struct Payload(String);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::serialize_cool_list,
            api::install_cools,
            api::uninstall_cools,
            api::check_cools,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
