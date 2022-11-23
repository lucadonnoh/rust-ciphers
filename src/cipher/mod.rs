pub mod symmetric;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> String;
    fn decrypt(&self, text: &str) -> String;
}

pub mod operation_mode {
    pub trait StreamCipher {
        fn shift_char(&self, c: char, key: u8) -> char {
            if c.is_ascii_alphabetic() {
                let offset = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let c = c as u8;
                let shifted = (c - offset + key) % 26 + offset;
                shifted as char
            } else {
                // TODO: use Result instead;
                c
            }
        }
    }
}