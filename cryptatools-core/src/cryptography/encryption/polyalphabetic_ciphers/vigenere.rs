use crate::utils::alphabets;

pub struct Vigenere {
    alphabet: String,
}

impl Vigenere {
    pub fn new(alphabet: String) -> Self {
        Vigenere { 
            alphabet: alphabet 
         }
    }

    pub fn encrypt(plain_text: Vec<u8>, passphrase_key: Vec<u8>) {

    }
}