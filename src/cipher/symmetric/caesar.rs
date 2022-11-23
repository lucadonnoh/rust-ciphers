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
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        text.chars().map(|c| self.shift_char(c, self.key)).collect()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        text.chars().map(|c| self.shift_char(c, 26 - self.key)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar() {
        let caesar = Caesar::new(3);
        assert_eq!(caesar.encrypt("abc").unwrap(), "def");
        assert_eq!(caesar.decrypt("def").unwrap(), "abc");
        
        let plaintext = "The quick brown fox jumps over the lazy dog";
        let ciphertext = caesar.encrypt(plaintext).unwrap();
        let decrypted = caesar.decrypt(&ciphertext).unwrap();
        assert_eq!(plaintext, decrypted);
    }
}