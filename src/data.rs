//! This module concentrates the information or configuration to run the software.
//!
//! TODO us a configuration file
//!
use crate::MAIN_ARGS;
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
 * Get the list of port to manage
 *
 * try from the command line argument (--ports=1,2,3)
 * else try the content of the env var KNOCKER_PORTS
 * else only the port tcp 22
 */
fn ports_string() -> String {
    let re_matching_ports_list = Regex::new(r"^\d{1,5}(?:,\d{1,5}){0,20}$").unwrap();

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
 * Return the seq
 *
 * try to get the SEG from the cli argument
 * else try the content of the env var KNOCKER_SEQ
 * else return "16001,16002,16003,16004,16005"
 *
 */
fn knock_seq_string() -> String {
    let re_matching_the_sequence: regex::Regex =
        Regex::new(r"^\d{1,5}(?:,\d{1,5}){4,20}$").unwrap();
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
