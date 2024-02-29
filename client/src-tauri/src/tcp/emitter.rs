use tauri::Manager;

use std::time::Instant;

#[derive(Clone, serde::Serialize)]
pub struct Payload<'a> {
	pub timestamp: u128,
	pub level: &'a str,
	pub module: &'a str,
	pub message: &'a str
}

macro_rules! internal_error {
    ($app_handle:expr, $message:expr) => {
		use crate::tcp::emitter::__internal_error;
        __internal_error($app_handle, module_path!(), $message);
    };
}

pub(crate) use internal_error;

pub fn log(app: &tauri::AppHandle, payload: Payload) {
	match app.emit_all("ttlog", payload) {
    	Ok(_) => {},
    	/* TODO: Handle exceptions */
    	Err(_) => panic!("Error while sending payload"),
  	}
}

/* Use the internal_error!() macro instead of calling this directly */
#[doc(hidden)]
pub fn __internal_error(app: &tauri::AppHandle, module: &str, message: &str) {
	match emit_internal(app, "PRGME", module, message) {
		Ok(_) => {},
		/* TODO: Handle exceptions */
		Err(_) => panic!("Error while sending PRGME payload!"),
	}
}

fn emit_internal(app: &tauri::AppHandle, level: &str, module: &str, message: &str) -> tauri::Result<()> {
	app.emit_all("ttlog", Payload {
    	timestamp: get_timestamp(),
    	level: level,
    	module: module,
    	message: message
  	})
}

fn get_timestamp() -> u128 {
	Instant::now().elapsed().as_millis()
}