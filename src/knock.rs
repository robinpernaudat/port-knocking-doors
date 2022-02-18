//! In this Module we implement the concept of knock.
//!
//!

use std::net::IpAddr;

pub struct Knock {
    pub ip: IpAddr,
    pub port: u16,
}

impl Knock {
    pub fn new(ip: IpAddr, port: u16) -> Knock {
        Knock { ip, port }
    }
}
