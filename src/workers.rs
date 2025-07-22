use ::dashmap::DashMap;
use ::std::collections::VecDeque;

pub type WorkerId = u32;

#[derive(Debug)]
pub struct Workers<T> {
    available: VecDeque<(WorkerId, T)>,
    busy: DashMap<WorkerId, T>,
    //TODO @mark: ^ use better dequeue
}

impl <T: Clone> Workers<T> {
    pub fn new() -> Workers<T> {
        Workers {
            available: VecDeque::with_capacity(1024),
            busy: DashMap::with_capacity(1024),
        }
    }

    pub fn add_new(&mut self, worker: WorkerId, data: T) {
        assert!(!self.busy.contains_key(&worker));
        self.available.push_back((worker, data));
    }
    pub fn remove(&mut self, worker: WorkerId) {
        self.busy.remove(&worker);
        //TODO @mark: also remove from available
    }

    pub fn mark_ready(&mut self, worker: WorkerId) {
        let existing = self.busy.remove(&worker);
        assert!(existing.is_some(), "try to mark a worker as ready that is not busy");
    }

    pub fn find_available(&mut self) -> Option<(WorkerId, T)> {
        let maybe_worker_data = self.available.pop_front();
        if let Some((worker, data)) = maybe_worker_data {
            self.busy.insert(worker, data.clone());
            return Some((worker, data))
        }
        None
    }
}
