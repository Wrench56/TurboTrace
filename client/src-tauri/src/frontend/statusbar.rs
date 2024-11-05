use tauri::{AppHandle, Emitter};

use lazy_static::lazy_static;

use crate::globals;
use core::time;
use std::sync::{Arc, Mutex};
use sysinfo::System;

lazy_static! {
    static ref RAW_BYTES_RECV: Mutex<usize> = Mutex::new(0);
}

#[derive(Clone, serde::Serialize)]
struct SysStatusPayload {
    mem_usage: f32,
    cpu_usage: f32,
}

#[derive(Clone, serde::Serialize)]
struct NetStatusPayload {
    connected: bool,
}

#[derive(Clone, serde::Serialize)]
struct RawBytesRecvPayload {
    raw_bytes: usize,
}

pub fn update_connection_status(connected: bool) {
    if let Some(app) = globals::get_app_handle() {
        if app
            .emit("net_stat", NetStatusPayload { connected })
            .is_err()
        {
            log::error!("Couldn't emit \"net_stat\" event");
        }
    } else {
        log::info!("Could not get AppHandle");
    }
}

pub fn update_bytes_recv(bytes_recv: usize) {
    let mut raw_bytes_recv_handle = RAW_BYTES_RECV.lock().unwrap();
    *raw_bytes_recv_handle += bytes_recv;

    if let Some(app) = globals::get_app_handle() {
        if app
            .emit(
                "raw_bytes",
                RawBytesRecvPayload {
                    raw_bytes: *raw_bytes_recv_handle,
                },
            )
            .is_err()
        {
            log::error!("Couldn't emit \"sys_stat\" event");
        }
    } else {
        log::info!("Could not get AppHandle");
    }
}

pub fn watch_system() {
    let mut sys: System = System::new();
    let app: Arc<AppHandle>;
    if let Some(app_) = globals::get_app_handle() {
        app = app_;
    } else {
        log::info!("Could not get AppHandle");
        return;
    }

    if sys.cpus().len() >= f32::MAX as usize {
        log::error!("You have more than 3.40282347*(10^38) CPUs... greetings from the past");
        return;
    }

    std::thread::spawn(move || loop {
        sys.refresh_cpu_all();
        sys.refresh_memory();

        if app
            .emit(
                "sys_stat",
                SysStatusPayload {
                    mem_usage: ((sys.used_memory() as f64) / (sys.total_memory() as f64) * 100.0)
                        as f32,
                    cpu_usage: sys.global_cpu_usage(),
                },
            )
            .is_err()
        {
            log::error!("Couldn't emit \"sys_stat\" event");
        }

        std::thread::sleep(time::Duration::from_secs(5));
    });
}
