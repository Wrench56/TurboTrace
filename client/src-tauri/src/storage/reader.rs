use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::constants::HOME_DIR;

use super::exceptions::StorageErrors;

pub fn load_struct_from_json<T: DeserializeOwned>(path: &str) -> Result<T, StorageErrors> {
    let file = File::open(path).map_err(|_| StorageErrors::Filesystem)?;
    let mut reader = BufReader::new(file);

    let mut json_string = String::new();
    reader
        .read_to_string(&mut json_string)
        .map_err(|_| StorageErrors::LoadJson)?;

    let result: T = serde_json::from_str(&json_string).map_err(|_| StorageErrors::CastToObject)?;
    Ok(result)
}

pub fn home_dir_exists() -> bool {
    let path = Path::new(HOME_DIR);
    path.is_dir()
}
