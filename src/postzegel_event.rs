use log::debug;
use log::trace;
use std::fmt;

#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

impl PostzegelEvent {
    pub fn new(code: [u8; 9]) -> PostzegelEvent {
        for c in code {
            assert!(
                is_valid_code_byte(c),
                "only alphanumeric characters allowed in postzegel code: {code:?}"
            );
        }
        PostzegelEvent { code }
    }
}

impl fmt::Display for PostzegelEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.code {
            write!(f, "{}", (c as char))?
        }
        Ok(())
    }
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);

fn is_valid_code_byte(c: u8) -> bool {
    if c >= b'a' && c <= b'z' {
        trace!("valid lowercase letter: {c}");
        return true;
    }
    if c >= b'A' && c <= b'Z' {
        trace!("valid uppercase letter: {c}");
        return true;
    }
    if c >= b'0' && c <= b'9' {
        trace!("valid digit: {c}");
        return true;
    }
    debug!("invalid character: {c} (should be alphanumeric)");
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase() {
        assert!(is_valid_code_byte(b'a'));
        assert!(is_valid_code_byte(b'q'));
        assert!(is_valid_code_byte(b'z'));
    }

    #[test]
    fn uppercase() {
        assert!(is_valid_code_byte(b'A'));
        assert!(is_valid_code_byte(b'F'));
        assert!(is_valid_code_byte(b'Z'));
    }

    #[test]
    fn digit() {
        assert!(is_valid_code_byte(b'0'));
        assert!(is_valid_code_byte(b'5'));
        assert!(is_valid_code_byte(b'9'));
    }

    #[test]
    fn not_alphanumeric() {
        assert!(!is_valid_code_byte(b'#'));
        assert!(!is_valid_code_byte(b'$'));
        assert!(!is_valid_code_byte(b'='));
        assert!(!is_valid_code_byte(b'\n'));
        assert!(!is_valid_code_byte(b' '));
        assert!(!is_valid_code_byte(b'\0'));
        assert!(!is_valid_code_byte(b'0' - 1));
        assert!(!is_valid_code_byte(b'9' + 1));
        assert!(!is_valid_code_byte(b'a' - 1));
        assert!(!is_valid_code_byte(b'z' + 1));
        assert!(!is_valid_code_byte(b'A' - 1));
        assert!(!is_valid_code_byte(b'Z' + 1));
    }
}