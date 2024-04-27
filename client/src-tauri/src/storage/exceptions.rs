pub enum StorageErrors {
    Filesystem,
    LoadJson,
    CastToObject,
}

pub enum InitErrors {
    HomeDir,
    SettingsDir,
    LoggingDir,
    LogtypesDir,
    SettingsJson,
}
