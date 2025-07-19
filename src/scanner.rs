#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use crate::postzegel_event::PostzegelEvent;
use ::crossbeam_channel::Sender;
use ::std::fmt::Debug;
use ::std::path::PathBuf;
use ::std::thread::sleep;
use ::log::debug;

#[derive(Debug)]
pub enum Scanner {
    Real(RealScanner),
    Mock(MockScanner),
}

impl Scanner {
    pub fn run(&self) -> ! {
        match self {
            Scanner::Real(scanner) => unimplemented!(),
            Scanner::Mock(scanner) => scanner.run(),
        }
    }
}

#[derive(Debug)]
pub struct RealScanner {
    pub address: PathBuf,
    pub sink: Sender<PostzegelEvent>,
}

#[derive(Debug)]
pub struct MockScanner {
    pub name: String,
    pub sink: Sender<PostzegelEvent>,
}

impl MockScanner {
    pub fn new(name: String, sink: Sender<PostzegelEvent>) -> Scanner {
        Scanner::Mock(MockScanner { name, sink })
    }

    pub fn run(&self) -> ! {
        let mut seq: u64 = 0;
        loop {
            debug!("sending mock event {seq}");
            self.sink.send(PostzegelEvent::new([b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'x', (b'0' + (seq % 10) as u8)]))
                .expect("failed to send event");
            sleep(std::time::Duration::from_secs(5));
            seq += 1;
        }
    }
}
