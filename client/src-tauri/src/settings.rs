use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::{
    constants::SETTINGS_JSON,
    storage::{reader::load_struct_from_json, writer::write_json},
};

pub(crate) static SETTINGS_LOCK: Lazy<RwLock<Settings>> =
    Lazy::new(|| RwLock::new(Settings::load()));

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub tt_log_ip: [u8; 4],
    pub tt_log_port: u16,
}

trait Savable {
    fn load() -> Self;
    fn save(&self);
    fn default() -> Self;
}

impl Savable for Settings {
    fn load() -> Settings {
        load_struct_from_json(SETTINGS_JSON).unwrap_or_else(|_| {
            log::warn!("Using default Settings");
            let settings: Settings = Self::default();
            settings.save();
            settings
        })
    }

    fn save(&self) {
        /* TODO: Implement save */
        let _ = write_json(
            serde_json::to_string(&self)
                .as_ref()
                .map_or("Error", String::as_str),
            SETTINGS_JSON,
        )
        .map_err(|_| log::error!("Error writing settings.json"));
    }

    fn default() -> Settings {
        Settings {
            tt_log_ip: [127, 0, 0, 1],
            tt_log_port: 40025,
        }
    }
}
