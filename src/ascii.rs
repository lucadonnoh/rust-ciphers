#[derive(Debug)]
pub enum EncodingError {
    InvalidAsciiString,
}

impl std::fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EncodingError::InvalidAsciiString => write!(f, "Invalid ASCII string"),
        }
    }
}

pub struct AsciiString {
    pub value: String,
}

impl AsciiString {
    pub fn from(value: String) -> Result<AsciiString, EncodingError> {
        if value.is_ascii() {
            Ok(AsciiString { value })
        } else {
            Err(EncodingError::InvalidAsciiString)
        }
    }
}

impl FromIterator<char> for AsciiString {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> AsciiString {
        let mut value = String::new();
        for c in iter {
            if c.is_ascii() {
                value.push(c);
            } else {
                panic!("Non-ASCII character in AsciiString");
            }
        }
        AsciiString { value }
    }
}