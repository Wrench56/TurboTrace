use super::emitter;

pub fn start_listener(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        loop {
            /* TODO: Implement actual TCP connection */
            emitter::new_log(&app);
        }
    });
}