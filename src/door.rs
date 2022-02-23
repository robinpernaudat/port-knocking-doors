//! This module manage the knocking doors.
//!
//! This start listening on UDP ports for knocking.
use crate::{data, firewall, knock, workflow};
use lazy_static::*;
use log::debug;
use mut_static::MutStatic;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

static mut THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
static mut SOCKETS_ADDR: Vec<SocketAddr> = Vec::new();

pub const MAX_OPENED_DURATION: Duration = Duration::from_secs(10);
pub const CLEANUP_PERIODE: Duration = Duration::from_secs(10);

lazy_static! {
    static ref MAIN_DOORS: MutStatic<Doors> = MutStatic::from(Doors::new());
}

pub struct Door {
    pub ip: IpAddr,
    pub opened_instant: Instant,
}

pub struct Doors {
    pub l: HashMap<IpAddr, Door>,
}

impl Doors {
    pub fn new() -> Doors {
        Doors { l: HashMap::new() }
    }
    pub fn cleanup(&mut self) {
        debug!("doors cleanning up");
        let mut idx_of_door_to_be_deleted: Vec<IpAddr> = Vec::new();
        for d in &self.l {
            let duration_since_last_knock = Instant::now() - d.1.opened_instant;
            if duration_since_last_knock > MAX_OPENED_DURATION {
                idx_of_door_to_be_deleted.push(d.1.ip);
            }
        }
        idx_of_door_to_be_deleted.reverse(); //to put the higher index first

        while let Some(ip) = idx_of_door_to_be_deleted.pop() {
            debug!("closing ports for {}", &ip);
            if firewall::close(ip) {
                self.l.remove(&ip);
            }
        }
    }
    pub fn open_the_door(&mut self, ip: IpAddr) {
        let door = Door {
            ip,
            opened_instant: Instant::now(),
        };
        if let Some(d) = self.l.get_mut(&ip) {
            debug!("ports stay opened for {}", &ip);
            d.opened_instant = Instant::now();
        } else {
            debug!("ports are opened for {}", &ip);
            if firewall::open(ip) {
                self.l.insert(ip, door);
            }
        }
    }
}

pub async fn init() {
    debug!("Initializing the door");

    let ports = data::knock_seq();
    for port in ports {
        let socket_address = SocketAddr::from(([0, 0, 0, 0], port));
        unsafe {
            SOCKETS_ADDR.push(socket_address.clone());
        }
        debug!("port {:?} availlable for port sequence", port);
        let socket = UdpSocket::bind(socket_address).expect("can't map port");
        let _ = std::thread::spawn(move || {
            unsafe {
                THREAD_COUNT.fetch_add(1, Ordering::Relaxed);
            };
            let mut buf = [0; 128];
            loop {
                debug!("waitting for message on {}", socket_address);
                let (readed, emetter): (usize, SocketAddr) =
                    socket.recv_from(&mut buf).expect("no data received");
                if readed == 6
                    && buf[0] == b'k'
                    && buf[1] == b'n'
                    && buf[2] == b'o'
                    && buf[3] == b'c'
                    && buf[4] == b'k'
                {
                    debug!("received : {}", readed);
                    let ip: IpAddr = emetter.ip();
                    workflow::knock(knock::Knock::new(ip, port));
                }
            }
        });
        debug!("port configured");
    }
    debug!("door configured");
}

pub fn cleanup() {
    MAIN_DOORS.write().unwrap().cleanup();
}

pub fn open_the_door(ip: IpAddr) {
    MAIN_DOORS.write().unwrap().open_the_door(ip);
}
