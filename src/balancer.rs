use crate::channel::channel;
use crate::channel::Fork;
use crate::channel::Sink;
use crate::channel::Source;
use crate::dispatcher::AssignResult;
use crate::dispatcher::Dispatcher;
use crate::global::ChannelKey;
use crate::postzegel::PostzegelEvent;
use crate::task_util::IdemId;
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
    backlog_sink: Sink<(PostzegelEvent, Option<IdemId>)>,
    backlog_source: Source<(PostzegelEvent, Option<IdemId>)>,
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
        let sink_copy = self.backlog_sink.fork();
        tokio::spawn(async move {
            while let Some(event) = self.source.receive().await {
                debug!("Got a postzegel event {}", event);
                sink_copy.try_send((event, None)).unwrap();
            }
            panic!("Postzegel source closed");
        });
        while let Some((event, idempotency_id)) = self.backlog_source.receive().await {
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
        panic!("Postzegel backlog closed");
    }
}
