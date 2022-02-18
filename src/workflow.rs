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

use log::{debug, error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender}; //one sender in this channel
use std::time::{Instant, Duration};
use std::net::IpAddr;

use crate::{knock, knockers, door};

pub enum Msg {
    KNOCK(knock::Knock),
    QUIT,
    CLEANUP,
}

static mut WANT_TO_QUIT: AtomicBool = AtomicBool::new(false);
static mut THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

static mut MAIN_WF: Option<WF> = None;

pub struct WF {
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
    knockers: knockers::Knockers,
    doors: door::Doors,
}

impl WF {
    pub fn wait_the_end(&mut self) {
        debug!("Wait for the end of the workflow");
        let mut last_knock_cleanup: Instant = Instant::now();
        let mut last_doors_cleanup: Instant = Instant::now();
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
            Msg::KNOCK(k) => {self.knockers.event(k);},
            Msg::CLEANUP => {
                self.knockers.clean_up();
            },
        }
    }

    fn send_msg(&self, msg: Msg) {
        let _ = self.sender.send(msg);
    }
}

fn quit() {
    unsafe {
        match &MAIN_WF {
            None => (),
            Some(wf) => {
                let m: Msg = Msg::QUIT;
                wf.send_msg(m);
            }
        }
    };
}

pub fn join() {
    unsafe {
        match &mut MAIN_WF {
            None => (),
            Some(wf) => wf.wait_the_end(),
        }
    };
    quit();
}

pub fn knock(k: knock::Knock) {
    debug!("konck on {} from {}", &k.port, &k.ip);
    unsafe {
        match &MAIN_WF {
            None => (),
            Some(wf) => {
                let m: Msg = Msg::KNOCK(k);
                wf.send_msg(m);
            }
        }
    };
}

pub fn open_the_door(ip: IpAddr){
    unsafe {
        match &mut MAIN_WF {
            None => (),
            Some(wf) => wf.doors.open_the_door(ip),
        }
    };
}

pub fn init() {
    debug!("Initializing the workflow.");
    let (s, r) = channel();
    let _ = std::thread::spawn(move || {
        unsafe {
            THREAD_RUNNING = AtomicBool::new(true);
        };
        let mut wf = WF {
            sender: s,
            receiver: r,
            knockers: knockers::Knockers::new(),
            doors: door::Doors::new(),
        };
        loop {
            if unsafe { WANT_TO_QUIT.load(Ordering::Relaxed) } {
                break;
            }
            wf.iterate();
        }
        unsafe {
            THREAD_RUNNING = AtomicBool::new(false);
        };
    });
}
