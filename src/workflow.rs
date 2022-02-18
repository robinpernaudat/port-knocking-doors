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

use std::time::{Duration, Instant};
use std::net::IpAddr;
use log::{debug};
use std::sync::mpsc::{channel, Sender, Receiver};//one sender in this channel


pub enum Msg{
    KNOCK(IpAddr, u16),
    QUIT,
}

static mut MAIN_WF: Option<WF_T> = None;


pub struct WF_T{
    //sender: Sender<Msg>,
    receiver: Receiver<Msg>,
    thread: std::thread::JoinHandle<()>,
}

impl WF_T{
    fn wait_the_end(self){
        debug!("Wait for the end of the workflow");
        self.thread.join();
    }
    
}

pub fn join(){
    unsafe{
        if let Some(w) = &MAIN_WF{
            &w.thread.join();
        }
    };
}

pub fn knock(who: IpAddr, knock_port: u16){
    debug!("konck on {} from {}", knock_port, who);
    
}

pub fn init()->Sender<Msg>{
    debug!("Initializing the workflow.");
    let (s,r)=channel();
    let th = std::thread::spawn(move ||{
        loop{}
    });
    let wf = WF_T{receiver:r, thread: th};
    unsafe{MAIN_WF = Some(wf);};
    s
}
