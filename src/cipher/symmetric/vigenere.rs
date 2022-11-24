use crate::ascii::AsciiString;

use crate::cipher::operation_mode::StreamCipher;
use crate::cipher::Cipher;
use crate::cipher::CipherError;

pub struct Vigenere {
    key: AsciiString,
}

impl Vigenere {
    pub fn new(key: AsciiString) -> Vigenere {
        Vigenere {
            key,
        }
    }
}

impl StreamCipher for Vigenere {}

impl Cipher for Vigenere {
    fn encrypt(&self, text: AsciiString) -> Result<AsciiString, CipherError> {
        let mut key_iter = self.key.value.chars().cycle();
        text.value.chars()
            .map(|c| {
                let key = key_iter.next().unwrap();
                let key = key.to_ascii_lowercase() as u8 - b'a';
                self.shift_char(c, key)
            }).collect()
    }

    fn decrypt(&self, text: AsciiString) -> Result<AsciiString, CipherError> {
        let mut key_iter = self.key.value.chars().cycle();
        text.value.chars()
            .map(|c| {
                let key = key_iter.next().unwrap();
                let key = key.to_ascii_lowercase() as u8 - b'a';
                self.shift_char(c, 26 - key)
            }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vigenere() {
        let key = AsciiString::from("key".to_string()).unwrap();
        let vigenere = Vigenere::new(key);
        let text = AsciiString::from("Hello, World!".to_string()).unwrap();
        let encrypted = vigenere.encrypt(text).unwrap();
        let decrypted = vigenere.decrypt(encrypted).unwrap();
        assert_eq!(decrypted.value, "Hello, World!");
    }
}