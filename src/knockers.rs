//! Knockers are client who knock.

use crate::{knock, data};
use log::{debug};
use std::net::IpAddr;
use std::time::{Instant, Duration};
use std::collections::HashMap;

const IGNORING_TIME_AFTER_ERROR: Duration = Duration::from_secs(5);
pub const MAX_KNOCKER_LIVE_TIME: Duration = Duration::from_secs(30);

pub struct Knocker{
    pub next_step: usize,
    pub last_knock: Instant,
    pub error: bool,
}

pub struct Knockers{
    list: HashMap<IpAddr, Knocker>,
    sequence: Vec<u16>,
}

impl Knockers{
    pub fn new()->Knockers{
        Knockers{list: HashMap::new(), sequence: data::knock_seq()}
    }
    pub fn event(&mut self, k: knock::Knock){
        let sequence_size: usize = self.list.len();
        match self.list.get_mut(&k.ip){
            Some(existing_knocker)=>{
                if existing_knocker.error{
                    let time_from_last_time: Duration = Instant::now() - existing_knocker.last_knock;
                    if time_from_last_time < IGNORING_TIME_AFTER_ERROR{
                        debug!("this peer was banned");
                        existing_knocker.last_knock = Instant::now();
                        return;
                    }
                }
                if existing_knocker.next_step >= sequence_size{
                    debug!("sequence restarted for this peer.");
                    existing_knocker.next_step = 0;
                }
                let aspected_port = self.sequence[existing_knocker.next_step];
                existing_knocker.last_knock = k.when;
                if aspected_port == k.port {
                    existing_knocker.next_step += 1;
                    existing_knocker.error = false;
                }else{
                    debug!("This IP [{}] try a bad sequence", &k.ip);
                    existing_knocker.error = true;
                    existing_knocker.next_step=0;
                }
                todo!("anti brutforce things")
            },
            None => {
                let new_knocker = Knocker{
                    next_step: 0,
                    error: false,
                    last_knock: k.when,
                };
                self.list.insert(k.ip, new_knocker);
                self.event(k);
            },
        }
    }

    /// knocker garbage collection
    /// 
    /// after a duration of MAX_KNOCKER_LIVE_TIME since the last knock, a knocker is removed from the list
    pub fn clean_up(&mut self){
        todo!();
    }
}