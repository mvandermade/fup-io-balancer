use self::proto::balancer_svc_client::BalancerSvcClient;
use self::proto::WorkAcknowledgement;

use ::clap::Parser;
use ::env_logger;
use ::log::debug;
use ::log::info;
use ::std::panic;
use ::std::thread;
use ::std::time;

use ::futures::StreamExt;
use ::log::warn;
use ::tokio::sync::mpsc::channel;
use ::tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use ::tonic::transport::Channel;
use ::tonic::transport::Uri;

mod proto {
    #![allow(non_camel_case_types)]
    tonic::include_proto!("balancerapi");
}

#[derive(Parser, Debug)]
#[command(
    about = "Test client for postzegel (the real one is in Kotlin)",
)]
pub struct ClientArgs {
    /// The ip and port to connect to
    #[arg(short = 'a', long, default_value = "http://127.0.0.1:7331")]
    pub addr: Uri,
    /// How many times to retry the initial connection after the first
    /// (does not reconnect if disconnected later)
    #[arg(long, default_value = "999")]
    pub max_connection_retry: u32,
    /// Debug option to stop sending ack after some iterations
    #[arg(long)]
    pub max_ack: Option<u32>,
}

#[test]
fn test_cli_args() {
    ClientArgs::try_parse_from(["client", "-a", "http://localhost:8080"]).unwrap();
}

#[tokio::main]
async fn main() {
    let args = ClientArgs::parse();
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    run(&args).await;
}

async fn run(args: &ClientArgs) {
    info!("Starting test client, connecting to {}", args.addr);

    let mut client = connect_with_retry(&args.addr, args.max_connection_retry).await;
    info!("Connected to {}", args.addr);

    let (task_sender, task_receiver) = channel::<WorkAcknowledgement>(1);
    let outbound_stream = ReceiverStream::new(task_receiver);
    let mut work_stream = client.work(tonic::Request::new(outbound_stream))
        .await.expect("Could not send grpc request")
        .into_inner();
    let mut ack_sent = 0;
    while let Some(resp) = work_stream.next().await {
        info!("Received work request: {:?}", resp);
        if let Ok(resp) = resp {
            let max_ack = args.max_ack.unwrap_or(u32::MAX);
            if args.max_ack.is_none() || ack_sent < max_ack {
                ack_sent += 1;
                debug!("Acknowledging task {:?} (#{})", resp.task_id, ack_sent);
                task_sender.send(WorkAcknowledgement { task_id: resp.task_id, error: "".to_string() }).await.unwrap();
            } else {
                debug!("Not acknowledging task {:?} because at most {} tasks can be acknowledged (cli config)", resp.task_id, max_ack);
            }
        }
    }
    info!("End of response stream (server might have stopped, or kicked us)");
}

async fn connect_with_retry(addr: &Uri, max_connection_retry: u32) -> BalancerSvcClient<Channel> {
    assert!(addr.scheme().is_some(), "Provide a protocol to -a, like http:// or https://");
    for attempt in 0 .. max_connection_retry {
        match BalancerSvcClient::connect(addr.clone()).await {
            Ok(client) => return client,
            Err(err) => {
                warn!("Client could not connect to {addr}; err: {err}; retrying ({}/{})...", attempt + 1, max_connection_retry + 1);
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    }
    panic!("Could not connect to {addr} after {} attempts", max_connection_retry + 1);
}