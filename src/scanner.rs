
use crate::postzegel::PostzegelEvent;
use ::crossbeam_channel::Sender;
use ::log::debug;
use ::std::fmt::Debug;
use ::std::path::PathBuf;
use ::std::thread::sleep;

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
    pub name: u32,
    pub sink: Sender<PostzegelEvent>,
}

impl MockScanner {
    pub fn new(name: u32, sink: Sender<PostzegelEvent>) -> Scanner {
        Scanner::Mock(MockScanner { name, sink })
    }

    pub fn run(&self) -> ! {
        let mut seq: u64 = 0;
        loop {
            debug!("Sending mock event {seq} from scanner#{}", self.name);
            self.sink.send(PostzegelEvent::new([(b'0' + (self.name % 10) as u8), b'0', b'0', b'0', b'0', b'0', b'x',
                (b'0' + ((seq / 10) % 10) as u8), (b'0' + (seq % 10) as u8)]))
                .expect("failed to send event");
            sleep(std::time::Duration::from_secs(4));
            seq += 1;
        }
    }
}
