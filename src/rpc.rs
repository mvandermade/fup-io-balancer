use crate::rpc::balancer_svc_server::BalancerSvc;
use ::log::info;
use ::tonic::Response;
use ::tonic::Status;

tonic::include_proto!("balancerapi");

#[derive(Debug)]
pub struct BalancerRpc {}

#[tonic::async_trait]
impl BalancerSvc for BalancerRpc {
    async fn request_work(&self, request: tonic::Request<Request>) -> Result<Response<Reply>, Status> {
        let req = request.into_inner();
        info!("got request named {}, sending response", req.name);
        Ok(Response::new(Reply {
            message: format!("Hello, {}!", req.name),
        }))
    }
}
