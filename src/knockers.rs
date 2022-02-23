//! Knockers are client who knock.

use crate::{data, knock, workflow};
use lazy_static::*;
use log::debug;
use mut_static::MutStatic;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

const IGNORING_TIME_AFTER_ERROR: Duration = Duration::from_secs(5);
pub const MAX_KNOCKER_LIVE_TIME: Duration = Duration::from_secs(30);

lazy_static! {
    static ref MAIN_KNOCKERS: MutStatic<Knockers> = MutStatic::from(Knockers::new());
}

pub struct Knocker {
    pub next_step: usize,
    pub last_knock: Instant,
    pub error: bool,
}

pub struct Knockers {
    list: HashMap<IpAddr, Knocker>,
    sequence: Vec<u16>,
}

impl Knockers {
    pub fn new() -> Knockers {
        Knockers {
            list: HashMap::new(),
            sequence: data::knock_seq(),
        }
    }

    pub fn event(&mut self, k: knock::Knock) {
        let sequence_size: usize = self.list.len();
        match self.list.get_mut(&k.ip) {
            Some(existing_knocker) => {
                if existing_knocker.error {
                    let time_from_last_time: Duration =
                        Instant::now() - existing_knocker.last_knock;
                    if time_from_last_time < IGNORING_TIME_AFTER_ERROR {
                        debug!(
                            "This peer was banned, it have to wait for {} seconds from now !",
                            IGNORING_TIME_AFTER_ERROR.as_secs()
                        );
                        existing_knocker.last_knock = Instant::now();
                        return;
                    }
                }
                if existing_knocker.next_step >= sequence_size {
                    debug!("sequence restarted for this peer.");
                    existing_knocker.next_step = 0;
                }
                let aspected_port = self.sequence[existing_knocker.next_step];
                existing_knocker.last_knock = k.when;
                if aspected_port == k.port {
                    existing_knocker.next_step += 1;
                    existing_knocker.error = false;
                    if existing_knocker.next_step == sequence_size {
                        self.open_port_for(k.ip);
                    }
                } else {
                    debug!("This IP [{}] try a bad sequence", &k.ip);
                    existing_knocker.error = true;
                    existing_knocker.next_step = 0;
                }
            }
            None => {
                let new_knocker = Knocker {
                    next_step: 0,
                    error: false,
                    last_knock: k.when,
                };
                self.list.insert(k.ip, new_knocker);
                self.event(k);
            }
        }
    }

    fn open_port_for(&self, ip: IpAddr) {
        workflow::open_the_door(ip);
    }

    /// knocker garbage collection
    ///
    /// after a duration of MAX_KNOCKER_LIVE_TIME since the last knock, a knocker is removed from the list
    pub fn clean_up(&mut self) {
        let mut to_be_deleted: Vec<IpAddr> = Vec::new();
        for (ip, knocker) in &self.list {
            let duration_since_last_knock = Instant::now() - knocker.last_knock;
            if duration_since_last_knock > MAX_KNOCKER_LIVE_TIME {
                to_be_deleted.push(ip.clone());
            }
        }
        for ip_to_delete in to_be_deleted {
            self.list.remove(&ip_to_delete);
        }
    }
}

pub fn event(k: knock::Knock) {
    MAIN_KNOCKERS.write().unwrap().event(k);
}

pub fn clean_up() {
    MAIN_KNOCKERS.write().unwrap().clean_up();
}
