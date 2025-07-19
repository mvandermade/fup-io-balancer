use std::path::PathBuf;
use ::env_logger;
use ::log::info;
use log::debug;
use crate::scanner::{PostzegelEvent, Scanner};

mod scanner;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("use RUST_LOG=... to change log level (debug/info/warn/error)");

    info!("Let's start some scnanners!");
    let (snd, rcv) = crossbeam_channel::bounded::<PostzegelEvent>(1024);
    let scanner = Scanner { address: PathBuf::from("/dev/ttyUSB0"), sink: snd.clone() };
    info!("Going to wait for postzegel events");
    loop {
        match rcv.recv() {
            Ok(event) => {
                debug!("Got a postzegel event {:?}", event)
            },
            Err(_) => panic!("channel disconnected, cannot get more events"),
        }
    }
    info!("ready!");
}

