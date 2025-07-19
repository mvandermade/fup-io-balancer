#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use ::clap::Parser;
use ::env_logger;
use ::log::debug;
use ::log::info;
use ::std::panic;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::thread;
use ::tonic::transport::Server;

use ::tonic::Response;
use ::tonic::Status;
use tonic::transport::Uri;
use crate::balancer_svc_client::BalancerSvcClient;

tonic::include_proto!("balancerapi");

#[derive(Parser, Debug)]
#[command(
    about = "Test client for postzegel (the real one is in Kotlin)",
)]
pub struct ClientArgs {
    /// The ip and port to connect to
    #[arg(short = 'a', long, default_value = "http://127.0.0.1:7331")]
    pub addr: Uri,
}

#[test]
fn test_cli_args() {
    ClientArgs::try_parse_from(&["client", "-a", "http://localhost:8080"]).unwrap();
}

#[tokio::main]
async fn main() {
    let args = ClientArgs::parse();
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    run(args.addr).await;
}

async fn run(addr: Uri) {
    info!("Starting test client, connecting to {addr}");

    assert!(addr.scheme().is_some(), "Provide a protocol to -a, like http:// or https://");
    let mut client = BalancerSvcClient::connect(addr.clone()).await
        .unwrap_or_else(|err| panic!("Client could not connect to {addr}; err: {err}"));
    info!("Connected to {addr}");

    info!("Sending request");
    let req = WorkRequest { request: Some(Request::Availability(WorkerAvailability { name: "dummy-client".to_string() })) };
    let resp = client.request_work(tonic::Request::new(req))
        .await.expect("Could not send grpc request");
    info!("Received response: {:?}", resp);
}
