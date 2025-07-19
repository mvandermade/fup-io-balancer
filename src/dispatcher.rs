use crate::rpc::balancer_svc_server::BalancerSvc;
use ::dashmap::DashMap;
use ::futures::StreamExt;
use ::log::debug;
use ::log::error;
use ::log::info;
use ::std::pin::Pin;
use ::std::sync::atomic;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use ::tonic::IntoRequest;

tonic::include_proto!("balancerapi");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WorkId {
    worker_id: u32,
    task_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WorkerState {
    Busy,
    Available,
}

/// stores data about workers and in-flight tasks (but not backlog)
#[derive(Debug)]
pub struct Dispatcher {
    top_worker_id: AtomicU32,
    top_task_id: AtomicU64,
    //TODO @mark: split into busy queue and idle queue
    workers: DashMap<u32, WorkerState>,
    in_flight: DashMap<WorkId, ()>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            top_worker_id: AtomicU32::new(0),
            top_task_id: AtomicU64::new(0),
            workers: DashMap::with_capacity(16),
            in_flight: Default::default()
        }
    }

    fn complete_work(&self, work_id: WorkId) {
        let ongoing_task = self.in_flight.remove(&work_id);
        if ongoing_task.is_some() {
            debug!("Got ack for work request {} by worker {worker_id}", work_id.task_id);
            self.workers.insert(work_id.worker_id, WorkerState::Available);
        } else {
            error!("Got ack for work request {} that we not in progress by worker {worker_id}", work_id.task_id);
            //TODO @mark: better error handling
        }
    }

    fn timeout_work(&self, task_id: WorkId) {
        //TODO @mark: impl & call this
    }

    fn try_assign(&self, task: WorkAssignment) -> Option<()> {
        todo!()
    }
}
