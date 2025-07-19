use std::collections::VecDeque;

type WorkerId = u32;

#[derive(Debug)]
pub struct Workers {
    available: VecDeque<WorkerId>,
    busy: VecDeque<WorkerId>,
    //TODO @mark: ^ use better dequeue
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            available: VecDeque::with_capacity(1024),
            busy: VecDeque::with_capacity(1024),
        }
    }

    pub fn add(&mut self, worker: WorkerId) {
        self.available.push_back(worker);
    }
}
