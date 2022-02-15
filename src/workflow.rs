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

pub async fn init(){
    debug!("Initializing the workflow.");
}

pub fn knock(who: IpAddr, knock_port: u16){
    debug!("konck on {} from {}", knock_port, who);

}

pub fn wait_the_end(){

}