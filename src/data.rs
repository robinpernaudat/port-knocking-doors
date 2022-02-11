//! This module concentrates the information or configuration to run the software.
//!
//! TODO us a configuration file

struct Parameters{

}

// pub fn configuration_path_finder() -> String{
//     if !envmnt::exists("KNOCKER_FILE") {
//         format!("{}",envmnt::get_or_panic("KNOCKER_FILE"))
//     }
// }


pub fn knock_seq_string() -> String{
    if !envmnt::exists("KNOCKER_SE") {
        format!("{}",envmnt::get_or_panic("KNOCKER_FILE"))
    }
    String::from("1,2,3,4,5")
}



pub fn is_terminal()->bool{
    use atty::Stream;
    return atty::is(Stream::Stdout);
}