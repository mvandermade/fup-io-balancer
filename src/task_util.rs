use crate::channel::Sink;
use crate::postzegel::PostzegelEvent;
use ::log::warn;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdemId {
    id: u64,
}

impl IdemId {
    pub fn new(id: u64) -> IdemId {
        IdemId { id }
    }

    pub fn as_number(&self) -> u64 {
        self.id
    }
}

#[derive(Debug)]
pub enum FailReason {
    Disconnect,
    Timeout,  //TODO @mark:
    WorkerError(String),
}

#[derive(Debug)]
pub struct TaskFailureHandler {
    event: PostzegelEvent,
    sink: Sink<(PostzegelEvent, Option<IdemId>)>,
}

impl TaskFailureHandler {
    pub fn new(event: PostzegelEvent, sink: Sink<(PostzegelEvent, Option<IdemId>)>) -> Self {
        TaskFailureHandler { event, sink }
    }
}

impl TaskFailureHandler {
    pub async fn fail_task(self, idempotency_id: IdemId) {
        if let Err(((event, _), err)) = self.sink.try_send((self.event, Some(idempotency_id))) {
            warn!("Could not re-add failed event {event} to backlog, err: {err}")
            //TODO @mark: metrics?
        }
    }
}
