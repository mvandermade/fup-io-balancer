use crate::postzegel_event::PostzegelEvent;
use ::crossbeam_channel::Receiver;
use ::log::debug;
use ::log::info;

#[derive(Debug)]
pub struct Balancer {
    pub source: Receiver<PostzegelEvent>,
}

impl Balancer {
    pub fn new(source: Receiver<PostzegelEvent>) -> Self {
        Balancer { source }
    }
}

impl Balancer {
    pub fn run(&self) -> ! {
        info!("Going to wait for postzegel events");
        loop {
            match self.source.recv() {
                Ok(event) => {
                    debug!("Got a postzegel event {}", event)
                },
                Err(_) => panic!("channel disconnected, cannot get more events"),
            }
        }
    }
}
