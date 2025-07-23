use crate::channel::Sink;
use crate::postzegel::PostzegelEvent;

#[derive(Debug)]
pub enum FailReason {
    Disconnect,
    Timeout,  //TODO @mark:
    Error(String),
}

#[derive(Debug)]
pub struct TaskFailureHandler {
    sink: Sink<(PostzegelEvent, Option<u64>)>,
}

impl TaskFailureHandler {
    pub fn new(sink: Sink<(PostzegelEvent, Option<u64>)>) -> Self {
        TaskFailureHandler { sink }
    }
}

impl TaskFailureHandler {
    pub async fn fail_task(self, reason: FailReason) {
        todo!()
    }
}
