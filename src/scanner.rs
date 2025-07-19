use ::std::path::PathBuf;

#[derive(Debug)]
pub struct Scanner {
    address: PathBuf,
}

#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);
