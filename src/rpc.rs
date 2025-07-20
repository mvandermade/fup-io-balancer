use crate::dispatcher::Dispatcher;
use crate::dispatcher::WorkId;
use crate::rpc::balancer_svc_server::BalancerSvc;
use ::dashmap::DashMap;
use ::futures::StreamExt;
use ::log::debug;
use ::log::error;
use ::log::info;
use ::std::pin::Pin;
use ::std::sync::atomic;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use ::std::sync::Arc;
use ::tokio::sync::mpsc::channel;
use ::tokio::task;
use ::tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use ::tonic::IntoRequest;
use ::tonic::Status;
use log::warn;

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

    type workStream = Pin<Box<dyn futures::Stream<Item=Result<WorkAssignment, tonic::Status>> + Send + 'static>>;

    async fn work(&self, request: tonic::Request<tonic::Streaming<WorkAcknowledgement>>) -> Result<tonic::Response<Self::workStream>, tonic::Status> {
        let (task_sender, task_receiver) = channel::<WorkAssignment>(1);
        let worker_id = self.dispatcher.new_worker(task_sender).await;

        let dispatcher_clone = self.dispatcher.clone();
        task::spawn(async move {
            debug!("Starting work rpc acknowledge listening stream for worker {}", worker_id);
            let mut request_stream = request.into_inner();
            while let Some(req) = request_stream.next().await {
                let ack = match req {
                    Ok(ack) => ack,
                    Err(err) => {
                        warn!("Could not read message from worker {worker_id}, it might have disconnected and will be unregistered ({err})");
                        let worker_id = self.dispatcher.remove_worker(worker_id).await;
                        //TODO @mark: what if it was busy
                        break;
                    },
                };

                debug!("Got ack for work request {}", ack.task_id);
                let task_id = WorkId { worker_id, task_id: ack.task_id };
                if ack.error.is_empty() {
                    dispatcher_clone.complete_work(task_id);
                } else {
                    dispatcher_clone.fail_work(task_id);
                }
            }
            info!("Empty work rpc stream for worker {}", worker_id);
        });

        debug!("Starting work rpc task sending stream for worker {}", worker_id);
        let outbound_stream = ReceiverStream::new(task_receiver);
        Ok(tonic::Response::new(Box::pin(outbound_stream.map(|work| Ok(work)))))
    }
}