use crate::balancer::Balancer;
use crate::channel::channel;
use crate::channel::Fork;
use crate::cli::CliArgs;
use crate::dispatcher::Dispatcher;
use crate::global::ChannelKey;
use crate::postzegel::PostzegelEvent;
use crate::rpc::BalancerRpc;
use crate::rpc::BalancerSvcServer;
use crate::scanner::MockScanner;
use ::clap::Parser;
use ::env_logger;
use ::log::info;
use ::std::net::SocketAddr;
use ::std::panic;
use ::std::process::exit;
use ::std::sync::Arc;
use ::std::thread;
use ::tonic::transport::Server;

mod dispatcher;
mod task_util;
mod rpc;
mod balancer;
mod scanner;
mod postzegel;
mod workers;
mod cli;
mod demos;
mod channel;
mod global;

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

    run(args.addr, args.no_worker_delay_us).await;
}

async fn run(addr: SocketAddr, no_worker_delay_us: u64) {
    info!("Let's start some scanners!");
    let (snd, rcv) = channel::<PostzegelEvent>(1024, ChannelKey::Scanner);

    let mut workers = Vec::with_capacity(8);
    for nr in 1 ..= 3 {
        //TODO @mark: make a real scanner?
        let snd_copy = snd.fork();
        let scanner_worker = tokio::spawn(MockScanner::new(nr, snd_copy).run());
        workers.push(scanner_worker);
    }

    let dispatcher = Arc::new(Dispatcher::new());
    let dispatcher_clone = dispatcher.clone();
    let balancer_worker = tokio::spawn(Balancer::new(rcv, dispatcher_clone).run(no_worker_delay_us));
    workers.push(balancer_worker);

    info!("Going to listen on {}", addr);
    Server::builder()
        .add_service(BalancerSvcServer::new(BalancerRpc::new(dispatcher)))
        .serve(addr)
        .await.expect("Could not start server");

    info!("Started {} tasks", workers.len());
}
