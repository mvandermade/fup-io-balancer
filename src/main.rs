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
    info!("Let's start some scanners!");
    let (snd, rcv) = crossbeam_channel::bounded::<PostzegelEvent>(1024);

    let mut workers = Vec::with_capacity(8);
    for nr in 1 ..= 3 {
        //TODO @mark: make a real scanner?
        let snd_copy = snd.clone();
        let scanner_worker = thread::Builder::new().name(format!("scanner{nr}"))
            .spawn(|| MockScanner::new(snd_copy).run())
            .expect("Failed to spawn scanner thread");
        workers.push(scanner_worker);
    }

    let balancer_worker = thread::Builder::new().name("balancer".to_string())
        .spawn(|| Balancer::new(rcv).run())
        .expect("Failed to spawn scanner thread");
    workers.push(balancer_worker);

    info!("Started {} threads", workers.len());
    for worker in workers {
        worker.join().expect("Failed to join thread");
    }
}
