use crate::channel::{channel, Sink, Source};
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
    backlog_source: Sink<(PostzegelEvent, Option<u64>)>,
    backlog_sink: Source<(PostzegelEvent, Option<u64>)>,
    //TODO @mark: handle backlog items
}

impl Balancer {
    pub fn new(source: Source<PostzegelEvent>, dispatcher: Arc<Dispatcher>) -> Self {
        let (backlog_source, backlog_sink) = channel(BACKLOG_SIZE, ChannelKey::BalancerBacklog);
        Balancer {
            source,
            dispatcher,
            backlog_source,
            backlog_sink,
        }
    }
}

impl Balancer {
    pub async fn run(mut self) -> ! {
        info!("Going to wait for postzegel events");
        while let Some(event) = self.source.receive().await {
            debug!("Got a postzegel event {}", event);
            let handler = TaskFailureHandler {};
            let assignment = self.dispatcher.try_assign(event.code_str()).await;
            if let AssignResult::Assigned(work_id) = assignment {
                debug!(
                    "Event {} ({event}) assigned to worker {}",
                    work_id.task_id, work_id.worker_id
                );
            } else {
                debug!("Event ({event}) not assigned, send to backlog");
                if self.backlog.len() < BACKLOG_SIZE {
                    self.backlog.push_back(event);
                } else {
                    warn!("Backlog is full, rejecting event {event}");
                }
            }
        }
        //TODO @mark: drain queue here
        panic!("Scanner channel closed, existing balancer")
    }
}
