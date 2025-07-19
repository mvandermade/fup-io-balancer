#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use ::crossbeam_channel::Sender;
use ::std::path::PathBuf;
use ::std::fmt::Debug;
use ::std::iter::Scan;

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

#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implement_test() {
        // panic!("{}", size_of::<PathBuf>());
        panic!("{}", size_of::<Scanner>());
    }
}
