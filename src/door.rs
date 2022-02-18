//! This module manage the knocking doors.
//!
//! This start listening on UDP ports for knocking.
use std::net::{IpAddr, SocketAddr, UdpSocket};

use crate::{data, knock, workflow};
use log::debug;

use std::sync::atomic::{AtomicUsize, Ordering};

static mut THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
static mut SOCKETS_ADDR: Vec<SocketAddr> = Vec::new();

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
}
