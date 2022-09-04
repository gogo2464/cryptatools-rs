//! Encrypt with Caesar shifting encryption algorithm.
pub struct CaesarNumberAlgorithm {
    /// Alphabet used by the caesar number encryption Algotithm.
    pub alphabet : String,
}

impl CaesarNumberAlgorithm {
    pub fn new(alphabet: String) -> Self {
        CaesarNumberAlgorithm {
            alphabet
        }
    }

     ///  Encrypt the plain text with the caesar number encryption algorithm.
     ///
     ///  The plain text is passed as argument. Each character in the plain text is shifted of `key` ranges in the alphabet.
     ///  If the alphabet overflows, then the cipher text continues from the start of the alphabet.
     ///  The custom alphabet has been put in the constructor of the struct CaesarNumberAlgorithm.
     /// 
     ///  ```
     ///  use cryptatools_core::cryptography::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
     ///  use cryptatools_core::utils::alphabets::ASCII_ALPHABET;
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(String::from(ASCII_ALPHABET));
     ///  let encrypted = c.encrypt(vec![0x41, 0x41, 0x41], 1);
     ///  assert_eq!(vec![0x42, 0x42, 0x42], encrypted);
     ///  ```
     /// 
     ///  ```
     ///  use cryptatools_core::cryptography::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
     ///  use cryptatools_core::utils::alphabets::ASCII_ALPHABET;
     ///  use std::char;
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(String::from(ASCII_ALPHABET));
     ///  let plain_text: Vec<u8> = vec!(0x41, 0x41, 0x41);
     ///  let encrypted = c.encrypt(plain_text, 1);
     ///  let mut re_encrypted = String::new();
     ///  for character_int in encrypted {
     ///      re_encrypted.push(character_int.into());
     ///  }
     ///  assert_eq!(re_encrypted, "BBB");
     ///  ```
     /// 
     ///  ```
     ///  use cryptatools_core::cryptography::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
     ///  use cryptatools_core::utils::alphabets::ASCII_ALPHABET;
     ///  use std::char;
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(String::from(ASCII_ALPHABET));
     ///  let plain_text: Vec<u8> = vec!(0x41, 0x41, 0x41);
     ///  let encrypted = c.encrypt(plain_text, 10);
     ///  let mut re_encrypted = String::new();
     ///  for character_int in encrypted {
     ///      re_encrypted.push(character_int.into());
     ///  }
     ///  assert_eq!(re_encrypted, "KKK");
     ///  ```

     pub fn encrypt(&self, plain_text: Vec<u8>, key: u32) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for element in plain_text {
            let character: u8 = (element + key as u8) % self.alphabet.len() as u8; //TODO: set key and alphabet len as usize for infinite len
            result.push(character);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::* ;
    use crate::utils::alphabets::ASCII_ALPHABET;

    #[test]
    fn encrypt_with_caesar_number_encryption_algorithm() {
        let c = CaesarNumberAlgorithm::new(String::from(String::from(ASCII_ALPHABET)));
        let encrypted = c.encrypt(vec![0x42, 0x42, 0x42], 1);
        assert_eq!(vec![0x43, 0x43, 0x43], encrypted);
    }
}