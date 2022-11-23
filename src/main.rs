mod cipher;

use cipher::Cipher;
use cipher::symmetric::caesar::Caesar;
use cipher::symmetric::vigenere::Vigenere;
use cipher::CipherError;

fn main() {
    print!("Enter a message: \n");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();

    print!("Enter cipher type (caesar or vigenere): \n");
    let mut cipher_type = String::new();
    std::io::stdin().read_line(&mut cipher_type).unwrap();
    let cipher_type = cipher_type.trim();

    let cipher: Box<dyn Cipher> = match cipher_type {
        "caesar" => {
            print!("Enter shift: \n");
            let mut shift = String::new();
            std::io::stdin().read_line(&mut shift).unwrap();
            let shift = shift.trim().parse().unwrap();
            Box::new(Caesar::new(shift))
        }
        "vigenere" => {
            print!("Enter key: \n");
            let mut key = String::new();
            std::io::stdin().read_line(&mut key).unwrap();
            let key = key.trim();
            Box::new(Vigenere::new(key))
        }
        _ => panic!("Unknown cipher type"),
    };

    let encrypted = cipher.encrypt(message);
    match encrypted {
        Ok(ref encrypted) => println!("Encrypted: {}", encrypted),
        Err(CipherError::InvalidChar(c)) => println!("Invalid character: {}", c),
    }

    // TODO: si può migliorare togliendo l'unwrap
    let decrypted = cipher.decrypt(&encrypted.unwrap());
    match decrypted {
        Ok(decrypted) => println!("Decrypted: {}", decrypted),
        Err(CipherError::InvalidChar(c)) => println!("Invalid character: {}", c),
    }
}