use tauri::Manager;

use once_cell::sync::Lazy;
use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

static SINCE_THE_EPOCH: Lazy<Duration> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
});

#[derive(Clone, serde::Serialize)]
pub struct Payload<'a> {
    pub timestamp: u64,
    pub level: &'a str,
    pub module: &'a str,
    pub message: &'a str,
}

#[macro_export]
macro_rules! internal_error {
    ($message:expr) => {
        $crate::frontend::emitter::__internal_error(
            $crate::globals::get_app_handle(),
            module_path!(),
            $message,
        );
    };
}

pub(crate) use internal_error;

pub fn log(app: &Option<Arc<tauri::AppHandle>>, payload: Payload) {
    match app {
        Some(app) => {
            match app.emit_all("ttlog", payload) {
                Ok(_) => {}
                /* TODO: Handle exceptions */
                Err(_) => panic!("Error while sending payload"),
            }
        }
        None => {
            log::error!("AppHandle has not yet been initialized");
            return;
        }
    }
}

/* Use the internal_error!() macro instead of calling this directly */
#[doc(hidden)]
pub fn __internal_error(app: Option<Arc<tauri::AppHandle>>, module: &str, message: &str) {
    match app {
        Some(app) => {
            match emit_internal(&app, "PRGME", module, message) {
                Ok(_) => {}
                /* TODO: Handle exceptions */
                Err(_) => log::error!("Failure sending PRGME internal error"),
            }
        }
        None => {
            log::error!("AppHandle has not yet been initialized");
            return;
        }
    }
}

fn emit_internal(
    app: &tauri::AppHandle,
    level: &str,
    module: &str,
    message: &str,
) -> tauri::Result<()> {
    app.emit_all(
        "ttlog",
        Payload {
            timestamp: get_timestamp(),
            level,
            module,
            message,
        },
    )
}

fn get_timestamp() -> u64 {
    SINCE_THE_EPOCH.as_secs() * 1000 + SINCE_THE_EPOCH.subsec_nanos() as u64 / 1_000_000
}
