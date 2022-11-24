mod cipher;
mod ascii;

use crate::ascii::AsciiString;
use crate::ascii::EncodingError;

use crate::cipher::Cipher;
use crate::cipher::symmetric::caesar::Caesar;
use crate::cipher::symmetric::vigenere::Vigenere;
use crate::cipher::CipherError;

#[derive(Debug)]
enum Error {
    CipherError(CipherError),
    IoError(std::io::Error),
    EncodingError(EncodingError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::CipherError(e) => write!(f, "Cipher error: {}", e),
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::EncodingError(e) => write!(f, "Encoding error: {}", e),
        }
    }
}

fn main() -> Result<(), Error> {
    print!("Enter a message: \n");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();

    let msg = AsciiString::from(message.to_string()).map_err(Error::EncodingError)?;

    print!("Enter cipher type (caesar or vigenere): \n");
    let mut cipher_type = String::new();
    std::io::stdin().read_line(&mut cipher_type).map_err(Error::IoError)?;
    let cipher_type = cipher_type.trim();

    let cipher: Box<dyn Cipher> = match cipher_type {
        "caesar" => {
            print!("Enter shift: \n");
            let mut shift = String::new();
            std::io::stdin().read_line(&mut shift).map_err(Error::IoError)?;
            let shift = shift.trim().parse::<u8>().unwrap();
            Box::new(Caesar::new(shift))
        }
        "vigenere" => {
            print!("Enter key: \n");
            let mut key = String::new();
            std::io::stdin().read_line(&mut key).map_err(Error::IoError)?;
            let key = AsciiString::from(key.trim().to_string()).map_err(Error::EncodingError)?;
            Box::new(Vigenere::new(key))
        }
        _ => panic!("Unknown cipher type"),
    };

    println!("Message: {}", msg.value);
    let encrypted = cipher.encrypt(msg).map_err(Error::CipherError)?;
    println!("Encrypted: {}", encrypted.value);
    let decrypted = cipher.decrypt(encrypted).map_err(Error::CipherError)?;
    println!("Decrypted: {}", decrypted.value);

    Ok(())
}