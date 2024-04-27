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
pub struct Payload {
    pub timestamp: u64,
    pub level: String,
    pub module: String,
    pub message: String,
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
            if app.emit_all("ttlog", payload).is_ok() {
            } else {
                log::error!("Error while sending payload");
            }
        }
        None => {
            log::error!("AppHandle has not yet been initialized");
        }
    }
}

/* Use the internal_error!() macro instead of calling this directly */
#[doc(hidden)]
pub fn __internal_error<T: AsRef<str>>(app: Option<Arc<tauri::AppHandle>>, module: T, message: T) {
    match app {
        Some(app) => {
            match emit_internal(
                &app,
                "PRGME".to_string(),
                module.as_ref().to_string(),
                message.as_ref().to_string(),
            ) {
                Ok(()) => {}
                /* TODO: Handle exceptions */
                Err(_) => log::error!("Failure sending PRGME internal error"),
            }
        }
        None => {
            log::error!("AppHandle has not yet been initialized");
        }
    }
}

fn emit_internal(
    app: &tauri::AppHandle,
    level: String,
    module: String,
    message: String,
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

pub fn get_timestamp() -> u64 {
    SINCE_THE_EPOCH.as_secs() * 1000 + u64::from(SINCE_THE_EPOCH.subsec_nanos()) / 1_000_000
}
