use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::tcp::emitter;

use crate::settings::SETTINGS_LOCK;

pub fn start_listener(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        loop {
            let ip: IpAddr;
            let port: u16;
        
            if let Ok(settings) = SETTINGS_LOCK.read() {
                ip = IpAddr::from(settings.tt_log_ip);
                port = settings.tt_log_port;
            } else {
                emitter::internal_error!(&app, "RwLock failed!");

                /* Leave thread */
                return;
            }
            
        
            if let Ok(stream) = TcpStream::connect_timeout(
                &SocketAddr::new(ip, port), Duration::from_secs(2)) {
                /* Parse and send logs */
            } else {
                emitter::internal_error!(&app, "TCP connection failed!");
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}