use std::{fs, io::Write};

use crate::constants;

use super::exceptions::InitErrors;

pub fn create_home() -> Result<(), InitErrors> {
    fs::create_dir(constants::HOME_DIR).map_err(|_| InitErrors::HomeDirError)?;
    fs::create_dir(constants::SETTINGS_PATH).map_err(|_| InitErrors::SettingsDirError)?;
    fs::create_dir(constants::LOGGING_PATH).map_err(|_| InitErrors::LoggingDirError)?;
    let mut file = fs::File::create("foo.txt").map_err(|_| InitErrors::SettingsJsonError)?;
    file.write_all(b"\n")
        .map_err(|_| InitErrors::SettingsJsonError)?;
    Ok(())
}
