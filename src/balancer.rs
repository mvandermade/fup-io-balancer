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
use ::futures::pin_mut;
use ::futures::select;
use ::futures::FutureExt;
use ::log::debug;
use ::log::info;
use ::log::warn;
use ::std::sync::Arc;
use tokio::time;

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
    pub async fn run(mut self, no_worker_delay_us: u64) -> ! {
        info!("Going to wait for postzegel events");
        loop {
            let f1 = self.source.receive().fuse();
            let f2 = self.backlog_source.receive().fuse();
            pin_mut!(f1, f2);
            let (event, idempotency_id) = select! {
                fresh = f1 => if let Some(fresh) = fresh { (fresh, None) } else { continue },
                backlog = f2 => if let Some(backlog) = backlog { backlog } else { continue },
                complete => panic!("Balancer source and backlog closed"),
            };
            //TODO @mark: does this indeed stop if both are closed?
            debug!("Got a postzegel event {}", event);
            let handler = TaskFailureHandler::new(event.clone(), self.backlog_sink.fork());
            let assignment = self.dispatcher.try_assign(event.code_str(), handler, idempotency_id).await;
            if let AssignResult::Assigned(work_id) = assignment {
                debug!(
                    "Event {} ({event}) assigned to worker {}",
                    work_id.task_id, work_id.worker_id
                );
            } else {
                debug!("Event ({event}) not assigned, send to backlog and waiting {} ms", no_worker_delay_us / 1000);
                if let Err(((event, _), err)) = self.backlog_sink.try_send((event, None)) {
                    warn!("Backlog is full or closed, rejecting event {event} (err: {err})");
                    //TODO @mark: metric
                };
                time::sleep(time::Duration::from_micros(no_worker_delay_us)).await;
            }
        }
    }
}
