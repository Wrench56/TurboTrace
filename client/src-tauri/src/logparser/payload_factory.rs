use std::{collections::HashMap, io::Read};

use ringbuf::Consumer;

use crate::{frontend::emitter, logparser::logtype, tcp::receiver::TCPRingbuffer, utils::concats};

use super::{chunk::ChunkType, logtype::LogType};

pub struct PayloadFactory {
    logtypes: HashMap<u32, logtype::LogType>,
    curr_timestamp: u64,
}

impl PayloadFactory {
    pub fn new(initial_timestamp: u64) -> PayloadFactory {
        let logtypes = logtype::load_logtypes().unwrap_or_default();
        Self {
            curr_timestamp: initial_timestamp,
            logtypes,
        }
    }

    pub fn create_payload(
        &mut self,
        cons: &mut Consumer<u8, TCPRingbuffer>,
        header: &[u8; 6],
    ) -> emitter::Payload {
        /*
            The payload is as follows:
            - time elapsed in ms (u16) (2 * u8)
            - message_type (u32) (4 * u16)
            - (optional) message_length (in case of message_type: dynamic)
            - message (dynamic)

            Base/Header size: 6 bytes
            Full size: 6 bytes + message (including message_length)
        */

        /* Update curr_timestamp */
        self.curr_timestamp += u64::from(concats::concat_u8_to_u16(&header[..2]).unwrap_or(0));
        match logtype::get_logtype(
            concats::concat_u8_to_u32(&header[2..=5]).unwrap_or(0),
            &self.logtypes,
        ) {
            Some(logtype) => logtype
                .decode_from_data(self.curr_timestamp, self.construct_data_vec(cons, logtype)),
            None => {
                let err_msg: String = format!(
                    "Logtype with ID \"{}\" doesn't exist",
                    concats::concat_u8_to_u32(&header[2..=5]).unwrap_or(0)
                );
                log::error!("{}", err_msg);
                emitter::Payload {
                    timestamp: emitter::get_timestamp(),
                    level: "PRGME".to_string(),
                    module: "logparser::payload_factory".to_string(),
                    message: err_msg,
                }
            }
        }
    }

    fn construct_data_vec(
        &self,
        cons: &mut Consumer<u8, TCPRingbuffer>,
        logtype: &LogType,
    ) -> Vec<Vec<u8>> {
        let mut args: Vec<Vec<u8>> = Vec::new();
        let mut arg_num: usize = 0;

        loop {
            match logtype.get_next_arg_chunk(arg_num) {
                ChunkType::SizeChunk => {
                    let mut u8_size_buff: Vec<u8> = vec![0; 1];

                    if cons.read_exact(&mut u8_size_buff).is_err() {
                        log::error!("Error while reading u8 size");
                        emitter::internal_error!("Error while reading u8 size");
                        break;
                    }

                    /* Max size for dynamic argument: u16 */
                    let size: usize = if u8_size_buff[0] == 0 {
                        /* Size defined by the next u16 */
                        let mut u16_size_buff: Vec<u8> = vec![0; 2];
                        if cons.read_exact(&mut u16_size_buff).is_err() {
                            log::error!("Error while reading u16 size");
                            emitter::internal_error!("Error while reading u16 size");
                            break;
                        }
                        usize::from(concats::concat_u8_to_u16(&u16_size_buff[..2]).unwrap_or(0))
                    } else {
                        usize::from(u8_size_buff[0])
                    };

                    self.add_arg(cons, &mut args, size);
                }
                ChunkType::KnownSizeChunk(size) => {
                    self.add_arg(cons, &mut args, usize::from(size));
                }
                ChunkType::UnknownChunk => {
                    log::error!("Unknown chunk detected while parsing incoming LogType");
                    emitter::internal_error!(
                        "Unknown chunk detected while parsing incoming LogType"
                    );
                }
                ChunkType::End => break,
            }

            arg_num += 1;
        }

        args
    }

    fn add_arg(
        &self,
        cons: &mut Consumer<u8, TCPRingbuffer>,
        args: &mut Vec<Vec<u8>>,
        size: usize,
    ) {
        let mut arg_buffer: Vec<u8> = vec![0; size];
        if cons.read_exact(&mut arg_buffer).is_err() {
            log::error!("Can't read argument from ringbuffer");
        }
        args.push(arg_buffer);
    }
}
