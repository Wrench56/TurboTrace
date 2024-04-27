use std::fs;

use crate::constants;

use super::exceptions::InitErrors;

pub fn create_home() -> Result<(), InitErrors> {
    fs::create_dir(constants::HOME_DIR).map_err(|_| InitErrors::HomeDir)?;
    fs::create_dir(constants::SETTINGS_PATH).map_err(|_| InitErrors::SettingsDir)?;
    fs::create_dir(constants::LOGGING_PATH).map_err(|_| InitErrors::LoggingDir)?;
    fs::create_dir(constants::LOGTYPES_PATH).map_err(|_| InitErrors::LogtypesDir)?;
    fs::File::create(constants::SETTINGS_JSON).map_err(|_| InitErrors::SettingsJson)?;

    Ok(())
}
