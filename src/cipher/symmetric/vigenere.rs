use crate::cipher::operation_mode::StreamCipher;
use crate::cipher::Cipher;
use crate::cipher::CipherError;

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
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut key_iter = self.key.chars().cycle();
        text.chars()
            .map(|c| {
                let key = key_iter.next().unwrap();
                let key = key.to_ascii_lowercase() as u8 - b'a';
                self.shift_char(c, key)
            }).collect()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut key_iter = self.key.chars().cycle();
        text.chars()
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
        let vigenere = Vigenere::new("abc");
        assert_eq!(vigenere.encrypt("abc").unwrap(), "ace");
        assert_eq!(vigenere.decrypt("ace").unwrap(), "abc");
        
        let plaintext = "The quick brown fox jumps over the lazy dog";
        let ciphertext = vigenere.encrypt(plaintext).unwrap();
        let decrypted = vigenere.decrypt(&ciphertext).unwrap();
        assert_eq!(plaintext, decrypted);
    }
}