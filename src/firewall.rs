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
    info!("firewall identification");
    let cmd_exec = Command::new("bash")
        .arg("(which firewalld || which iptables || echo error) | sed 's/^.*\\///'")
        .output()
        .expect("failed to execute process");
    let whiche_one: String = String::from_utf8(cmd_exec.stdout).unwrap();
    let wall = if whiche_one == "firewalld" {
        FirewallType::FIREWALLD
    }
    // else if whiche_one == "iptables"{
    //     FirewallType::IPTABLES
    // }
    else if whiche_one == "error" {
        panic!("firewalld must be installed");
    } else {
        panic!("Bad firewall : {}", whiche_one);
    };
    unsafe { FIREWALL_TYPE = wall };

    checkup(true);
}

#[cfg(target_os = "windows")]
pub fn init() {
    panic!("The firewall control for Windows isn't implemented yet !");
}

pub fn open(ip: IpAddr) -> bool {
    match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--zone=knock-access")
            .arg(format!("--add-source={}/32", ip))
            .output()
            .expect("Can't add ip in white list."),
        //FirewallType::IPTABLES => "",
        _ => panic!("This app can't work if it can't control the firewall."),
    };

    return false;
}

pub fn close(ip: IpAddr) -> bool {
    match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--zone=knock-access")
            .arg(format!("--remove-source={}/32", ip))
            .output()
            .expect("Can't remove ip in white list."),
        //FirewallType::IPTABLES => "",
        _ => panic!("This app can't work if it can't control the firewall."),
    };
    return false;
}

fn is_firewall_zone_exists() -> bool {
    info!("Checking if the firewall zone exists");
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
    info!("preparing the firewall zone");
    let _result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--new-zone=knock-access")
            .output()
            .expect("error while creating a zone"),
        //FirewallType::IPTABLES => Command::new("").output().expect("error wil creating a zone"),,
        _ => panic!("This app can't work if it can't control the firewall."),
    };
}

fn drop_firewall_zone() {
    info!("droping the firewall zone.");
    let _result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--delete-zone=knock-access")
            .output()
            .expect("error while removing a zone"),
        //FirewallType::IPTABLES => Command::new("").output().expect("error wil creating a zone"),,
        _ => panic!("This app can't work if it can't control the firewall."),
    };
}

fn add_ports_list_to_the_firewall_zone() {
    info!("configuring the port of the firewall's zone.");
    let ports: Vec<u16> = data::ports();
    for port in ports {
        Command::new("firewall-cmd")
            .arg("--zone=knock-access")
            .arg(format!("--add-port={}/tcp", port))
            .output()
            .expect("error while opening ports for our zone");
    }
}

/**
 * it ensure the firewall configured.    
 *
 * If the zone doesn't exist it create it and set the port concerned.
 * Else if @reset == true, it will recreate the zone. So if the daemon restarted, it will cleanup the zone.
 */
pub fn checkup(reset: bool) {
    info!("firewall checkup");
    if !is_firewall_zone_exists() {
        prepare_firewall_zone();
        add_ports_list_to_the_firewall_zone();
    } else if reset {
        drop_firewall_zone();
        checkup(false);
    }
}
