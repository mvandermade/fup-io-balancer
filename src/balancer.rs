use crate::dispatcher::{Dispatcher, WorkAssignment};
use crate::postzegel_event::PostzegelEvent;
use ::crossbeam_channel::Receiver;
use ::log::debug;
use ::log::info;
use ::std::sync::Arc;

#[derive(Debug)]
pub struct Balancer {
    pub source: Receiver<PostzegelEvent>,
    dispatcher: Arc<Dispatcher>,
    backlog: (), //TODO @mark:
}

impl Balancer {
    pub fn new(source: Receiver<PostzegelEvent>, dispatcher: Arc<Dispatcher>) -> Self {
        Balancer { source, dispatcher, backlog: () }
    }
}

impl Balancer {
    pub fn run(&self) -> ! {
        info!("Going to wait for postzegel events");
        loop {
            match self.source.recv() {
                Ok(event) => {
                    debug!("Got a postzegel event {}", event);
                    self.dispatcher.try_assign(event.code_str())
                },
                Err(_) => panic!("channel disconnected, cannot get more events"),
            }
        }
    }
}
