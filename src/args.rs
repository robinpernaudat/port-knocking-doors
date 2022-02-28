use clap::Parser;


/// This software manage the port knocking.
///
/// You can also use the environnement variables:
///  KNOCKER_SEQ for the magic sequence at least 5 UDP ports (ex `export KNOCKER_SEQ=1,2,3,4,5,6,7`)
///  KNOCKER_PORTS to define the list of port to open (ex `export KNOCKER_PORTS=22,80,10080`)
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// this is the magic sequence (between 5 and 20 UDP ports) in the format "port_num,port_num,port_num,port_num,port_num" (example: "10001,10002,10003,10004,10005")
    #[clap(short, long, default_value_t = String::from("#####"))]
    pub magic_seq: String,

    /// this is the ports that we want to control. The format is like this "port1,port2,..." (examples: "22", "22,80,8080")
    #[clap(short, long, default_value_t = String::from("#####"))]
    pub ports: String,

    /// set configuration file path
    #[clap(short, long, default_value_t = String::from(""))]
    pub conf: String,
}

pub fn parse() -> Args {
    return Args::parse();
}
