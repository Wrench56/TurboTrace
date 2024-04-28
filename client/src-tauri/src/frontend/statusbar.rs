use tauri::{AppHandle, Manager};

use crate::globals;
use core::time;
use std::sync::Arc;
use sysinfo::System;

#[derive(Clone, serde::Serialize)]
struct SysStatusPayload {
    mem_usage: f32,
    cpu_usage: f32,
}

#[derive(Clone, serde::Serialize)]
struct NetStatusPayload {
    connected: bool,
}

pub fn update_connection_status(connected: bool) {
    if let Some(app) = globals::get_app_handle() {
        if app
            .emit_all("net_stat", NetStatusPayload { connected })
            .is_err()
        {
            log::error!("Couldn't emit \"net_stat\" event");
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
        sys.refresh_cpu();
        sys.refresh_memory();

        if app
            .emit_all(
                "sys_stat",
                SysStatusPayload {
                    mem_usage: ((sys.used_memory() as f64) / (sys.total_memory() as f64) * 100.0)
                        as f32,
                    cpu_usage: sys.global_cpu_info().cpu_usage(),
                },
            )
            .is_err()
        {
            log::error!("Couldn't emit \"sys_stat\" event");
        }

        std::thread::sleep(time::Duration::from_secs(5));
    });
}
