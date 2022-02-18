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
use std::time::Duration;

use crate::knock;

pub enum Msg {
    KNOCK(knock::Knock),
    QUIT,
}

static mut WANT_TO_QUIT: AtomicBool = AtomicBool::new(false);
static mut THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

static mut MAIN_WF: Option<WF> = None;

pub struct WF {
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
}

impl WF {
    pub fn wait_the_end(&self) {
        debug!("Wait for the end of the workflow");
        loop {
            std::thread::sleep(Duration::from_millis(100));
            if !unsafe { THREAD_RUNNING.load(Ordering::Relaxed) } {
                break;
            }
        }
    }

    /**
     * That's where messages are treated
     *
     * This is a blocking call.
     * When a message on @receiver is there, it's treated and then the function return.
     */
    fn iterate(&self) {
        match self.receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => self.treat_message(msg),
            Err(RecvTimeoutError::Timeout) => (),
            Err(e) => error!("Something wrong in the workflow iterate function : {}", e),
        }
    }

    fn treat_message(&self, m: Msg) {
        match m {
            Msg::QUIT => unsafe {
                WANT_TO_QUIT = AtomicBool::new(true);
            },
            Msg::KNOCK(k) => {}
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
        match &MAIN_WF {
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

pub fn init() {
    debug!("Initializing the workflow.");
    let (s, r) = channel();
    let _ = std::thread::spawn(move || {
        unsafe {
            THREAD_RUNNING = AtomicBool::new(true);
        };
        let wf = WF {
            sender: s,
            receiver: r,
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
