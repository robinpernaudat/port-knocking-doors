//! This module concentrates the information or configuration to run the software.
//!
//! TODO us a configuration file
//!
use crate::MAIN_ARGS;
use log::debug;
use regex::Regex;

// struct Parameters{

// }

// pub fn configuration_path_finder() -> String{
//     if !envmnt::exists("KNOCKER_FILE") {
//         format!("{}",envmnt::get_or_panic("KNOCKER_FILE"))
//     }
// }
// fn regex_validators()->(Regex, Regex){
//     let re_matching_the_sequence: regex::Regex = Regex::new(r"^\d{1,5}(?:,\d{1,5}){4,20}$").unwrap();
//     let re_matching_ports_list = Regex::new(r"^\d{1,5}(?:,\d{1,5}){0,20}$").unwrap();
//     return (re_matching_the_sequence, re_matching_ports_list);
// }

/**
 * regex matching the port list
 *
 * It's matching a list of 1 to 20 port.
 * Each number have from 1 to 5 digit.
 */
fn match_port_liste() -> Regex {
    Regex::new(r"^\d{1,5}(?:,\d{1,5}){0,20}$").unwrap()
}

/**
 * Get the list of port to manage
 *
 * try from the command line argument (--ports=1,2,3)
 * else try the content of the env var KNOCKER_PORTS
 * else only the port tcp 22
 */
fn ports_string() -> String {
    let re_matching_ports_list = match_port_liste();

    // treat arguments.
    let seq_from_args: crate::args::Args = unsafe { MAIN_ARGS.clone().unwrap().clone() };
    if re_matching_ports_list.is_match(seq_from_args.ports.as_str()) {
        return seq_from_args.ports;
    }

    // if not yet, search in the
    if envmnt::exists("KNOCKER_PORTS") {
        let content: String = envmnt::get_or_panic("KNOCKER_PORTS");
        if re_matching_ports_list.is_match(content.as_str()) {
            return content;
        }
    }

    String::from("22")
}

pub fn ports() -> Vec<u16> {
    ports_string()
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

/**
 * regex matching the sequence list of ports
 *
 * It's matching a list of 4 to 20 numbers.
 * Each number have from 1 to 5 digit.
 */
fn re_matching_sequence() -> Regex {
    Regex::new(r"^\d{1,5}(?:,\d{1,5}){4,20}$").unwrap()
}

/**
 * Return the seq
 *
 * try to get the SEG from the cli argument
 * else try the content of the env var KNOCKER_SEQ
 * else return "16001,16002,16003,16004,16005"
 *
 */
fn knock_seq_string() -> String {
    let re_matching_the_sequence: regex::Regex = re_matching_sequence();
    // treat arguments.
    let seq_from_args: crate::args::Args = unsafe { MAIN_ARGS.clone().unwrap().clone() };
    if re_matching_the_sequence.is_match(seq_from_args.magic_seq.as_str()) {
        return seq_from_args.magic_seq;
    }

    // if not yet, search in the
    if envmnt::exists("KNOCKER_SEQ") {
        let content: String = envmnt::get_or_panic("KNOCKER_SEQ");
        if re_matching_the_sequence.is_match(content.as_str()) {
            return content;
        }
    }

    // by default :
    String::from("16001,16002,16003,16004,16005")
}

pub fn knock_seq() -> Vec<u16> {
    knock_seq_string()
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

pub fn is_terminal() -> bool {
    use atty::Stream;
    return atty::is(Stream::Stdout);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_match_port_liste() {
        let r: Regex = match_port_liste();
        assert!(r.is_match("1,2,3,5"));
    }

    #[test]
    fn test_matching_sequence() {
        let r: Regex = re_matching_sequence();
        assert!(r.is_match("222,3,5,97,6546,7974,14,14,41,41,41"));
        assert!(!r.is_match("54,,98,897,84,654,64,654,65,4654,e,2"));
    }
}
