//! In this Module we implement the concept of knock.

use std::net::IpAddr;
use std::time::Instant;

pub struct Knock {
    pub ip: IpAddr,
    pub port: u16,
    pub when: Instant,
}

impl Knock {
    pub fn new(ip: IpAddr, port: u16) -> Knock {
        Knock { ip, port, when: std::time::Instant::now()}
    }
}

