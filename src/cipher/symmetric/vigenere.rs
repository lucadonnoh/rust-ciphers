use crate::cipher::operation_mode::StreamCipher;
use crate::cipher::Cipher;

pub struct Vigenere {
    key: String,
}

impl Vigenere {
    pub fn new(key: &str) -> Vigenere {
        Vigenere {
            key: key.to_string(),
        }
    }
}

impl StreamCipher for Vigenere {}

impl Cipher for Vigenere {
    fn encrypt(&self, text: &str) -> String {
        let mut key_iter = self.key.chars().cycle();
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let key = key_iter.next().unwrap();
                    let key = key.to_ascii_lowercase() as u8 - b'a';
                    self.shift_char(c, key)
                } else {
                    c
                }
            })
            .collect()
    }

    fn decrypt(&self, text: &str) -> String {
        let mut key_iter = self.key.chars().cycle();
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let key = key_iter.next().unwrap();
                    let key = key.to_ascii_lowercase() as u8 - b'a';
                    self.shift_char(c, 26 - key)
                } else {
                    c
                }
            })
            .collect()
    }
}