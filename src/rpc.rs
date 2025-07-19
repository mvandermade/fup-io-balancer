use crate::rpc::balancer_svc_server::BalancerSvc;
use ::dashmap::DashMap;
use ::futures::StreamExt;
use ::log::debug;
use ::log::info;
use ::std::pin::Pin;
use ::std::sync::atomic;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use ::tonic::IntoRequest;
use log::error;

tonic::include_proto!("balancerapi");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TaskId {
    worker_id: u32,
    task_id: u64,
}

#[derive(Debug)]
pub struct BalancerRpc {
    top_worker_id: AtomicU32,
    top_task_id: AtomicU64,
    workers: DashMap<u32, ()>,
    in_flight: DashMap<TaskId, ()>,
}

impl BalancerRpc {
    pub fn new() -> Self {
        BalancerRpc {
            top_worker_id: AtomicU32::new(0),
            top_task_id: AtomicU64::new(0),
            workers: DashMap::with_capacity(16),
            in_flight: Default::default()
        }
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

        let mut request_stream = request.into_inner();
        while let Some(req) = request_stream.next().await {
            let Ok(ack) = req else {
                panic!("error reading work request");
            };

            debug!("Got ack for work request {}", ack.task_id);
            let ongoing_task = self.in_flight.remove(&TaskId { worker_id, task_id: ack.task_id });
            if ongoing_task.is_none() {
                error!("Got ack for work request {} that we not in progress by worker {worker_id}", ack.task_id);
            } else {
                debug!("Got ack for work request {} by worker {worker_id}", ack.task_id);
            }
            //TODO @mark: make sure we get ack for each work request, otherwise re-send
        }

        todo!();
    }
}
