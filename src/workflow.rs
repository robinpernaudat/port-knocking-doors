//! This module manage the main workflow.
//!
//! All the work is defined here.
//!
//! TODO
//! - keep IP list of tryers for 1 minutes
//! - keep in memory IP allowed for 5 minutes
//! - IP for seq failled are added wrotten in log
//! - an IP allowed, reset his timeout if seq redone
//! - conf firewalld, iptables if Linux
//! - conf Windows firewall if windows

use log::{debug, error, info};
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use crossbeam_channel::{unbounded, Receiver, RecvTimeoutError, Sender}; //one sender in this channel
use std::time::{Duration, Instant};

//#[macro_use]
use lazy_static::*;
use mut_static::MutStatic;

use crate::{door, firewall, knock, knockers};

pub enum Msg {
    KNOCK(knock::Knock),
    QUIT,
    CLEANUP,
}

static mut WANT_TO_QUIT: AtomicBool = AtomicBool::new(false);
static mut THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

lazy_static! {
    static ref MAIN_WF: MutStatic<WF> = MutStatic::from(new());
}
pub struct WF {
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
    knockers: knockers::Knockers,
    doors: door::Doors,
}

impl Drop for WF {
    fn drop(&mut self) {
        info!("END of the workflow");
    }
}

fn new()->WF{
    let (s, r) = unbounded();
    WF{
        sender: s,
        receiver: r,
        knockers: knockers::Knockers::new(),
        doors: door::Doors::new(),
    }
}

impl WF {
    

    pub fn wait_the_end(&mut self) {
        debug!("Wait for the end of the workflow");
        let mut last_knock_cleanup: Instant = Instant::now();
        let mut last_doors_cleanup: Instant = Instant::now();
        let mut last_firwall_checkup: Instant = Instant::now();
        loop {
            std::thread::sleep(Duration::from_millis(100));
            if !unsafe { THREAD_RUNNING.load(Ordering::Relaxed) } {
                break;
            }
            let time_since_last_knock_clean_up: Duration = Instant::now() - last_knock_cleanup;
            if time_since_last_knock_clean_up > knockers::MAX_KNOCKER_LIVE_TIME {
                last_knock_cleanup = Instant::now();
                let _ = self.sender.send(Msg::CLEANUP);
            }
            let time_since_last_door_clean_up: Duration = Instant::now() - last_doors_cleanup;
            if time_since_last_door_clean_up > door::CLEANUP_PERIODE {
                last_doors_cleanup = Instant::now();
                self.doors.cleanup();
            }

            let time_since_last_firewall_checkup = Instant::now() - last_firwall_checkup;
            if time_since_last_firewall_checkup > firewall::RULES_CHECK_PERIODE {
                last_firwall_checkup = Instant::now();
                firewall::checkup(false);
            }
        }
    }

    /**
     * That's where messages are treated
     *
     * This is a blocking call.
     * When a message on @receiver is there, it's treated and then the function return.
     */
    fn iterate(&mut self) {
        match self.receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => self.treat_message(msg),
            Err(RecvTimeoutError::Timeout) => (),
            Err(e) => error!("Something wrong in the workflow iterate function : {}", e),
        }
    }

    fn treat_message(&mut self, m: Msg) {
        match m {
            Msg::QUIT => unsafe {
                WANT_TO_QUIT = AtomicBool::new(true);
            },
            Msg::KNOCK(k) => {
                self.knockers.event(k);
            }
            Msg::CLEANUP => {
                self.knockers.clean_up();
            }
        }
    }

    fn send_msg(&self, msg: Msg) {
        let _ = self.sender.send(msg);
    }
}

fn quit() {
    //unsafe {
        let m: Msg = Msg::QUIT;
        MAIN_WF.send_msg(m);
    //};
}

pub fn join() {
    //unsafe {
    MAIN_WF.wait_the_end();
    //};
    quit();
}

pub fn knock(k: knock::Knock) {
    debug!("konck on {} from {}", &k.port, &k.ip);
    unsafe {
        MAIN_WF.send_msg(Msg::KNOCK(k));
    };
}

pub fn open_the_door(ip: IpAddr) {
    unsafe {
        MAIN_WF.doors.open_the_door(ip);
    };
}

pub fn init() {
    debug!("Initializing the workflow.");
    let mut wf_ref_in_static: AtomicBool = AtomicBool::new(false);
    let _ = std::thread::spawn(move || {
        unsafe {
            THREAD_RUNNING = AtomicBool::new(true);
        };
        
        loop {
            if unsafe { WANT_TO_QUIT.load(Ordering::Relaxed) } {
                break;
            }
            unsafe {
                MAIN_WF.iterate();
            };
        }
        unsafe {
            THREAD_RUNNING = AtomicBool::new(false);
        };
    });
    while !wf_ref_in_static.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
