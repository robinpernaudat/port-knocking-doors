//! This module integrate interactions with the firewall.
//!
//! Here we open and close ports by managing de firewall zone "knock-access".
//!
//! The zone and rules aren't permenent. So il the firewall is restarted, the zone will be create again
//! and the clients have to réopen there ports.

use log::{debug, info};
use std::net::IpAddr;
use std::process::Command;

use crate::data;

#[derive(Clone)]
enum FirewallType {
    UNDEFINED,
    //IPTABLES,
    FIREWALLD,
    #[cfg(target_os = "windows")]
    WINDOWS_DEFAULT,
}

static mut FIREWALL_TYPE: FirewallType = FirewallType::UNDEFINED;

#[cfg(target_os = "linux")]
pub fn init() {
    info!("firewall identification");
    let cmd_exec = Command::new("bash")
        .arg("-c")
        .arg("(which firewalld || which iptables || echo error) | sed 's/^.*\\///'")
        .output()
        .expect("failed to execute process");
    let whiche_one: String = String::from_utf8(cmd_exec.stdout).unwrap();
    let wall = if whiche_one.eq("firewalld\n") {
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
    debug!("opening the zone for the ip {}", ip);
    match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--zone=knock-access")
            .arg(format!("--add-source={}/32", ip))
            .output()
            .expect("Can't add ip in white list."),
        //FirewallType::IPTABLES => "",
        _ => panic!("This app can't work if it can't control the firewall."),
    };

    return true;
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
    return true;
}

fn is_firewall_zone_exists() -> bool {
    debug!("Checking if the firewall zone exists");
    let regex_for_zone_name_detection: regex::Regex = regex::Regex::new("^knock-access.*").unwrap();
    let result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("bash")
            .arg("-c")
            .arg("firewall-cmd --list-all-zones | egrep \"^knock-access\"")
            .output()
            .expect("error wil creating a zone"),
        //FirewallType::IPTABLES => Command::new("").output().expect("error wil creating a zone"),,
        _ => panic!("This app can't work if it can't control the firewall."),
    };
    let s: String = String::from_utf8(result.stdout).unwrap();

    if regex_for_zone_name_detection.is_match(s.as_str()) {
        return true;
    }
    debug!("firewall zone not found it responded : {}", s);
    false
}

fn prepare_firewall_zone() {
    info!("preparing the firewall zone");
    let result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--new-zone=knock-access")
            .arg("--permanent")
            .output()
            .expect("error while creating a zone"),
        //FirewallType::IPTABLES => Command::new("").output().expect("error wil creating a zone"),,
        _ => panic!("This app can't work if it can't control the firewall."),
    };
    firewall_cmd_reload();
    debug!(
        "returned : {}  |  {}",
        String::from_utf8(result.stdout).unwrap(),
        String::from_utf8(result.stderr).unwrap()
    );
}

fn drop_firewall_zone() {
    info!("droping the firewall zone.");
    let _result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--delete-zone=knock-access")
            .arg("--permanent")
            .output()
            .expect("error while removing a zone"),
        //FirewallType::IPTABLES => Command::new("").output().expect("error wil creating a zone"),,
        _ => panic!("This app can't work if it can't control the firewall."),
    };
    firewall_cmd_reload();
}

fn firewall_cmd_reload() {
    info!("reloading the firewall conf");
    let _result = match unsafe { FIREWALL_TYPE.clone() } {
        FirewallType::FIREWALLD => Command::new("firewall-cmd")
            .arg("--reload")
            .output()
            .expect("error while reloading the firewall"),
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
    debug!("firewall checkup");
    if !is_firewall_zone_exists() {
        debug!("the firewall zone doesn't exists");
        prepare_firewall_zone();
        add_ports_list_to_the_firewall_zone();
    } else if reset {
        debug!("the firewall zone exists");
        drop_firewall_zone();
        checkup(false);
    }
}
