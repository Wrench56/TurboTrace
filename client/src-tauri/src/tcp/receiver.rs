use ringbuf::{Consumer, HeapRb, Producer, SharedRb};
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::Arc;

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::frontend::statusbar::update_bytes_recv;
use crate::frontend::{emitter, statusbar};
use crate::globals;
use crate::logparser::payload_factory::PayloadFactory;
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

/* Types */
pub type TCPRingbuffer = Arc<SharedRb<u8, Vec<MaybeUninit<u8>>>>;

pub fn start_listener() {
    std::thread::spawn(move || {
        let ip: IpAddr;
        let port: u16;

        if let Ok(settings) = SETTINGS_LOCK.read() {
            ip = IpAddr::from(settings.tt_log_ip);
            port = settings.tt_log_port;
        } else {
            log::error!("RwLock failed!");
            emitter::internal_error!("RwLock failed!");

            /* Leave thread */
            return;
        }

        loop {
            /* Listen until successful connection */
            match TcpStream::connect_timeout(&SocketAddr::new(ip, port), Duration::from_secs(2)) {
                Ok(stream) => {
                    log::info!("Incoming connection");

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
                    statusbar::update_connection_status(true);
                    log::info!("Connection verified");

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
                    let (tx, rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();

                    /* New thread */
                    update_frontend(cons, rx, timestamp);

                    /* Blocks this thread */
                    handle_connection(&stream, prod);

                    /* Close update_frontend thread */
                    drop(tx);
                }
                Err(error) => match error.kind() {
                    std::io::ErrorKind::TimedOut => {}
                    error => {
                        log::error!("{}", &format!("Unhandled error: {error}"));
                        emitter::internal_error!(&format!("Unhandled error: {error}"));
                    }
                },
            }
        }
    });
}

fn verify_connection(mut stream: &TcpStream, tries: u8) -> bool {
    match stream.write_all(&VER_BUF) {
        Ok(()) => (),
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
    if stream.read_exact(&mut buf).is_ok() && buf == VERIFICATION_CODE {
        let _ = stream.write_all(&ACK_BUF);
        return true;
    }

    if tries < ATTEMPTS {
        return verify_connection(stream, tries + 1);
    }
    let _ = stream.write_all(&ERR_BUF);
    false
}

fn get_initial_timestamp(mut stream: &TcpStream, tries: u8) -> u64 {
    match stream.write_all(&ITS_BUF) {
        Ok(()) => (),
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
        Ok(()) => {
            /* DeltaTimestamp is 0 */
            if buf[..2] != [0, 0] {
                log::error!("DeltaTimestamp is not zero");
                let _ = stream.write_all(&ERR_BUF);
            }
            /* ID is ITS (1) */
            if concats::concat_u8_to_u32(&buf[2..=5]).unwrap_or(0) != 1 {
                log::error!("Log ID is not 1 for initial timestamp message");
                let _ = stream.write_all(&ERR_BUF);
            }

            match concats::concat_u8_to_u64(&buf[6..]) {
                Ok(value) => {
                    let _ = stream.write_all(&ACK_BUF);
                    value
                }
                Err(()) => {
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

fn handle_connection(mut stream: &TcpStream, mut prod: Producer<u8, TCPRingbuffer>) {
    let mut temp_buf: [u8; 1024] = [0; 1024];

    'outer: loop {
        match stream.read(&mut temp_buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    /* Connection closed */
                    log::info!("Connection closed");
                    statusbar::update_connection_status(false);
                    break 'outer;
                }

                update_bytes_recv(bytes_read);
                /* TODO: Overflow protection */
                let _ = prod.write_all(&temp_buf[..bytes_read]);
            }
            Err(ref e) if e.kind() == io::ErrorKind::ConnectionAborted => {
                log::info!("Connection aborted");
                statusbar::update_connection_status(false);
                break 'outer;
            }
            Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => {
                log::info!("Connection reset");
                statusbar::update_connection_status(false);
                break 'outer;
            }
            Err(ref _e) => {
                break 'outer;
            }
        };
    }
}

fn update_frontend(
    mut cons: Consumer<u8, TCPRingbuffer>,
    rx: mpsc::Receiver<()>,
    initial_timestamp: u64,
) {
    let mut payload_factory = PayloadFactory::new(initial_timestamp);
    std::thread::spawn(move || {
        let mut header: [u8; 6] = [0; 6];
        loop {
            match rx.try_recv() {
                Ok(()) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            /* Schedule: RINGBUFFER_READ_SCHEDULE */
            std::thread::sleep(std::time::Duration::from_millis(RINGBUFFER_READ_SCHEDULE));
            if cons.is_empty() {
                continue;
            }

            if cons.read_exact(&mut header).is_ok() {
                emitter::log(
                    &globals::get_app_handle(),
                    payload_factory.create_payload(&mut cons, &header),
                );
            };
        }
    });
}
