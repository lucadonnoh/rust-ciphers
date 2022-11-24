pub mod symmetric;
use crate::ascii::*;

#[derive(Debug, PartialEq)]
pub enum CipherError {
    InvalidChar(char)
}

impl std::fmt::Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CipherError::InvalidChar(c) => write!(f, "Invalid character: {}", c),
        }
    }
}

pub trait Cipher {
    fn encrypt(&self, text: AsciiString) -> Result<AsciiString, CipherError>;
    fn decrypt(&self, text: AsciiString) -> Result<AsciiString, CipherError>;
}

pub mod operation_mode {
    use super::CipherError;

    // TODO: is the ascii check needed? if no, remove CipherError
    pub trait StreamCipher {
        fn shift_char(&self, c: char, key: u8) -> Result<char, CipherError> {
            if c.is_ascii_alphabetic() {
                let offset = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let c = c as u8;
                let shifted = (c - offset + key) % 26 + offset;
                Ok(shifted as char)
            } else if c.is_ascii_punctuation() || c.is_ascii_digit() || c.is_whitespace() {
                Ok(c)
            } else {
                Err(CipherError::InvalidChar(c))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cipher::{symmetric::caesar::Caesar, operation_mode::StreamCipher};

    #[test]
    fn shift_char() {
        let caesar = Caesar::new(3);
        assert_eq!(caesar.shift_char('a', 3).unwrap(), 'd');
        assert_eq!(caesar.shift_char('A', 3).unwrap(), 'D');
        assert_eq!(caesar.shift_char('z', 3).unwrap(), 'c');
        assert_eq!(caesar.shift_char('Z', 3).unwrap(), 'C');
        assert_eq!(caesar.shift_char(' ', 3).unwrap(), ' ');
        assert_eq!(caesar.shift_char('!', 3).unwrap(), '!');
        assert_eq!(caesar.shift_char('1', 3).unwrap(), '1');

        assert_eq!(caesar.shift_char('d', 26 - 3).unwrap(), 'a');
        assert_eq!(caesar.shift_char('D', 26 - 3).unwrap(), 'A');
        assert_eq!(caesar.shift_char('c', 26 - 3).unwrap(), 'z');
        assert_eq!(caesar.shift_char('C', 26 - 3).unwrap(), 'Z');
        assert_eq!(caesar.shift_char(' ', 26 - 3).unwrap(), ' ');
        assert_eq!(caesar.shift_char('!', 26 - 3).unwrap(), '!');
        assert_eq!(caesar.shift_char('1', 26 - 3).unwrap(), '1');

        assert_eq!(caesar.shift_char('°', 3), Err(CipherError::InvalidChar('°')));
    }
}