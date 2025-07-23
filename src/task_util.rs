
#[derive(Debug)]
pub enum FailReason {
    Disconnect,
    Timeout,  //TODO @mark:
    Error(String),
}

#[derive(Debug)]
pub struct TaskFailureHandler {
    task_id: u64,
}

impl TaskFailureHandler {
    pub async fn fail_task(self, reason: FailReason) {
        todo!()
    }
}
