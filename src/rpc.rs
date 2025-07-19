use crate::rpc::balancer_svc_server::BalancerSvc;
use ::std::pin::Pin;
use futures::StreamExt;
use log::debug;
use tonic::IntoRequest;
use crate::rpc::work_request::Request;

tonic::include_proto!("balancerapi");

#[derive(Debug, Clone)]
pub struct BalancerRpc {}

impl BalancerRpc {
    pub fn new() -> Self {
        BalancerRpc {}
    }
}

#[tonic::async_trait]
impl BalancerSvc for BalancerRpc {
    // async fn request_work(&self, request: tonic::Request<WorkRequest>) -> Result<tonic::Response<WorkResponse>, tonic::Status> {
    //     let req = request.into_inner();
    //     info!("got request named {}, sending response", req.name);
    //     Ok(tonic::Response::new(WorkResponse {
    //         message: format!("Hello, {}!", req.name),
    //     }))
    // }

    type workStream = Pin<Box<dyn futures::Stream<Item = Result<WorkAssignment, tonic::Status>> + Send + 'static>>;

    async fn work(&self, request: tonic::Request<tonic::Streaming<WorkRequest>>) -> Result<tonic::Response<Self::workStream>, tonic::Status> {

        let mut request_stream = request.into_inner();
        while let Some(req) = request_stream.next().await {
            let Ok(req) = req else {
                panic!("error reading work request");
            };
            let Some(req) = req.request else {
                panic!("empty work request");
            };

            match req {
                Request::Availability(available) => {}
                Request::Ack(ack) => {
                    debug!("Got ack for work request {}", ack.work_id);
                    //TODO @mark: make sure we get ack for each work request, otherwise re-send
                }
            }
        }

        todo!();
    }
}
