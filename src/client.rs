use self::proto::balancer_svc_client::BalancerSvcClient;
use self::proto::WorkAcknowledgement;

use ::clap::Parser;
use ::env_logger;
use ::futures::StreamExt;
use ::log::debug;
use ::log::info;
use ::log::warn;
use ::std::panic;
use ::std::thread;
use ::std::time;
use ::std::time::Duration;
use std::cmp::max;
use ::tokio::sync::mpsc::channel;
use ::tokio::time::sleep;
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
    /// Fail every nth task, for testing
    #[arg(long)]
    pub fail_every_nth: Option<u32>,
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
    let fail_every_nth = max(1, args.fail_every_nth.unwrap_or(u32::MAX));
    while let Some(resp) = work_stream.next().await {
        info!("Received work request: {:?}", resp);
        match resp {
            Ok(assignment) => {
                let max_ack = args.max_ack.unwrap_or(u32::MAX);
                if args.max_ack.is_none() || ack_sent < max_ack {
                    ack_sent += 1;
                    if ack_sent % fail_every_nth != 0 {
                        debug!("Acknowledging task {:?} (#{})", assignment.task_id, ack_sent);
                        task_sender.send(WorkAcknowledgement { task_id: assignment.task_id, error: "".to_string() }).await.unwrap();
                    } else {
                        debug!("Failing acknowledgement for task {:?} (#{})", assignment.task_id, ack_sent);
                        task_sender.send(WorkAcknowledgement { task_id: assignment.task_id, error: format!("test failure {ack_sent}") }).await.unwrap();
                    }
                } else {
                    debug!("Not acknowledging task {:?} because at most {} tasks can be acknowledged (cli config)", assignment.task_id, max_ack);
                }
            }
            Err(err) => {
                warn!("Error while receiving work: {:?}", err);
                break;
            }
        }
    }
    info!("End of response stream (server might have stopped, or kicked us)");
    sleep(Duration::from_millis(200)).await;
    debug!("Stopping client");
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