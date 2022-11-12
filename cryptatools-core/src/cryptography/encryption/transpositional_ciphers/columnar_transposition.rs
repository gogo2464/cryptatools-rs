use crate::utils::alphabets::Alphabet;
use crate::utils::alphabets::split_bytes_by_characters_representation;
use crate::utils::alphabets::uniffy_opcode_group;

pub struct ColumnarTranspositionAlgorithm {
    alphabet: Alphabet,
}

impl ColumnarTranspositionAlgorithm {
    pub fn new(alphabet: Alphabet) -> Self {
        ColumnarTranspositionAlgorithm { 
            alphabet: alphabet
         }
    }
    
    /// Encrypt the plain text with the columnar transposition encryption algorithm.
    ///
    /// An array with lines of size `key` is created with the `plain_text` argument. The cipher text is producted as output with reading the plain text, column by column, from top to bot, from lefgt to right.
    /// The custom alphabet has been put in the constructor of the struct CaesarNumberAlgorithm.
    /// 
    /// ```
    /// use cryptatools_core::utils::alphabets::Alphabet;
    /// use cryptatools_core::utils::convert::{Encode, Decode};
    /// use cryptatools_core::cryptography::encryption::transpositional_ciphers::columnar_transposition::ColumnarTranspositionAlgorithm;
    /// 
    /// let printable_ascii_alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// let key = 6;
    /// let plain_text = "all work and no play makes johnny a dull boy".replace(" ", "").to_uppercase();
    /// let plain_text_opcodes = Encode::encode(&printable_ascii_alphabet, String::from(plain_text));
    /// let mut columnar = ColumnarTranspositionAlgorithm::new(printable_ascii_alphabet);
    /// let cipher_text_opcodes = columnar.encrypt(plain_text_opcodes, 6);
    /// let ascii_plain_text = Decode::from_u8_to_ascii(cipher_text_opcodes);
    /// 
    /// assert_eq!(ascii_plain_text, "AKPKNLLALENLLNASYBWDYJAOONMODYROAHU");
    /// ```
    pub fn encrypt(&mut self, plain_text: Vec<u8>, key: u32) -> Vec<u8> {
        let plain_text_group = split_bytes_by_characters_representation(&self.alphabet, plain_text);
        let columns: Vec<Vec<u8>> = (0..key)
            .flat_map(|collumn| (collumn..plain_text_group.len() as u32)
                .step_by(key as usize)
                .map(|i| plain_text_group[i as usize].clone()))
            .collect();

        uniffy_opcode_group(columns)
    }
    
    /// Decrypt the plain text with the columnar transposition encryption algorithm.
    /// 
    /// Exatcly same operation as `encrypt` method.
    /// 
    /// ```
    /// use cryptatools_core::utils::alphabets::Alphabet;
    /// use cryptatools_core::utils::convert::{Encode, Decode};
    /// use cryptatools_core::cryptography::encryption::transpositional_ciphers::columnar_transposition::ColumnarTranspositionAlgorithm;
    ///
    /// let printable_ascii_alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// let mut columnar = ColumnarTranspositionAlgorithm::new(printable_ascii_alphabet);
    /// let cipher_text_ascii = String::from("AKPKNLLALENLLNASYBWDYJAOONMODYROAHU");
    /// let opcodes_plain_text = Encode::from_ascii_to_u8(cipher_text_ascii);
    /// let key = 6;
    /// let plain_text_opcodes = columnar.decrypt(opcodes_plain_text, key);
    /// let ascii_plain_text = Decode::from_u8_to_ascii(plain_text_opcodes);
    /// 
    /// assert_eq!(ascii_plain_text, "all work and no play makes johnny a dull boy".replace(" ", "").to_uppercase());
    /// ```
    pub fn decrypt(&mut self, cipher_text: Vec<u8>, key: u32) -> Vec<u8> {
        let cipher_text_group = split_bytes_by_characters_representation(&self.alphabet, cipher_text);
        let columns: Vec<Vec<u8>> = (0..key)
            .flat_map(|collumn| (collumn..cipher_text_group.len() as u32)
                .step_by(key as usize)
                .map(|i| cipher_text_group[i as usize].clone()))
            .collect();

        uniffy_opcode_group(columns)
    }
}