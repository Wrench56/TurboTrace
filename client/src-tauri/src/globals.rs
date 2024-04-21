use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref APP_HANDLE: Mutex<Option<Arc<tauri::AppHandle>>> = Mutex::new(None);
}

pub fn get_app_handle() -> Option<Arc<tauri::AppHandle>> {
    let handle = APP_HANDLE.lock().unwrap();
    handle.clone()
}

pub fn set_app_handle(new_app_handle: tauri::AppHandle) {
    let arc_handle = Arc::new(new_app_handle);
    let mut handle = APP_HANDLE.lock().unwrap();
    *handle = Some(arc_handle);
}
