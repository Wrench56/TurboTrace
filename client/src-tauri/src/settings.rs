use std::sync::RwLock;
use once_cell::sync::Lazy;

pub(crate) static SETTINGS_LOCK: Lazy<RwLock<Settings>> = Lazy::new(|| RwLock::new(Settings::load()));

pub struct Settings {
    pub tt_log_ip: [u8; 4],
    pub tt_log_port: u16
}

trait Savable {
    fn load() -> Self;
    fn save(&self) -> Result<(), ()>;
    fn default() -> Self;
}

impl Savable for Settings {
    fn load() -> Settings {
        /* TODO: Implement load */
        Self::default()
    }

    fn save(&self) -> Result<(), ()> {
        /* TODO: Implement save */
        todo!()
    }

    fn default() -> Settings {
        Settings {
            tt_log_ip: [172, 22, 11, 2],
            tt_log_port: 1000
        }
    }
}