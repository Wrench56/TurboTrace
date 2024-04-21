use std::fs;

use crate::constants;

use super::exceptions::InitErrors;

pub fn create_home() -> Result<(), InitErrors> {
    fs::create_dir(constants::HOME_DIR).map_err(|_| InitErrors::HomeDirError)?;
    fs::create_dir(constants::SETTINGS_PATH).map_err(|_| InitErrors::SettingsDirError)?;
    fs::create_dir(constants::LOGGING_PATH).map_err(|_| InitErrors::LoggingDirError)?;
    fs::create_dir(constants::LOGTYPES_PATH).map_err(|_| InitErrors::LogtypesDirError)?;
    fs::File::create(constants::SETTINGS_JSON).map_err(|_| InitErrors::SettingsJsonError)?;

    Ok(())
}
