use std::fs;

use super::exceptions::StorageErrors;

pub fn write_json(json: &str, path: &str) -> Result<(), StorageErrors> {
    fs::write(path, json).map_err(|_| StorageErrors::Filesystem)?;
    Ok(())
}
