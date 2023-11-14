// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{GlobalWindowEvent, Manager, WindowEvent};

use cool::result::CoolResult;
use cool::{info, init_backtrace};

use crate::event::EventLoop;
use crate::server::start_server;

mod api;
mod cool_data;
mod error;
mod event;
mod server;
mod task_data;

#[tokio::main]
async fn main() -> CoolResult<()> {
    init_backtrace();
    let (shutdown, server_handle) = start_server();
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            std::thread::spawn(move || {
                EventLoop::start_event_loop(main_window);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::serialize_cool_list,
            api::install_cools,
            api::uninstall_cools,
            api::check_cools,
            api::callback_ask_pass,
        ])
        .run(tauri::generate_context!())?;
    info!("waiting for server to shutdown");
    shutdown.send(())?;
    server_handle.await??;
    info!("server shutdown");
    Ok(())
}
