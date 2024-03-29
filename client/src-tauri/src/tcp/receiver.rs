use ringbuf::{Consumer, HeapRb, Producer, SharedRb};
use std::io::{Read, Write};
use std::mem::MaybeUninit;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::Arc;

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::tcp::{emitter, parser};

use crate::settings::SETTINGS_LOCK;

use super::parser::concat_u8_to_u32;

/* Constants */
const RINGBUFFER_SIZE: usize = 4096;
const RINGBUFFER_READ_SCHEDULE: u64 = 10;
const MESSAGE_WAIT_TIME: u64 = 500;
const VERIFICATION_CODE: [u8; 6] = [b'T', b'T', b'i', b'n', b'i', b't'];
const VER_BUF: [u8; 3] = [b'V', b'E', b'R'];
const ITS_BUF: [u8; 3] = [b'I', b'T', b'S'];
const ACK_BUF: [u8; 3] = [b'A', b'C', b'K'];
const ERR_BUF: [u8; 3] = [b'E', b'R', b'R'];
const ATTEMPTS: u8 = 5;

pub fn start_listener(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let ip: IpAddr;
        let port: u16;

        if let Ok(settings) = SETTINGS_LOCK.read() {
            ip = IpAddr::from(settings.tt_log_ip);
            port = settings.tt_log_port;
        } else {
            emitter::internal_error!(&app, "RwLock failed!");

            /* Leave thread */
            return;
        }

        loop {
            /* Listen until successful connection */
            match TcpStream::connect_timeout(&SocketAddr::new(ip, port), Duration::from_secs(2)) {
                Ok(stream) => {
                    /* Verification */
                    if !verify_connection(&stream, 0) {
                        /* Restart connection on fail */
                        emitter::internal_error!(&app, "Verification failed!");
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("Shutdown failed");
                        continue;
                    }

                    /* Base timestamp */
                    let timestamp = get_initial_timestamp(&stream, 0);
                    if timestamp == 0 {
                        /* Restart connection on fail */
                        emitter::internal_error!(&app, "Initial timestamp invalid!");
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("Shutdown failed");
                        continue;
                    }

                    let ringbuff = HeapRb::<u8>::new(RINGBUFFER_SIZE);
                    let (prod, cons) = ringbuff.split();
                    let (_tx, rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();

                    /* New thread */
                    update_frontend(&app, cons, rx, timestamp);

                    /* Blocks this thread */
                    handle_connection(&stream, prod);
                }
                Err(error) => match error.kind() {
                    std::io::ErrorKind::TimedOut => {}
                    error => {
                        emitter::internal_error!(&app, &format!("Unhandled error: {}", error));
                    }
                },
            }
        }
    });
}

fn verify_connection(mut stream: &TcpStream, tries: u8) -> bool {
    match stream.write_all(&VER_BUF) {
        Ok(_) => (),
        Err(_) => {
            if tries < ATTEMPTS {
                return verify_connection(stream, tries + 1);
            }
            let _ = stream.write_all(&ERR_BUF);
            return false;
        }
    }
    std::thread::sleep(Duration::from_millis(MESSAGE_WAIT_TIME));
    let mut buf: [u8; 6] = [0; 6];
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            if buf == VERIFICATION_CODE {
                let _ = stream.write_all(&ACK_BUF);
                return true;
            }
        }
        Err(_) => (),
    }

    if tries < ATTEMPTS {
        return verify_connection(stream, tries + 1);
    }
    let _ = stream.write_all(&ERR_BUF);
    false
}

fn get_initial_timestamp(mut stream: &TcpStream, tries: u8) -> u128 {
    match stream.write_all(&ITS_BUF) {
        Ok(_) => (),
        Err(_) => {
            if tries < ATTEMPTS {
                return get_initial_timestamp(stream, tries + 1);
            }
            let _ = stream.write_all(&ERR_BUF);
            return 0;
        }
    }
    std::thread::sleep(Duration::from_millis(MESSAGE_WAIT_TIME));
    let mut buf: [u8; 16] = [0; 16];

    /* TODO: Do some timestamp checking */
    match stream.read_exact(&mut buf) {
        Ok(_) => match parser::concat_u8_to_u128(&buf) {
            Ok(value) => {
                let _ = stream.write_all(&ACK_BUF);
                value
            }
            Err(_) => {
                let _ = stream.write_all(&ERR_BUF);
                0
            }
        },
        Err(_) => {
            if tries < ATTEMPTS {
                return get_initial_timestamp(stream, tries + 1);
            }
            let _ = stream.write_all(&ERR_BUF);
            0
        }
    }
}

fn handle_connection(
    mut stream: &TcpStream,
    mut prod: Producer<u8, Arc<SharedRb<u8, Vec<MaybeUninit<u8>>>>>,
) {
    let mut temp_buf: [u8; 1024] = [0; 1024];

    'outer: loop {
        match stream.read(&mut temp_buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    /* Connection closed */
                    break 'outer;
                }

                /* TODO: Overflow protection */
                let _ = prod.write_all(&temp_buf[..bytes_read]);
            }
            Err(_) => {
                /* TODO: Oftentimes recoverable, implement handling for non-recoverables */
            }
        };
    }
}

fn update_frontend(
    original_app: &tauri::AppHandle,
    mut cons: Consumer<u8, Arc<SharedRb<u8, Vec<MaybeUninit<u8>>>>>,
    rx: mpsc::Receiver<()>,
    initial_timestamp: u128,
) {
    let app: tauri::AppHandle = original_app.clone();
    std::thread::spawn(move || {
        let mut header: [u8; 9] = [0; 9];
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            /* Schedule: RINGBUFFER_READ_SCHEDULE */
            std::thread::sleep(std::time::Duration::from_millis(RINGBUFFER_READ_SCHEDULE));
            if cons.is_empty() {
                continue;
            }

            if let Ok(_) = cons.read_exact(&mut header) {
                /*
                    The payload is as follows:
                     - time elapsed in ms (u32) (4 * u8)
                     - level (u8) (1 * u8)
                     - module (u16) (2 * u16)
                     - message_type (u16) (2 * u16)
                     - (optional) message_length (in case of message_type: dynamic)
                     - message (dynamic)

                    Base/Header size: 9 bytes
                    Full size: 9 bytes + message (including message_length)
                */

                let time_elapsed = concat_u8_to_u32(&header[..4]).unwrap_or_else(|_| 0);
                let payload: emitter::Payload = emitter::Payload {
                    timestamp: initial_timestamp + time_elapsed as u128,
                    level: parser::char_to_level(&header[4]),
                    module: parser::module_name(&header[5..7]),
                    message: "Hello World",
                };

                emitter::log(&app, payload);
            };
        }
    });
}
