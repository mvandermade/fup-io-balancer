use crate::channel::Sink;
use crate::rpc::WorkAssignment;
use crate::task_util::IdemId;
use crate::task_util::FailReason;
use crate::task_util::TaskFailureHandler;
use crate::workers::WorkerId;
use crate::workers::Workers;
use ::dashmap::DashMap;
use ::log::debug;
use ::log::error;
use ::log::info;
use ::log::warn;
use ::std::fmt;
use ::std::sync::atomic;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkId {
    pub worker_id: u32,
    pub task_id: u64,
}

/// stores data about workers and in-flight tasks (but not backlog)
pub struct Dispatcher {
    top_worker_id: AtomicU32,
    top_task_id: AtomicU64,
    workers: Workers<Sink<WorkAssignment>>,
    in_flight: DashMap<WorkId, (IdemId, TaskFailureHandler)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignResult {
    Assigned(WorkId),
    NoWorkers,
    Error(String),
}

impl fmt::Display for FailReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FailReason::Disconnect => write!(f, "worker disconnected"),
            FailReason::Timeout => write!(f, "task timed out"),
            FailReason::WorkerError(msg) => write!(f, "failed: {}", msg),
        }
    }
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            top_worker_id: AtomicU32::new(0),
            top_task_id: AtomicU64::new(0),
            workers: Workers::new(),
            in_flight: DashMap::new(),
        }
    }

    pub async fn new_worker(&self, task_sender: Sink<WorkAssignment>) -> WorkerId {
        let worker_id = self.top_worker_id.fetch_add(1, atomic::Ordering::Relaxed);
        self.workers.add_new(worker_id, task_sender).await;
        worker_id
    }

    pub async fn remove_worker(&self, worker_id: WorkerId) {
        self.workers.remove(worker_id);
        let ongoing_task = self.in_flight.remove(&WorkId { worker_id, task_id: 0 });
        if let Some((task, _)) = ongoing_task {
            self.fail_work(task, FailReason::Disconnect).await;
        }
    }

    pub async fn complete_work(&self, work_id: WorkId) {
        let ongoing_task = self.in_flight.remove(&work_id);
        if let Some(_) = ongoing_task {
            debug!("Got ack for work request {} by worker {}", work_id.task_id, work_id.worker_id);
            self.workers.mark_ready(work_id.worker_id).await;
        } else {
            error!("Got ack for work request {} that we not in progress by worker {} \
                (it might have timed out)", work_id.task_id, work_id.worker_id);
            //TODO @mark: better error handling
        }
    }

    pub async fn fail_work(&self, task_id: WorkId, reason: FailReason) {
        let existing = self.in_flight.remove(&task_id);
        if let Some((_, (idempotency_id, failure_handler))) = existing {
            info!("Task {} for worker {} failed: {}", task_id.task_id, task_id.worker_id, reason);
            self.workers.mark_ready(task_id.worker_id).await;
            failure_handler.fail_task(idempotency_id).await;
        } else {
            warn!("Task {} for worker {} marked as failed, but was not found: {}", task_id.task_id, task_id.worker_id, reason);
        }
    }

    pub async fn try_assign(&self, postzegel_code: String, handler: TaskFailureHandler, idempotency_id: Option<IdemId>) -> AssignResult {
        // idempotency is set if this is a retry, we use the same id again to detect duplicates
        let task_id = self.top_task_id.fetch_add(1, atomic::Ordering::Relaxed);
        let idempotency_id = idempotency_id.unwrap_or(IdemId::new(task_id));
        let work = WorkAssignment {
            task_id,
            idempotency_id: idempotency_id.as_number(),
            //TODO @mark: ^ make sure this stays the same if task times out or fails
            postzegel_code,
        };
        //TODO @mark: add a timeout here
        let (worker, sender) = self.workers.find_available().await;
        // todo return AssignResult::NoWorkers;
        let work_id = WorkId { worker_id: worker, task_id };
        self.in_flight.insert(work_id, (idempotency_id, handler));
        match sender.send(work).await {
            Ok(()) => AssignResult::Assigned(work_id),
            Err(err) => {
                // The channel is probably closed, but the next attempt might get a different worker, so try again
                if let Some((_, (idempotency_id, handler))) = self.in_flight.remove(&work_id) {
                    warn!("Failed to send work request {} to worker {}: {}", task_id, worker, err);
                    handler.fail_task(idempotency_id).await;
                } else {
                    warn!("Failed to send work request {} to worker {}, and could not find failure handler: {}", task_id, worker, err);
                }
                AssignResult::Error(err.to_string())
            },
        }
    }
}
