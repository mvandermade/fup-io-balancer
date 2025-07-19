use crate::rpc::balancer_svc_server::BalancerSvc;
use ::std::pin::Pin;
use std::sync::atomic;
use std::sync::atomic::AtomicU64;
use dashmap::DashMap;
use futures::StreamExt;
use log::{debug, info};
use tonic::IntoRequest;
use crate::rpc::work_request::Request;

tonic::include_proto!("balancerapi");

#[derive(Debug)]
pub struct BalancerRpc {
    top_id: AtomicU64,
    workers: DashMap<u32, ()>
}

impl BalancerRpc {
    pub fn new() -> Self {
        BalancerRpc { top_id: AtomicU64::new(0), workers: DashMap::with_capacity(16) }
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
        let worker_id = self.top_id.fetch_add(1, atomic::Ordering::SeqCst);

        let mut request_stream = request.into_inner();
        while let Some(req) = request_stream.next().await {
            let Ok(req) = req else {
                panic!("error reading work request");
            };
            let Some(req) = req.request else {
                panic!("empty work request");
            };

            match req {
                Request::Availability(available) => {
                    info!("New available worker: {}", available.name);
                    self.workers.insert().unwrap();
                }
                Request::Ack(ack) => {
                    debug!("Got ack for work request {}", ack.work_id);
                    //TODO @mark: make sure we get ack for each work request, otherwise re-send
                }
            }
        }

        todo!();
    }
}
