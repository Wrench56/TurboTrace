use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct Payload<'a> {
  timestamp: u32,
  level: &'a str,
  module: &'a str,
  message: &'a str
}

pub fn new_log(app: &tauri::AppHandle) {
  std::thread::sleep(std::time::Duration::from_secs(1));
  match app.emit_all("ttlog", Payload {
    timestamp: 0,
    level: "DEBUG",
    module: "src.lib.backend",
    message: "Hello World from Rust!"
  }) {
    Ok(_) => {},
    /* TODO: Handle exceptions */
    Err(_) => panic!("Error while sending payload"),
  }
}