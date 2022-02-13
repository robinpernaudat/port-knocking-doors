mod data;
mod args;

use log::{info, trace, warn, debug};


use log::{Record, Level, Metadata};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
use log::{SetLoggerError, LevelFilter};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_log() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
}

static mut MAIN_ARGS: Option<args::Args> = None;

fn main() {
    let _ = init_log();
    unsafe{MAIN_ARGS = Some(args::parse());}
    debug!("Starting");
    if data::is_terminal(){
        debug!("running in terminal");
    }
    debug!("port sequence = {:?}", data::knock_seq());
    debug!("ports = {:?}", data::ports());
}
