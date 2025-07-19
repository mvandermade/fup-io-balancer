#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use crate::balancer::Balancer;
use crate::cli::CliArgs;
use crate::postzegel_event::PostzegelEvent;
use crate::rpc::balancer_svc_server::BalancerSvcServer;
use crate::rpc::BalancerRpc;
use crate::scanner::MockScanner;
use crate::scanner::RealScanner;
use crate::scanner::Scanner;
use ::clap::Parser;
use ::env_logger;
use ::log::debug;
use ::log::info;
use ::std::net::SocketAddr;
use ::std::panic;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::thread;
use ::tonic::transport::Server;

mod rpc;
mod balancer;
mod scanner;
mod postzegel_event;
mod demos;
mod cli;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let default_log = match (args.quiet, args.verbose) {
        (true, true) => unreachable!(),
        (true, false) => "error",
        (false, true) => "debug",
        (false, false) => {
            "info"
        },
    };
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, default_log),
    );
    if default_log == "info" {
        info!("Pass -v or -q as cli args, or alternatively use RUST_LOG=... to change log level (debug/info/warn/error)");
    }
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let name = thread.name().unwrap_or("unnamed");
        eprintln!("[PANIC] in thread '{}': {}", name, info);
        exit(2)
    }));

    run(args.addr).await;
}

async fn run(addr: SocketAddr) {
    info!("Let's start some scanners!");
    let (snd, rcv) = crossbeam_channel::bounded::<PostzegelEvent>(1024);

    let mut workers = Vec::with_capacity(8);
    for nr in 1 ..= 3 {
        //TODO @mark: make a real scanner?
        let snd_copy = snd.clone();
        let scanner_worker = thread::Builder::new().name(format!("scanner{nr}"))
            .spawn(move || MockScanner::new(nr, snd_copy).run())
            .expect("Failed to spawn scanner thread");
        workers.push(scanner_worker);
    }

    let balancer_worker = thread::Builder::new().name("balancer".to_string())
        .spawn(|| Balancer::new(rcv).run())
        .expect("Failed to spawn scanner thread");
    workers.push(balancer_worker);

    info!("Going to listen on {}", addr);
    Server::builder()
        .add_service(BalancerSvcServer::new(BalancerRpc::new()))
        .serve(addr)
        .await.expect("Could not start server");

    info!("Started {} threads", workers.len());
    for worker in workers {
        worker.join().expect("Failed to join thread");
    }
}
