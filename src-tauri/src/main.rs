// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod cool_data;
mod state;
mod task_data;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api::serialize_cool_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
