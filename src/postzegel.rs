use ::log::debug;
use ::log::trace;
use ::std::fmt;

#[derive(Debug)]
pub struct PostzegelEvent {
    code: [u8; 9],
}

impl PostzegelEvent {
    pub fn code_str(&self) -> String {
        self.code.iter()
            .map(|c| *c as char)
            .collect()
    }
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
        write!(f, "{}", self.code_str())
    }
}

const _: () = assert!(size_of::<PostzegelEvent>() < 10);

fn is_valid_code_byte(c: u8) -> bool {
    if c.is_ascii_lowercase() {
        trace!("valid lowercase letter: {c}");
        return true;
    }
    if c.is_ascii_uppercase() {
        trace!("valid uppercase letter: {c}");
        return true;
    }
    if c.is_ascii_digit() {
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

    #[test]
    fn letters() {
        let str = PostzegelEvent::new([b'a', b'b', b'c', b'd', b'e', b'f', b'G', b'H', b'I']).code_str();
        assert_eq!(str, "abcdefGHI");
    }

    #[test]
    fn code_str_digits() {
        let str = PostzegelEvent::new([b'1', b'3', b'5', b'7', b'9', b'2', b'4', b'6', b'8']).code_str();
        assert_eq!(str, "135792468");
    }

    #[test]
    fn code_str_mixed() {
        let str = PostzegelEvent::new([b'1', b'3', b'5', b'7', b'9', b'A', b'L', b'l', b'o']).code_str();
        assert_eq!(str, "13579ALlo");
    }
}