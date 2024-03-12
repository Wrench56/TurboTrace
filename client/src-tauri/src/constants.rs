use const_format::concatcp;

#[cfg(target_os = "windows")]
pub const HOME_DIR: &'static str = "C:/TurboTrail/";
#[cfg(target_os = "unix")]
pub const HOME_DIR: &str = "/opt/TurboTrail/";

pub const SETTINGS_PATH: &'static str = concatcp!(HOME_DIR, "settings/");
pub const LOGGING_PATH: &'static str = concatcp!(HOME_DIR, "logs/");
pub const SETTINGS_JSON: &'static str = concatcp!(SETTINGS_PATH, "settings.json");
pub const LOGGING_FILE: &'static str = concatcp!(LOGGING_PATH, "log.txt");
