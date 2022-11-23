use crate::cipher::operation_mode::StreamCipher;
use crate::cipher::Cipher;

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
    fn encrypt(&self, text: &str) -> String {
        text.chars().map(|c| self.shift_char(c, self.key)).collect()
    }

    fn decrypt(&self, text: &str) -> String {
        text.chars().map(|c| self.shift_char(c, 26 - self.key)).collect()
    }
}