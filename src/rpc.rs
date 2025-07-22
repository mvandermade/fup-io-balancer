pub use self::proto::balancer_svc_server::BalancerSvc;
pub use self::proto::balancer_svc_server::BalancerSvcServer;
pub use self::proto::WorkAcknowledgement;
pub use self::proto::WorkAssignment;

use crate::dispatcher::Dispatcher;
use crate::dispatcher::FailReason;
use crate::dispatcher::WorkId;
use crate::channel::channel;
use ::futures::StreamExt;
use ::log::debug;
use ::log::info;
use ::log::trace;
use ::log::warn;
use ::std::pin::Pin;
use ::std::sync::Arc;
use ::tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use crate::global::ChannelKey;

mod proto {
    #![allow(non_camel_case_types)]
    tonic::include_proto!("balancerapi");
}

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

    type workStream = Pin<Box<dyn futures::Stream<Item=Result<WorkAssignment, tonic::Status>> + Send + 'static>>;

    async fn work(&self, request: tonic::Request<tonic::Streaming<WorkAcknowledgement>>) -> Result<tonic::Response<Self::workStream>, tonic::Status> {
        let (task_sender, task_receiver) = channel::<WorkAssignment>(1, ChannelKey::Assignments);
        let worker_id = self.dispatcher.new_worker(task_sender).await;

        let dispatcher_clone = self.dispatcher.clone();
        let mut request_stream = request.into_inner();

        tokio::spawn(async move {
            debug!("Starting work rpc acknowledge listening stream for worker {}", worker_id);
            while let Some(req) = request_stream.next().await {
                match req {
                    Ok(ack) => {
                        trace!("Got ack for work request {}", ack.task_id);
                        let task_id = WorkId { worker_id, task_id: ack.task_id };
                        if ack.error.is_empty() {
                            dispatcher_clone.complete_work(task_id).await;
                        } else {
                            dispatcher_clone.fail_work(task_id, FailReason::Error(ack.error)).await;
                        }
                    }
                    Err(err) => {
                        warn!("Could not read message from worker {worker_id}, will be unregistered ({err}) (this often just means the client disconnected)");
                        dispatcher_clone.remove_worker(worker_id).await;
                        break;
                    }
                }
            }
            info!("Empty work rpc stream for worker {}", worker_id);
        });

        debug!("Starting work rpc task sending stream for worker {}", worker_id);
        let outbound_stream = ReceiverStream::new(task_receiver.expose_receiver());
        Ok(tonic::Response::new(Box::pin(outbound_stream.map(Ok))))
    }

}