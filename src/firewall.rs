//! This module integrate interactions with the firewall.
//!
//! Here we open and close ports by managing de firewall zone "knock-access".
//!
//! The zone and rules aren't permenent. So il the firewall is restarted, the zone will be create again
//! and the clients have to réopen there ports.

use log::info;
use std::net::IpAddr;
use std::process::Command;
use std::time::Duration;

use crate::data;

#[derive(Clone)]
enum FirewallType {
    UNDEFINED,
    //IPTABLES,
    FIREWALLD,
    #[cfg(target_os = "windows")]
    WINDOWS_DEFAULT,
}

pub const RULES_CHECK_PERIODE: Duration = Duration::from_secs(10);
static mut FIREWALL_TYPE: FirewallType = FirewallType::UNDEFINED;

#[cfg(target_os = "linux")]
pub fn init() {
    // firewall identification
    let cmd_exec = Command::new("bash")
        .arg("(which firewalld || which iptables || echo error) | sed 's/^.*\\///'")
        .output()
        .expect("failed to execute process");
    let whiche_one: String = String::from_utf8(cmd_exec.stdout).unwrap();
    let wall = if whiche_one == "firewalld" {
        FirewallType::FIREWALLD
    }
    /*else if whiche_one == "iptables"{
        FirewallType::IPTABLES
    }*/
    else if whiche_one == "error" {
        panic!("firewalld must be installed");
    } else {
        panic!("OOOOOhhhhh");
    };
    unsafe { FIREWALL_TYPE = wall };

    checkup();
}

#[cfg(target_os = "windows")]
pub fn init() {
    panic!("The firewall control for Windows isn't implemented yet !");
}

pub fn open(ip: IpAddr) -> bool {
    //firewall-cmd --zone=public --add-port=80/tcp
    //--add-source=10.24.96.5/20
    // voir l'histoire de zone.
    let cmd = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => "firewall-cmd --zone=knock-access --add-source={}/32",
        //FirewallType::IPTABLES => "",
        _ => panic!("This app can't work if it can't control the firewall."),
    };
    todo!();
    return false;
}

pub fn close(ip: IpAddr) -> bool {
    //firewall-cmd --zone=public --remove-port=10050/tcp
    let cmd = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => "firewall-cmd --zone=knock-access --remove-source={}/32",
        //FirewallType::IPTABLES => "",
        _ => panic!("This app can't work if it can't control the firewall."),
    };
    todo!();
    return false;
}

fn is_firewall_zone_exists() -> bool {
    let result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("bash")
            .arg("firewall-cmd --list-all-zones | egrep \"^knock-access\"")
            .output()
            .expect("error wil creating a zone"),
        //FirewallType::IPTABLES => Command::new("").output().expect("error wil creating a zone"),,
        _ => panic!("This app can't work if it can't control the firewall."),
    };
    let s: String = String::from_utf8(result.stdout).unwrap();
    if s == String::from("knock-access\n") {
        return true;
    }
    false
}

fn prepare_firewall_zone() {
    let _result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--new-zone=knock-access")
            .output()
            .expect("error wil creating a zone"),
        //FirewallType::IPTABLES => Command::new("").output().expect("error wil creating a zone"),,
        _ => panic!("This app can't work if it can't control the firewall."),
    };
}

fn add_ports_list_to_the_firewall_zone(){
    let ports: Vec<u16> = data::ports();
    for port in ports{
        Command::new("firewall-cmd")
            .arg("--zone=knock-access")
            .arg(format!("--add-port={}/tcp", port))
            .output()
            .expect("error while opening ports for our zone");
    }
}

pub fn checkup() {
    if !is_firewall_zone_exists() {
        prepare_firewall_zone();
        add_ports_list_to_the_firewall_zone();
    }
}
