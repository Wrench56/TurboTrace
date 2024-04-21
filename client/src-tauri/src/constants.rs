use const_format::concatcp;

#[cfg(target_os = "windows")]
pub const HOME_DIR: &'static str = "C:/TurboTrace/";
#[cfg(target_os = "unix")]
pub const HOME_DIR: &str = "/opt/TurboTrace/";

pub const SETTINGS_PATH: &'static str = concatcp!(HOME_DIR, "settings/");
pub const LOGGING_PATH: &'static str = concatcp!(HOME_DIR, "logs/");
pub const LOGTYPES_PATH: &'static str = concatcp!(HOME_DIR, "logtypes/");
pub const SETTINGS_JSON: &'static str = concatcp!(SETTINGS_PATH, "settings.json");
pub const LOGGING_FILE: &'static str = concatcp!(LOGGING_PATH, "client.log");
pub const CURRENT_LOGTYPES_JSON: &'static str = concatcp!(LOGTYPES_PATH, "logtypes.json");
