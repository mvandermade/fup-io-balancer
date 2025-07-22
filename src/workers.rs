use crate::channel::Sink;
use ::dashmap::DashMap;
use ::std::collections::VecDeque;
use ::std::fmt::Debug;

pub type WorkerId = u32;

#[derive(Debug)]
pub struct Workers<T: Debug> {
    available: VecDeque<(WorkerId, Sink<T>)>,
    busy: DashMap<WorkerId, Sink<T>>,
    //TODO @mark: ^ use better dequeue
}

impl <T: Debug> Workers<T> {
    pub fn new() -> Workers<T> {
        Workers {
            available: VecDeque::with_capacity(1024),
            busy: DashMap::with_capacity(1024),
        }
    }

    pub fn add_new(&mut self, worker: WorkerId, data: Sink<T>) {
        assert!(!self.busy.contains_key(&worker));
        self.available.push_back((worker, data));
    }
    pub fn remove(&mut self, worker: WorkerId) {
        self.busy.remove(&worker);
        //TODO @mark: also remove from available
    }

    pub fn mark_ready(&mut self, worker: WorkerId) {
        let existing = self.busy.remove(&worker);
        if let Some(worker_data) = existing {
            self.available.push_back(worker_data);
        } else {
            panic!("Try to mark a worker as ready that is not busy");
        }
    }

    pub fn find_available(&mut self) -> Option<(WorkerId, Sink<T>)> {
        let maybe_worker_data = self.available.pop_front();
        if let Some((worker_id, data)) = maybe_worker_data {
            let existing = self.busy.insert(worker_id, data.fork());
            if let Some(_) = existing {
                panic!("Worker {} was already busy", worker_id);
            }
            return Some((worker_id, data))
        }
        None
    }
}
