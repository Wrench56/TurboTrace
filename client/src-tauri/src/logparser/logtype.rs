use std::{collections::HashMap, fs::File, io::Read};

use crate::logparser::chunk::ChunkType;
use crate::logparser::formatter;

use crate::constants::CURRENT_LOGTYPES_JSON;
use crate::frontend::emitter::Payload;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LogType {
    template: String,
    level: u8,
    module: String,
    arguments: Vec<String>,
}

pub fn load_logtypes() -> Result<HashMap<u32, LogType>, Box<dyn std::error::Error>> {
    let mut file = File::open(CURRENT_LOGTYPES_JSON)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let logtypes: HashMap<u32, LogType> = serde_json::from_str(&data)?;
    Ok(logtypes)
}

pub fn get_logtype(id: u32, logtypes: &HashMap<u32, LogType>) -> Option<&LogType> {
    logtypes.get(&id)
}

impl LogType {
    pub fn decode_from_data(&self, timestamp: u64, data: Vec<Vec<u8>>) -> Payload {
        Payload {
            timestamp,
            level: LogType::decode_level(&self.level).to_string(),
            module: self.module.clone(),
            message: formatter::format_template_string(&self.template, &self.arguments, data),
        }
    }

    fn decode_level(level: &u8) -> &str {
        match level {
            0 => "DEBUG",
            1 => "INFO",
            2 => "WARN",
            3 => "ERROR",
            4 => "CRIT",
            _ => "PRGME",
        }
    }

    pub fn get_next_arg_chunk(&self, arg_num: usize) -> ChunkType {
        match self.arguments.get(arg_num) {
            Some(chunk_type) => self.get_chunk_size(chunk_type),
            None => ChunkType::End,
        }
    }

    fn get_chunk_size(&self, chunk_type: &str) -> ChunkType {
        match chunk_type {
            "bool" => ChunkType::KnownSizeChunk(1),
            "short" => ChunkType::KnownSizeChunk(2),
            "int" => ChunkType::KnownSizeChunk(4),
            "long" => ChunkType::KnownSizeChunk(8),
            "dyn" => ChunkType::SizeChunk,
            _ => ChunkType::UnknownChunk,
        }
    }
}
