//! Encrypt with Caesar shifting encryption algorithm.
use std::sync::Arc;
use crate::utils::alphabets::Alphabet;
use crate::utils::alphabets;

pub struct CaesarNumberAlgorithm {
    /// Alphabet used by the caesar number encryption Algotithm.
    pub alphabet: Arc<Alphabet>,
}

impl CaesarNumberAlgorithm {
    pub fn new(alphabet: Arc<Alphabet>) -> Self {
        CaesarNumberAlgorithm {
            alphabet
        }
    }

     ///  Encrypt the plain text with the caesar number encryption algorithm.
     ///
     ///  The `plain_text` is passed as argument. Each character in the `plain_text` is shifted of `key` ranges in his opcode representation.
     ///  If the alphabet overflows, then the cipher text continues from the start of the alphabet.
     ///  The custom alphabet has been put in the constructor of the struct CaesarNumberAlgorithm.
     /// 
     ///  ```
     ///  use cryptatools_core::cryptography::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
     ///  use cryptatools_core::utils::alphabets::Alphabet;
     ///  
     ///  let ascii_alphabet = Alphabet::new_empty().ascii_encoding();
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(ascii_alphabet.into());
     ///  let encrypted = c.encrypt_by_opcode_shift(vec![0x41, 0x41, 0x41], 1);
     ///  assert_eq!(vec![0x42, 0x42, 0x42], encrypted);
     ///  ```
     /// 
     ///  ```
     ///  use cryptatools_core::cryptography::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
     ///  use cryptatools_core::utils::alphabets::Alphabet;
     ///  use std::char;
     /// 
     ///  let ascii_alphabet = Alphabet::new_empty().ascii_encoding();
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(ascii_alphabet.into());
     ///  let plain_text: Vec<u8> = vec![0x41, 0x41, 0x41];
     ///  let encrypted = c.encrypt_by_opcode_shift(plain_text, 1);
     ///  let mut re_encrypted = String::new();
     ///  for character_int in encrypted {
     ///      re_encrypted.push(character_int.into());
     ///  }
     ///  assert_eq!(re_encrypted, "BBB");
     ///  ```
     /// 
     ///  ```
     ///  use cryptatools_core::cryptography::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
     ///  use cryptatools_core::utils::alphabets::Alphabet;
     ///  use std::char;
     /// 
     ///  let printable_chars_of_ascii_characters = Alphabet::new_empty().ascii_printable_only_encoding();
     ///  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(printable_chars_of_ascii_characters.into());
     ///  let plain_text: Vec<u8> = vec![0x41, 0x41, 0x41];
     ///  let encrypted = c.encrypt_by_opcode_shift(plain_text, 10);
     ///  let mut re_encrypted = String::new();
     ///  for character_int in encrypted {
     ///      re_encrypted.push(character_int.into());
     ///  }
     ///  assert_eq!(re_encrypted, "KKK");
     ///  ```
     pub fn encrypt_by_opcode_shift(&self, plain_text: Vec<u8>, key: u32) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for element in plain_text {
            let character: u8 = ((element as u32 + key) % 255 as u32) as u8;
            result.push(character);
        }

        return result;
    }

     ///  Encrypt the plain text with the caesar number encryption algorithm.
     ///
     ///  The `plain_text` is passed as argument. Each character in the `plain_text` is shifted of `key` ranges in the alphabet.
     ///  If the alphabet overflows, then the cipher text continues from the start of the alphabet.
     ///  The custom alphabet has been put in the constructor of the struct CaesarNumberAlgorithm.
     /// 
    pub fn encrypt_by_alphabet_shift(&self, plain_text: Vec<u8>, key: u32) -> Vec<u8> {
        let plain_unified_opcodes = alphabets::split_bytes_by_characters_representation(&self.alphabet, plain_text.clone());
        let mut cipher_unified_opcodes = vec![];
        let ordered_alphabet = self.alphabet.get_encoding();

        for opcode in plain_unified_opcodes {
            let plain_text_position: u32 = ordered_alphabet.iter().position(|opcodes_key| opcodes_key.bytes == opcode).unwrap() as u32;
            let cipher_text_position: u32 = (plain_text_position + key) % (self.alphabet.get_encoding().len() as u32);

            cipher_unified_opcodes.push(ordered_alphabet[cipher_text_position as usize].bytes.clone());
        }

        alphabets::uniffy_opcode_group(cipher_unified_opcodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_with_caesar_number_encryption_algorithm() {
        let ascii = alphabets::Alphabet::new(vec![]).ascii_printable_only_encoding();
        let c = CaesarNumberAlgorithm::new(Arc::new(ascii));
        let encrypted = c.encrypt_by_opcode_shift(vec![0x42, 0x42, 0x42], 1);
        assert_eq!(vec![0x43, 0x43, 0x43], encrypted);
    }
}