use crate::ascii::AsciiString;
use crate::cipher::operation_mode::StreamCipher;
use crate::cipher::Cipher;
use crate::cipher::CipherError;

pub struct Caesar {
    key: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Caesar {
        Caesar { key: shift }
    }
}

impl StreamCipher for Caesar {}

impl Cipher for Caesar {
    fn encrypt(&self, text: AsciiString) -> Result<AsciiString, CipherError> {
        text.value.chars().map(|c| self.shift_char(c, self.key)).collect()
    }

    fn decrypt(&self, text: AsciiString) -> Result<AsciiString, CipherError> {
        text.value.chars().map(|c| self.shift_char(c, 26 - self.key)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar() {
        let caesar = Caesar::new(3);
        let text = AsciiString::from("Hello, World!".to_string()).unwrap();
        let encrypted = caesar.encrypt(text).unwrap();
        let decrypted = caesar.decrypt(encrypted).unwrap();
        assert_eq!(decrypted.value, "Hello, World!");
    }
}