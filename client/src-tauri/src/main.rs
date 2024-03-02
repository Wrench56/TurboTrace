// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod settings;
mod tcp;


fn main() {
  tauri::Builder::default()
    .setup(|app| {
      tcp::receiver::start_listener(app.app_handle());
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("Error while running tauri application!");
}
