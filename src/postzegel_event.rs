
#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

impl PostzegelEvent {
    pub fn new(code: [u8; 9]) -> PostzegelEvent {
        for c in code {
            assert!((c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || (c >= b'0' && c <= b'9'),
                    "only alphanumeric characters allowed in postzegel code: {code:?}");
        }
        PostzegelEvent { code }
    }
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);

