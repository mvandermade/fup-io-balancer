use crate::scanner::PostzegelEvent;
use crate::scanner::Scanner;
use ::env_logger;
use ::log::debug;
use ::log::info;
use ::std::panic;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::thread;

mod scanner;
mod balancer;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("use RUST_LOG=... to change log level (debug/info/warn/error)");
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let name = thread.name().unwrap_or("unnamed");
        eprintln!("[PANIC] in thread '{}': {}", name, info);
        exit(2)
    }));

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

