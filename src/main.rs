mod args;
mod config;
mod data;
mod door;
mod firewall;
mod knock;
mod knockers;
mod workflow;

use log::{/*trace, warn,*/ debug, info};

use log::{Level, Metadata, Record};

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

#[cfg(debug_assertions)]
pub fn init_log() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
}
#[cfg(not(debug_assertions))]
pub fn init_log() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
}

static mut MAIN_ARGS: Option<args::Args> = None;

#[tokio::main]
async fn main() {
    init_log();
    unsafe {
        MAIN_ARGS = Some(args::parse());
        config::CONFIGURATION = Some(config::load_conf());
        if MAIN_ARGS.clone().unwrap().set_configuration_file_in_home {
            config::store();
            return;
        }
    }
    info!("Starting");
    if data::is_terminal() {
        debug!("running in terminal");
    }
    debug!("port sequence = {:?}", data::knock_seq());
    info!("List of managed ports : {:?}", data::ports());

    knockers::Knockers::init();
    firewall::init();
    workflow::init();
    door::init().await;

    workflow::join();
}
