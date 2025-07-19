use crate::rpc::balancer_svc_server::BalancerSvc;
use ::log::info;

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
    async fn request_work(&self, request: tonic::Request<WorkRequest>) -> Result<tonic::Response<WorkResponse>, tonic::Status> {
        let req = request.into_inner();
        info!("got request named {}, sending response", req.name);
        Ok(tonic::Response::new(WorkResponse {
            message: format!("Hello, {}!", req.name),
        }))
    }
}
