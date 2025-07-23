use crate::channel::{channel, Fork};
use crate::channel::Sink;
use crate::channel::Source;
use crate::dispatcher::AssignResult;
use crate::dispatcher::Dispatcher;
use crate::global::ChannelKey;
use crate::postzegel::PostzegelEvent;
use crate::task_util::TaskFailureHandler;
use ::log::debug;
use ::log::info;
use ::log::warn;
use ::std::sync::Arc;

const BACKLOG_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Balancer {
    pub source: Source<PostzegelEvent>,
    dispatcher: Arc<Dispatcher>,
    //TODO @mark: ^ can Arc be removed once rpc doesn't have access to dispatcher?
    backlog_sink: Sink<(PostzegelEvent, Option<u64>)>,
    backlog_source: Source<(PostzegelEvent, Option<u64>)>,
    // idempotency id, in case the backlog entry is a retry ^
}

impl Balancer {
    pub fn new(source: Source<PostzegelEvent>, dispatcher: Arc<Dispatcher>) -> Self {
        let (backlog_sink, backlog_source) = channel(BACKLOG_SIZE, ChannelKey::BalancerBacklog);
        Balancer {
            source,
            dispatcher,
            backlog_sink,
            backlog_source,
        }
    }
}

impl Balancer {
    pub async fn run(mut self) -> ! {
        info!("Going to wait for postzegel events");
        while let Some(event) = self.source.receive().await {
            debug!("Got a postzegel event {}", event);
            let handler = TaskFailureHandler::new(event.clone(), self.backlog_sink.fork());
            let assignment = self.dispatcher.try_assign(event.code_str(), handler, idempotency_id).await;
            if let AssignResult::Assigned(work_id) = assignment {
                debug!(
                    "Event {} ({event}) assigned to worker {}",
                    work_id.task_id, work_id.worker_id
                );
            } else {
                debug!("Event ({event}) not assigned, send to backlog");
                if let Err(((event, _), err)) = self.backlog_sink.try_send((event, None)) {
                    warn!("Backlog is full or closed, rejecting event {event} (err: {err})");
                    //TODO @mark: metric
                };
            }
        }
        //TODO @mark: drain queue here
        panic!("Scanner channel closed, existing balancer")
    }
}
