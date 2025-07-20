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
use futures::StreamExt;
use log::warn;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::channel;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Channel, Error, Uri};
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
    let mut client = loop {
        match BalancerSvcClient::connect(addr.clone()).await {
            Ok(client) => break client,
            Err(err) => {
                warn!("Client could not connect to {addr}; err: {err}; will retry");
                thread::sleep(::std::time::Duration::from_secs(2));
            }
        }
    };

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
