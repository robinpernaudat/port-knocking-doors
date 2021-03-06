//! Knockers are client who knock.

use crate::{config, data, knock, workflow};
use log::debug;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

static mut MAIN_KNOCKERS: Option<Knockers> = None;

/**
 * Defining a knocker.
 */
#[derive(Debug)]
pub struct Knocker {
    pub next_step: usize,
    pub last_knock: Instant,
    pub error: bool,
}

/**
 * This store the list of recent knockers.
 */
pub struct Knockers {
    list: HashMap<IpAddr, Knocker>,
    sequence: Vec<u16>,
}

impl Knockers {
    pub fn init() {
        let ks = data::knock_seq();
        debug!("knock_seq for knockers : {:?}", ks);
        unsafe {
            MAIN_KNOCKERS = Some(Knockers {
                list: HashMap::new(),
                sequence: ks,
            });
        }
    }

    pub fn event(&mut self, k: knock::Knock) {
        let sequence_size: usize = self.sequence.len();
        let ignoring_time_after_error: Duration = Duration::from_secs(unsafe {
            config::CONFIGURATION
                .clone()
                .unwrap()
                .ignoring_period_after_knock_error
        });
        assert!(sequence_size > 0);
        match self.list.get_mut(&k.ip) {
            Some(existing_knocker) => {
                if existing_knocker.error {
                    let time_from_last_time: Duration =
                        Instant::now() - existing_knocker.last_knock;
                    if time_from_last_time < ignoring_time_after_error {
                        debug!(
                            "This peer was banned, it have to wait for {} seconds from now !",
                            ignoring_time_after_error.as_secs()
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
                    debug!("This knocker {:?} have finished the sequence.", k);
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
                debug!("new knocker : {}", k.ip);
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
        debug!("cleanup des knockers");
        let mut to_be_deleted: Vec<IpAddr> = Vec::new();
        let max_knocker_live_time = Duration::from_secs(unsafe {
            config::CONFIGURATION.clone().unwrap().max_knocker_live_time
        });
        for (ip, knocker) in &self.list {
            let duration_since_last_knock = Instant::now() - knocker.last_knock;
            if duration_since_last_knock > max_knocker_live_time {
                to_be_deleted.push(ip.clone());
            }
        }
        for ip_to_delete in to_be_deleted {
            self.list.remove(&ip_to_delete);
        }
    }
}

pub fn event(k: knock::Knock) {
    unsafe {
        MAIN_KNOCKERS.as_mut().unwrap().event(k);
    }
}

pub fn clean_up() {
    unsafe {
        MAIN_KNOCKERS.as_mut().unwrap().clean_up();
    }
}
