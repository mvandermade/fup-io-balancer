#![allow(unused)]  //TODO @mark: TEMPORARY! REMOVE THIS!

use ::std::path::PathBuf;
use ::crossbeam_channel::bounded;

#[derive(Debug)]
pub struct Scanner {
    address: PathBuf,
    sink: (),
}

#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);
