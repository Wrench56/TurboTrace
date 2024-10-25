// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
use storage::exceptions::InitErrors;
use storage::init::create_home;
use storage::reader::home_dir_exists;
use tauri::Manager;

mod constants;
mod frontend;
mod globals;
mod logparser;
mod settings;
mod storage;
mod tcp;
mod utils;

fn main() {
    if !home_dir_exists() {
        match create_home() {
            Err(InitErrors::HomeDir) => panic!("Couldn't create home directory"),
            Err(InitErrors::SettingsDir) => panic!("Couldn't create settings directory"),
            Err(InitErrors::LoggingDir) => panic!("Couldn't create logging directory"),
            Err(InitErrors::LogtypesDir) => panic!("Couldn't create logtypes directory"),
            Err(InitErrors::SettingsJson) => {}
            Ok(()) => {}
        }
    }
    /* Initialize logger */
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} - [{}] {} - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.target(),
                message
            ));
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(constants::LOGGING_FILE).expect("Failed to create log file!"))
        .apply()
        .expect("Failed to initialize logger!");

    log::info!("Starting tauri...");
    tauri::Builder::default()
        .setup(|app| {
            globals::set_app_handle(app.app_handle().clone());
            tcp::receiver::start_listener();
            frontend::statusbar::watch_system();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application!");
}
