use crate::balancer::Balancer;
use crate::postzegel_event::PostzegelEvent;
use crate::scanner::MockScanner;
use crate::scanner::RealScanner;
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
mod postzegel_event;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("use RUST_LOG=... to change log level (debug/info/warn/error)");
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let name = thread.name().unwrap_or("unnamed");
        eprintln!("[PANIC] in thread '{}': {}", name, info);
        exit(2)
    }));

    run();
}

fn run() {
    info!("Let's start some scnanners!");
    let (snd, rcv) = crossbeam_channel::bounded::<PostzegelEvent>(1024);
    let scanner1 = RealScanner { address: PathBuf::from("/dev/ttyUSB0"), sink: snd.clone() };
    let scanner2 = MockScanner { sink: snd.clone() };
    let balancer = Balancer { source: rcv };
    balancer.run();
}

struct Things {
    scanner: Vec<Scanner>,
    balancer: Balancer,
}
