#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use ::crossbeam_channel::Sender;
use ::std::path::PathBuf;

#[derive(Debug)]
pub struct Scanner {
    pub address: PathBuf,
    pub sink: Sender<PostzegelEvent>,
}

#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);
