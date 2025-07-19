use crate::rpc::balancer_svc_server::BalancerSvc;
use ::dashmap::DashMap;
use ::futures::StreamExt;
use ::log::debug;
use ::log::error;
use ::log::info;
use ::std::collections::VecDeque;
use ::std::pin::Pin;
use ::std::sync::atomic;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use ::std::sync::Arc;
use ::tokio::sync::Mutex;
use ::tonic::IntoRequest;
use crate::workers::{WorkerId, Workers};

tonic::include_proto!("balancerapi");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkId {
    pub(crate) worker_id: u32,
    pub(crate) task_id: u64,
}

/// stores data about workers and in-flight tasks (but not backlog)
#[derive(Debug)]
pub struct Dispatcher {
    top_worker_id: AtomicU32,
    top_task_id: AtomicU64,
    workers: Arc<Mutex<Workers>>,
    in_flight: DashMap<WorkId, ()>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            top_worker_id: AtomicU32::new(0),
            top_task_id: AtomicU64::new(0),
            workers: Arc::new(Mutex::new(Workers::new())),
            in_flight: Default::default()
        }
    }

    pub async fn new_worker(&self) -> WorkerId {
        let worker_id = self.top_worker_id.fetch_add(1, atomic::Ordering::Relaxed);
        self.workers.lock().await.add_new(worker_id);
        worker_id
    }

    pub async fn complete_work(&self, work_id: WorkId) {
        let ongoing_task = self.in_flight.remove(&work_id);
        if ongoing_task.is_some() {
            debug!("Got ack for work request {} by worker {}", work_id.task_id, work_id.worker_id);
            self.workers.lock().await.mark_ready(work_id.worker_id);
        } else {
            error!("Got ack for work request {} that we not in progress by worker {}", work_id.task_id, work_id.worker_id);
            //TODO @mark: better error handling
        }
    }

    /// Timeout or error
    pub async fn fail_work(&self, task_id: WorkId) {
        //TODO @mark: impl & call this
    }

    pub async fn try_assign(&self, postzegel_code: String) -> AssignResult {
        let task_id = self.top_task_id.fetch_add(1, atomic::Ordering::Relaxed);
        let work = WorkAssignment {
            task_id,
            idempotency_id: task_id,
            //TODO @mark: ^ make sure this stays the same if task times out or fails
            postzegel_code,
        };
        let Some(worker) = self.workers.lock().await.find_available() else {
            debug!("No workers available for task {task_id}");
            return AssignResult::NoWorkers;
        };
        let work_id = WorkId { worker_id: worker, task_id };
        self.in_flight.insert(work_id, ());
        todo!("send to rpc");  //TODO @mark: TEMPORARY! REMOVE THIS!
        AssignResult::Assigned
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignResult {
    Assigned,
    NoWorkers,
}
