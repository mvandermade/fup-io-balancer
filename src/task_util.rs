use log::warn;
use crate::channel::Sink;
use crate::postzegel::PostzegelEvent;

#[derive(Debug)]
pub enum FailReason {
    Disconnect,
    Timeout,  //TODO @mark:
    WorkerError(String),
    ServerError(String),
}

#[derive(Debug)]
pub struct TaskFailureHandler {
    event: PostzegelEvent,
    sink: Sink<(PostzegelEvent, Option<u64>)>,
}

impl TaskFailureHandler {
    pub fn new(event: PostzegelEvent, sink: Sink<(PostzegelEvent, Option<u64>)>) -> Self {
        TaskFailureHandler { event, sink }
    }
}

impl TaskFailureHandler {
    pub async fn fail_task(self, idempotency_id: u64) {
        if let Err(((event, _), err)) = self.sink.try_send((self.event, Some(idempotency_id))) {
            warn!("Could not re-add failed event {event} to backlog, err: {err}")
            //TODO @mark: metrics?
        }
    }
}
