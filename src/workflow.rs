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

use crossbeam_channel::{bounded, Receiver, RecvTimeoutError, Sender}; //one sender in this channel
use log::{debug, error, info};
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::{config, door, firewall, knock, knockers};

#[derive(PartialEq, Eq, Debug)]
pub enum Msg {
    KNOCK(knock::Knock),
    //ITERATE,
    QUIT,
}

static mut WANT_TO_QUIT: AtomicBool = AtomicBool::new(false);
static mut THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

/*lazy_static! {
    static ref MAIN_WF: MutStatic<WF> = MutStatic::from(new());
}*/

static mut G_WF_COM: WFComT = WFComT {
    sender: None,
    receiver: None,
};

#[derive(Clone)]
pub struct WFComT {
    sender: Option<Sender<Msg>>,
    receiver: Option<Receiver<Msg>>,
}

impl Drop for WFComT {
    fn drop(&mut self) {
        info!("END of the workflow");
    }
}

fn new() -> WFComT {
    debug!("Setup the MAIN_WF");
    let (s, r) = bounded(50);
    WFComT {
        sender: Some(s),
        receiver: Some(r),
    }
}

impl WFComT {
    pub fn wait_the_end(&self) {
        debug!("Wait for the end of the workflow");
        let max_knocker_live_time =
            unsafe { config::CONFIGURATION.clone().unwrap().max_knocker_live_time };
        let cleanup_periode =
            unsafe { config::CONFIGURATION.clone().unwrap().doors_cleanup_periode };
        let firewall_rules_check_periode_seconds = unsafe {
            config::CONFIGURATION
                .clone()
                .unwrap()
                .firewall_rules_check_periode_seconds
        };
        let mut last_knock_cleanup: Instant = Instant::now();
        let mut last_doors_cleanup: Instant = Instant::now();
        let mut last_firwall_checkup: Instant = Instant::now();
        while unsafe { THREAD_RUNNING.load(Ordering::Relaxed) } {
            std::thread::sleep(Duration::from_millis(100));

            let time_since_last_knock_clean_up: Duration = Instant::now() - last_knock_cleanup;
            if time_since_last_knock_clean_up > max_knocker_live_time {
                last_knock_cleanup = Instant::now();
                knockers::clean_up();
            }
            let time_since_last_door_clean_up: Duration = Instant::now() - last_doors_cleanup;
            if time_since_last_door_clean_up > cleanup_periode {
                last_doors_cleanup = Instant::now();
                door::cleanup();
            }

            let time_since_last_firewall_checkup = Instant::now() - last_firwall_checkup;
            if time_since_last_firewall_checkup > firewall_rules_check_periode_seconds {
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
        //debug!(".");
        match self
            .receiver
            .clone()
            .unwrap()
            .recv_timeout(Duration::from_secs(1))
        {
            Ok(msg) => self.treat_message(msg),
            Err(RecvTimeoutError::Timeout) => {
                /*debug!("timeout");*/
                ()
            }
            Err(e) => error!("Something wrong in the workflow iterate function : {}", e),
        }
        //debug!("#");
    }

    fn treat_message(&mut self, m: Msg) {
        //debug!("treat the message {:?}", m);
        match m {
            Msg::QUIT => unsafe {
                WANT_TO_QUIT = AtomicBool::new(true);
            },
            Msg::KNOCK(k) => {
                crate::knockers::event(k);
            } //Msg::ITERATE => {self.iterate()},
        }
    }

    fn send_msg(&mut self, msg: Msg) {
        //debug!("sending the message {:?}", msg);
        let _ = self.sender.clone().unwrap().send(msg);
    }
}

fn quit() {
    let m: Msg = Msg::QUIT;
    unsafe {
        G_WF_COM.send_msg(m);
    }
}

pub fn join() {
    debug!("joinning the workflow thread");
    unsafe {
        G_WF_COM.wait_the_end();
    };
    quit();
}

pub fn knock(k: knock::Knock) {
    debug!("knock on {} from {}", &k.port, &k.ip);
    let m = Msg::KNOCK(k);
    unsafe {
        G_WF_COM.send_msg(m);
    }
}

pub fn open_the_door(ip: IpAddr) {
    crate::door::open_the_door(ip);
}

pub fn init() {
    debug!("Initializing the workflow.");
    unsafe {
        G_WF_COM = new();
    };
    std::thread::spawn(move || {
        unsafe {
            THREAD_RUNNING = AtomicBool::new(true);
        };

        loop {
            if unsafe { WANT_TO_QUIT.load(Ordering::Relaxed) } {
                break;
            }
            unsafe {
                G_WF_COM.iterate();
            }; //(Msg::ITERATE);
        }
        unsafe {
            THREAD_RUNNING = AtomicBool::new(false);
        };
    });
    while !unsafe { THREAD_RUNNING.load(Ordering::Relaxed) } {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    debug!("workflow fully initialized");
}
