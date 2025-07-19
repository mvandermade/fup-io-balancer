use crate::dispatcher::Dispatcher;
use crate::dispatcher::AssignResult;
use crate::postzegel::PostzegelEvent;
use ::crossbeam_channel::Receiver;
use ::log::debug;
use ::log::info;
use ::std::collections::VecDeque;
use ::std::sync::Arc;
use ::tonic::async_trait;

#[derive(Debug)]
pub struct Balancer {
    pub source: Receiver<PostzegelEvent>,
    dispatcher: Arc<Dispatcher>,
    backlog: VecDeque<PostzegelEvent>,
    //TODO @mark: handle backlog items
}

impl Balancer {
    pub fn new(source: Receiver<PostzegelEvent>, dispatcher: Arc<Dispatcher>) -> Self {
        Balancer { source, dispatcher, backlog: VecDeque::with_capacity(1024), }
    }
}

impl Balancer {
    pub async fn run(&mut self) -> ! {
        info!("Going to wait for postzegel events");
        loop {
            match self.source.recv() {
                Ok(event) => {
                    debug!("Got a postzegel event {}", event);
                    let assignment = self.dispatcher.try_assign(event.code_str()).await;
                    if let AssignResult::Assigned(work_id) = assignment {
                        debug!("Event {} assigned to worker {}", work_id.worker_id, event);
                    } else {
                        debug!("Event {} not assigned, send to backlog", event);
                        self.backlog.push_back(event)
                    }
                },
                Err(_) => panic!("channel disconnected, cannot get more events"),
            }
        }
    }
}
