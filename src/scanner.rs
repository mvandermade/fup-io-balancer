use crate::postzegel::PostzegelEvent;
use crate::util::Sink;
use ::log::debug;
use ::std::fmt::Debug;
use ::std::path::PathBuf;
use tokio::time;

#[derive(Debug)]
pub enum Scanner {
    #[allow(dead_code)]  //TODO
    Real(RealScanner),
    Mock(MockScanner),
}

impl Scanner {
    pub async fn run(&self) -> ! {
        match self {
            Scanner::Real(_) => unimplemented!(),
            Scanner::Mock(scanner) => scanner.run().await,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]  //TODO
pub struct RealScanner {
    pub address: PathBuf,
    pub sink: Sink<PostzegelEvent>,
}

#[derive(Debug)]
pub struct MockScanner {
    pub name: u32,
    pub sink: Sink<PostzegelEvent>,
}

impl MockScanner {
    pub fn new(name: u32, sink: Sink<PostzegelEvent>) -> Scanner {
        Scanner::Mock(MockScanner { name, sink })
    }

    pub async fn run(&self) -> ! {
        let mut seq: u64 = 0;
        loop {
            debug!("Sending mock event {seq} from scanner#{}", self.name);
            self.sink.send(PostzegelEvent::new([(b'0' + (self.name % 10) as u8), b'0', b'0', b'0', b'0', b'0', b'x',
                (b'0' + ((seq / 10) % 10) as u8), (b'0' + (seq % 10) as u8)]))
                .await.expect("failed to send event");
            time::sleep(std::time::Duration::from_secs(4));
            seq += 1;
        }
    }
}
