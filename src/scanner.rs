#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use ::crossbeam_channel::Sender;
use ::std::path::PathBuf;
use std::fmt::Debug;
use std::iter::Scan;

pub trait Scanner : Debug {
    fn run(&self) -> !;
}

#[derive(Debug)]
pub struct RealScanner {
    pub address: PathBuf,
    pub sink: Sender<PostzegelEvent>,
}

impl Scanner for RealScanner {
    fn run(&self) -> ! {
        todo!()
    }
}

#[derive(Debug)]
pub struct MockScanner {
    pub sink: Sender<PostzegelEvent>,
}

impl Scanner for MockScanner {
    fn run(&self) -> ! {
        todo!()
    }
}

#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);
