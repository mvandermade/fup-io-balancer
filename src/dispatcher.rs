use crate::rpc::WorkAssignment;
use crate::workers::WorkerId;
use crate::workers::Workers;
use ::dashmap::DashMap;
use ::log::debug;
use ::log::error;
use ::log::info;
use ::std::fmt;
use ::std::sync::atomic;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use ::std::sync::Arc;
use ::tokio::sync::mpsc::Sender;
use ::tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkId {
    pub worker_id: u32,
    pub task_id: u64,
}

/// stores data about workers and in-flight tasks (but not backlog)
#[derive(Debug)]
pub struct Dispatcher {
    top_worker_id: AtomicU32,
    top_task_id: AtomicU64,
    workers: Arc<Mutex<Workers<Sender<WorkAssignment>>>>,
    in_flight: DashMap<WorkId, ()>,
}

#[derive(Debug)]
pub enum FailReason {
    Disconnect,
    Timeout,
    Error(String),
}

impl fmt::Display for FailReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FailReason::Disconnect => write!(f, "worker disconnected"),
            FailReason::Timeout => write!(f, "task timed out"),
            FailReason::Error(msg) => write!(f, "failed: {}", msg),
        }
    }
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            top_worker_id: AtomicU32::new(0),
            top_task_id: AtomicU64::new(0),
            workers: Arc::new(Mutex::new(Workers::new())),
            in_flight: DashMap::new(),
        }
    }

    pub async fn new_worker(&self, task_sender: Sender<WorkAssignment>) -> WorkerId {
        let worker_id = self.top_worker_id.fetch_add(1, atomic::Ordering::Relaxed);
        self.workers.lock().await.add_new(worker_id, task_sender);
        worker_id
    }

    pub async fn remove_worker(&self, worker_id: WorkerId) {
        self.workers.lock().await.remove(worker_id);
        let ongoing_task = self.in_flight.remove(&WorkId { worker_id, task_id: 0 });
        if let Some((task, _)) = ongoing_task {
            self.fail_work(task, FailReason::Disconnect).await;
        }
    }

    pub async fn complete_work(&self, work_id: WorkId) {
        let ongoing_task = self.in_flight.remove(&work_id);
        if ongoing_task.is_some() {
            debug!("Got ack for work request {} by worker {}", work_id.task_id, work_id.worker_id);
            self.workers.lock().await.mark_ready(work_id.worker_id);
        } else {
            error!("Got ack for work request {} that we not in progress by worker {} \
                (it might have timed out)", work_id.task_id, work_id.worker_id);
            //TODO @mark: better error handling
        }
    }

    pub async fn fail_work(&self, task_id: WorkId, reason: FailReason) {
        self.in_flight.remove(&task_id);
        info!("Task {} for worker {} failed: {}", task_id.task_id, task_id.worker_id, reason);
        //TODO @mark: add it back to queue
        //TODO @mark: impl timeout
    }

    pub async fn try_assign(&self, postzegel_code: String) -> AssignResult {
        let task_id = self.top_task_id.fetch_add(1, atomic::Ordering::Relaxed);
        let work = WorkAssignment {
            task_id,
            idempotency_id: task_id,
            //TODO @mark: ^ make sure this stays the same if task times out or fails
            postzegel_code,
        };
        let Some((worker, sender)) = self.workers.lock().await.find_available() else {
            debug!("No workers available for task {task_id}");
            return AssignResult::NoWorkers;
        };
        let work_id = WorkId { worker_id: worker, task_id };
        self.in_flight.insert(work_id, ());
        sender.send(work).await.expect("Failed to send work assignment to grpc channel");
        AssignResult::Assigned(work_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignResult {
    Assigned(WorkId),
    NoWorkers,
}