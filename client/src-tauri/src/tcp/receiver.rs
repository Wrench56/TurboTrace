use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

use crate::tcp::emitter;

static IP: [u8; 4] = [172, 22, 11, 2];
static PORT: u16 = 1000;

pub fn start_listener(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        loop {
            if let Ok(stream) = TcpStream::connect_timeout(&SocketAddr::from((IP, PORT)), Duration::from_secs(2)) {
                /* Parse and send logs */
            } else {
                emitter::internal_error!(&app, "TCP connection failed!");
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}