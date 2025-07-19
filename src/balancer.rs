use crate::scanner::PostzegelEvent;
use ::crossbeam_channel::Receiver;

#[derive(Debug)]
pub struct Balancer {
    pub sink: Receiver<PostzegelEvent>,
}