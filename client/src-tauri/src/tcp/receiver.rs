use ringbuf::{Consumer, HeapRb, Producer, SharedRb};
use std::io::{Read, Write};
use std::mem::MaybeUninit;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::Arc;

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::frontend::emitter;
use crate::globals;
use crate::utils::concats;

use crate::settings::SETTINGS_LOCK;
use log;

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

pub fn start_listener() {
    std::thread::spawn(move || {
        let ip: IpAddr;
        let port: u16;

        if let Ok(settings) = SETTINGS_LOCK.read() {
            ip = IpAddr::from(settings.tt_log_ip);
            port = settings.tt_log_port;
        } else {
            emitter::internal_error!("RwLock failed!");

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
                        log::error!("Verification failed");
                        emitter::internal_error!("Verification failed!");
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("Shutdown failed");
                        continue;
                    }

                    /* Base timestamp */
                    let timestamp = get_initial_timestamp(&stream, 0);
                    if timestamp == 0 {
                        /* Restart connection on fail */
                        log::error!("Initial timestamp invalid!");
                        emitter::internal_error!("Initial timestamp invalid!");
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .expect("Shutdown failed");
                        continue;
                    }

                    log::info!("Starting log listener loop");
                    let ringbuff = HeapRb::<u8>::new(RINGBUFFER_SIZE);
                    let (prod, cons) = ringbuff.split();
                    let (_tx, rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();

                    /* New thread */
                    update_frontend(cons, rx, timestamp);

                    /* Blocks this thread */
                    handle_connection(&stream, prod);
                }
                Err(error) => match error.kind() {
                    std::io::ErrorKind::TimedOut => {}
                    error => {
                        log::error!("{}", &format!("Unhandled error: {}", error));
                        emitter::internal_error!(&format!("Unhandled error: {}", error));
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

fn get_initial_timestamp(mut stream: &TcpStream, tries: u8) -> u64 {
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
    let mut buf: [u8; 14] = [0; 14];
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            /* DeltaTimestamp is 0 */
            if &buf[..2] != [0, 0] {
                log::error!("DeltaTimestamp is not zero");
                let _ = stream.write_all(&ERR_BUF);
                ()
            }
            /* ID is ITS (1) */
            if concats::concat_u8_to_u32(&buf[2..=5]).unwrap_or_else(|_| 0) != 1 {
                log::error!("Log ID is not 1 for initial timestamp message");
                let _ = stream.write_all(&ERR_BUF);
                ()
            }

            match concats::concat_u8_to_u64(&buf[6..]) {
                Ok(value) => {
                    let _ = stream.write_all(&ACK_BUF);
                    value
                }
                Err(_) => {
                    log::error!("Initial timestamp concat failed");
                    let _ = stream.write_all(&ERR_BUF);
                    0
                }
            }
        }
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
    mut cons: Consumer<u8, Arc<SharedRb<u8, Vec<MaybeUninit<u8>>>>>,
    rx: mpsc::Receiver<()>,
    initial_timestamp: u64,
) {
    let app = globals::get_app_handle();
    std::thread::spawn(move || {
        let mut header: [u8; 6] = [0; 6];
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
                     - time elapsed in ms (u16) (2 * u8)
                     - message_type (u32) (4 * u16)
                     - (optional) message_length (in case of message_type: dynamic)
                     - message (dynamic)

                    Base/Header size: 6 bytes
                    Full size: 6 bytes + message (including message_length)
                */

                let time_elapsed = concats::concat_u8_to_u16(&header[0], &header[1]);
                let payload: emitter::Payload = emitter::Payload {
                    timestamp: initial_timestamp + u64::from(time_elapsed),
                    level: "DEBUG",
                    module: "std::default",
                    message: "Hello World",
                };

                emitter::log(&app, payload);
            };
        }
    });
}
