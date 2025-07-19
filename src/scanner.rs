#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use crate::postzegel_event::PostzegelEvent;
use ::crossbeam_channel::Sender;
use ::std::fmt::Debug;
use ::std::path::PathBuf;

#[derive(Debug)]
pub enum Scanner {
    Real(RealScanner),
    Mock(MockScanner),
}

impl Scanner {
    pub fn run(&self) -> ! {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct RealScanner {
    pub address: PathBuf,
    pub sink: Sender<PostzegelEvent>,
}

#[derive(Debug)]
pub struct MockScanner {
    pub sink: Sender<PostzegelEvent>,
}

