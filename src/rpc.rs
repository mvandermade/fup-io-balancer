use crate::rpc::balancer_svc_server::BalancerSvc;
use ::dashmap::DashMap;
use ::futures::StreamExt;
use ::log::debug;
use ::log::info;
use ::std::pin::Pin;
use ::std::sync::atomic;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use std::sync::Arc;
use ::tonic::IntoRequest;
use log::error;
use crate::dispatcher::{Dispatcher, WorkId};

tonic::include_proto!("balancerapi");

#[derive(Debug)]
pub struct BalancerRpc {
    dispatcher: Arc<Dispatcher>,
}

impl BalancerRpc {
    pub fn new(dispatcher: Arc<Dispatcher>) -> Self {
        BalancerRpc { dispatcher }
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

    async fn work(&self, request: tonic::Request<tonic::Streaming<WorkAcknowledgement>>) -> Result<tonic::Response<Self::workStream>, tonic::Status> {
        let worker_id = self.top_worker_id.fetch_add(1, atomic::Ordering::SeqCst);
        self.workers.insert(worker_id, WorkerState::Available);

        let mut request_stream = request.into_inner();
        while let Some(req) = request_stream.next().await {
            let Ok(ack) = req else {
                panic!("error reading work request");
            };

            debug!("Got ack for work request {}", ack.task_id);
            let task_id = WorkId { worker_id, task_id: ack.task_id };
            self.dispatcher.complete_work(task_id);
        }

        todo!();
    }
}
