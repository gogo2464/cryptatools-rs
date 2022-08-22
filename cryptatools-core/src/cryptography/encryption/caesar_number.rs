//! Encrypt with Caesar shifting encryption algorithm.
pub struct CaesarNumberAlgorithm {
    /// Alphabet used by the caesar number encryption Algotithm.
    alphabet : &'static str,
}

impl CaesarNumberAlgorithm {
    pub fn new(alphabet: &'static str) -> Self {
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
     ///  use crate::cryptatools_core::cryptography::encryption::caesar_number::CaesarNumberAlgorithm;
     ///  use crate::cryptatools_core::cryptography::encoding::alphabets::ASCII_ALPHABET;
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(ASCII_ALPHABET);
     ///  let encrypted = c.encrypt(vec![0x41, 0x41, 0x41], 1);
     ///  assert_eq!(vec![0x42, 0x42, 0x42], encrypted);
     ///  ```
     /// 
     ///  ```
     ///  use crate::cryptatools_core::cryptography::encryption::caesar_number::CaesarNumberAlgorithm;
     ///  use crate::cryptatools_core::cryptography::encoding::alphabets::ASCII_ALPHABET;
     ///  use std::char;
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(ASCII_ALPHABET);
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
     ///  use crate::cryptatools_core::cryptography::encryption::caesar_number::CaesarNumberAlgorithm;
     ///  use crate::cryptatools_core::cryptography::encoding::alphabets::ASCII_ALPHABET;
     ///  use std::char;
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(ASCII_ALPHABET);
     ///  let plain_text: Vec<u8> = vec!(0x41, 0x41, 0x41);
     ///  let encrypted = c.encrypt(plain_text, 10);
     ///  let mut re_encrypted = String::new();
     ///  for character_int in encrypted {
     ///      re_encrypted.push(character_int.into());
     ///  }
     ///  assert_eq!(re_encrypted, "KKK");
     ///  ```

     pub fn encrypt(&mut self, plain_text: Vec<u8>, key: usize) -> Vec<u8> {
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
    use crate::cryptography::encoding::alphabets::ASCII_ALPHABET;

    #[test]
    fn encrypt_with_caesar_number_encryption_algorithm() {
        let mut c = CaesarNumberAlgorithm::new(ASCII_ALPHABET);
        let encrypted = c.encrypt(vec![0x42, 0x42, 0x42], 1);
        assert_eq!(vec![0x43, 0x43, 0x43], encrypted);
    }
}