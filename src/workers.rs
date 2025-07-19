use std::collections::{HashMap, VecDeque};
use dashmap::DashMap;

pub type WorkerId = u32;

#[derive(Debug)]
pub struct Workers {
    available: VecDeque<WorkerId>,
    busy: DashMap<WorkerId, ()>,
    //TODO @mark: ^ use better dequeue
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            available: VecDeque::with_capacity(1024),
            busy: DashMap::with_capacity(1024),
        }
    }

    pub fn add_new(&mut self, worker: WorkerId) {
        assert!(!self.busy.contains_key(&worker));
        self.available.push_back(worker);
    }

    pub fn mark_ready(&mut self, worker: WorkerId) {
        let existing = self.busy.remove(&worker);
        assert!(existing.is_some(), "try to mark a worker as ready that is not busy");
    }

    pub fn find_available(&mut self) -> Option<WorkerId> {
        let worker = self.available.pop_front();
        if let Some(worker) = worker {
            self.busy.insert(worker, ());
        }
        worker
    }
}
