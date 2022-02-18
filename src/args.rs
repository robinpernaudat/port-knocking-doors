use clap::Parser;

/**
This software manage the port knocking.

You can also use the environnement variables:
  KNOCKER_SEQ for the magic sequence at least 5 UDP ports (ex `export KNOCKER_SEQ=1,2,3,4,5,6,7`)
  KNOCKER_PORTS to define the list of port to open (ex `export KNOCKER_PORTS=22,80,10080`)
*/
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t = String::from("#####"))]
    pub magic_seq: String,

    #[clap(short, long, default_value_t = String::from("#####"))]
    pub ports: String,
}

pub fn parse() -> Args {
    return Args::parse();
}
