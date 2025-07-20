#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use ::clap::Parser;
use ::env_logger;
use ::log::debug;
use ::log::info;
use std::env::args;
use ::std::panic;
use ::std::path::PathBuf;
use ::std::process::exit;
use ::std::thread;
use std::time;
use ::tonic::transport::Server;

use crate::balancer_svc_client::BalancerSvcClient;
use futures::StreamExt;
use log::warn;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::channel;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Channel, Error, Uri};
use ::tonic::Response;
use ::tonic::Status;

tonic::include_proto!("balancerapi");

#[derive(Parser, Debug)]
#[command(
    about = "Test client for postzegel (the real one is in Kotlin)",
)]
pub struct ClientArgs {
    /// The ip and port to connect to
    #[arg(short = 'a', long, default_value = "http://127.0.0.1:7331")]
    pub addr: Uri,
    /// How many times to retry the initial connection (does not reconnect if disconnected later)
    #[arg(short = 'r', long, default_value = "1000")]
    pub max_connection_retry: u32,
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

    run(args.addr, args.max_connection_retry).await;
}

async fn run(addr: Uri, max_connection_retry: u32) {
    info!("Starting test client, connecting to {addr}");

    assert!(addr.scheme().is_some(), "Provide a protocol to -a, like http:// or https://");
    let mut client = connect_with_retry(&addr, max_connection_retry).await;
    info!("Connected to {addr}");

    let (task_sender, task_receiver) = channel::<WorkAcknowledgement>(1);
    let outbound_stream = ReceiverStream::new(task_receiver);
    let mut response_stream = client.work(tonic::Request::new(outbound_stream))
        .await.expect("Could not send grpc request")
        .into_inner();
    while let Some(resp) = response_stream.next().await {
        info!("Received response: {:?}", resp);
        if let Ok(resp) = resp {
            debug!("Acknowledging task: {:?}", resp.task_id);
            task_sender.send(WorkAcknowledgement { task_id: resp.task_id, error: "".to_string() });
        }
    }
    info!("End of response stream (server might have stopped, or kicked us)");
}

async fn connect_with_retry(addr: &Uri, max_connection_retry: u32) -> BalancerSvcClient<Channel> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        if attempt > max_connection_retry {
            panic!("Could not connect to {addr} after {max_connection_retry} attempts");
        }
        match BalancerSvcClient::connect(addr.clone()).await {
            Ok(client) => break client,
            Err(err) => {
                warn!("Client could not connect to {addr}; err: {err}; retrying ({attempt}/{max_connection_retry})...");
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    }
}
