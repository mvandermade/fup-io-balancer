use crate::channel::channel;
use crate::channel::Fork;
use crate::channel::Sink;
use crate::channel::Source;
use crate::global::ChannelKey;
use ::dashmap::DashMap;
use ::std::fmt::Debug;
use ::tokio::sync::Mutex;

pub type WorkerId = u32;

#[derive(Debug)]
pub struct Workers<T: Fork + Debug> {
    available_source: Mutex<Source<(WorkerId, T)>>,
    available_sink: Sink<(WorkerId, T)>,
    busy: DashMap<WorkerId, T>,
    //TODO @mark: ^ use better dequeue
}

impl <T: Fork + Debug> Workers<T> {
    pub fn new() -> Workers<T> {
        let (sink, source) = channel(1024, ChannelKey::AvailableWorkers);
        Workers {
            available_source: Mutex::new(source),
            available_sink: sink,
            busy: DashMap::with_capacity(1024),
        }
    }

    pub async fn add_new(&self, worker: WorkerId, data: T) {
        assert!(!self.busy.contains_key(&worker));
        self.available_sink.send((worker, data))
            .await.expect("Available worker channel closed");
    }
    pub fn remove(&self, worker: WorkerId) {
        self.busy.remove(&worker);
        //TODO @mark: also remove from available
    }

    pub async fn mark_ready(&self, worker: WorkerId) {
        let existing = self.busy.remove(&worker);
        if let Some(worker_data) = existing {
            self.available_sink.send(worker_data)
                .await.expect("Available worker channel closed");
        } else {
            panic!("Try to mark a worker as ready that is not busy");
        }
    }

    pub async fn find_available(&self) -> (WorkerId, T) {
        let maybe_worker_data = self.available_source.lock().await.receive().await;
        if let Some((worker_id, data)) = maybe_worker_data {
            let existing = self.busy.insert(worker_id, data.fork());
            if let Some(_) = existing {
                panic!("Worker {} was already busy", worker_id);
            }
            return (worker_id, data)
        }
        panic!("Available worker channel closed");
    }
}