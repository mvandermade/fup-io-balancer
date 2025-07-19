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
    #[arg(short = 'b', default_value = "127.0.0.1:7331")]
    pub addr: Uri,
}

#[test]
fn test_cli_args() {
    ClientArgs::try_parse_from(&["client", "-b", "localhost:8080"]).unwrap();
}

// #[derive(Debug, Clone)]
// pub struct ClientRpc {}
//
// impl ClientRpc {
//     pub fn new() -> Self {
//         ClientRpc {}
//     }
// }
//
// #[tonic::async_trait]
// impl BalancerSvc for ClientRpc {
//     async fn request_work(&self, request: tonic::Request<Request>) -> Result<Response<Reply>, Status> {
//         let req = request.into_inner();
//         info!("got request named {}, sending response", req.name);
//         Ok(Response::new(Reply {
//             message: format!("Hello, {}!", req.name),
//         }))
//     }
// }

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

    let mut client = BalancerSvcClient::connect(addr.clone()).await
        .unwrap_or_else(|err| panic!("Client could not connect to {addr}"));

    // Client::builder()
    //     .add_service(BalancerSvcClient::new(ClientRpc::new()))
    //     .serve(addr)
    //     .await.expect("Could not start server");
}
