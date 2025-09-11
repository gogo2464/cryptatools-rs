use std::sync::Arc;
use std::{collections::HashMap, fs, path::Path};
use once_cell::sync::Lazy;
use crate::cryptography::classical::encryption::polyalphabetic_ciphers::vigenere::VigenereNoTable;
use crate::utils::alphabets::Alphabet;
use crate::utils::convert::{Encode};

use crate::CoincidenceIndexGuesser;

use rand::Rng;

pub struct VigenereCoincidenceIndexGenerator {
    alphabet: Arc<Alphabet>,
}

impl VigenereCoincidenceIndexGenerator {
    pub fn new(alphabet: Arc<Alphabet>) -> Self {
        VigenereCoincidenceIndexGenerator {
            alphabet: alphabet,
        }
    }
    /// Generate coincidence index of a specific key_size choosen poorly for a specified plain text
    /// 
    /// This function does not only aim to be used against cipher text.
    /// The goal of this function is also to provide statistics about plain text coincidence index in order to decipher another encrypted text.
    /// 
    /// ```
    /// use cryptatools_core::utils::{convert::Encode, alphabets::Alphabet};
    /// use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::frequency_analysis::coincidence_index::modules::vigenere_coincidence_index::VigenereCoincidenceIndexGenerator;
    /// 
    /// use approx::assert_abs_diff_eq;
    /// 
    /// let alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// 
    /// let mut vcig = VigenereCoincidenceIndexGenerator::new(alphabet.into());
    /// let mut ci = vcig.generate_coincidence_index_for_key(0, Encode::from_ascii_to_u8(String::from("The ennemy will never attack!")));
    /// assert_abs_diff_eq!(ci, 0.0541871921182266, epsilon = 1e-16);
    /// ci = vcig.generate_coincidence_index_for_key(5, Encode::from_ascii_to_u8(String::from("The ennemy will never attack! I think the ennemy is simply too coward in order to lead an attack. He will hesistate too much. Prepare your troops for a very very long defense of this siege.")));
    /// assert_abs_diff_eq!(ci, 0.02, epsilon = 1e-1);
    /// ci = vcig.generate_coincidence_index_for_key(10, Encode::from_ascii_to_u8(String::from("The ennemy will never attack!")));
    /// assert_abs_diff_eq!(ci, 0.01, epsilon = 1e-1);
    /// 
    /// ci = vcig.generate_coincidence_index_for_key(10, Encode::from_ascii_to_u8(String::from("The ennemy will never attack!")));
    /// assert_abs_diff_eq!(ci, 0.01, epsilon = 1e-1);
    /// ```
    pub fn generate_coincidence_index_for_key(&self, key_size: usize, input: Vec<u8>) -> f64 {
        let mut rng = rand::thread_rng();
        let mut key = vec![];
        for _i in 0..key_size {
            let random_value = rng.gen_range(0..self.alphabet.encoding.len());
            let random_byte = self.alphabet.encoding.right_values().nth(random_value).unwrap()[0];// TODO get multichars // .sorted() ?
            key.push(vec![random_byte]);
        }

        let vig: VigenereNoTable = VigenereNoTable::new(self.alphabet.clone());
        let vigenere_coincidence_index_guesser = CoincidenceIndexGuesser::new(self.alphabet.clone());
        let cipher_text = vig.encrypt(input, key);
        let coincidence_index = vigenere_coincidence_index_guesser.guess_coincidence_index(cipher_text);

        coincidence_index
    }

    /// Generate coincidence index corresponding for a sepcific size coincidence index values of an external file in plain text.
    /// 
    /// 
    /// The coincidence index depends of the alphabet.
    /// ```
    /// use once_cell::sync::Lazy;
    /// use cryptatools_core::utils::{convert::Encode,  alphabets::Alphabet};
    /// use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::frequency_analysis::coincidence_index::modules::vigenere_coincidence_index::VigenereCoincidenceIndexGenerator;
    /// use approx::assert_abs_diff_eq;
    /// 
    /// 
    /// let alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// let mut vcig = VigenereCoincidenceIndexGenerator::new(alphabet.into());
    /// let mut ci = vcig.generate_coincidence_index_for_key_from_file(5, String::from("./data/text-corpus-for-statistics/gutenberg/austen-emma.txt"));
    /// 
    /// assert_abs_diff_eq!(ci, 0.01, epsilon = 1e-1);
    /// ```
    pub fn generate_coincidence_index_for_key_from_file(&self, key_size: usize, plain_text_ascii_file_input: String) -> f64 {
        let file_path = Path::new(&plain_text_ascii_file_input);
        let file_content = match fs::read_to_string(file_path) {
            Ok(file) => file.replace("\r\n", "\n"),
            Err(error) => panic!("{0}", error),
        };

        let encoded_file_content = Encode::encode(&self.alphabet, file_content);
        let coincidence_index_found = self.generate_coincidence_index_for_key(key_size, encoded_file_content);
        coincidence_index_found
    }
}

pub static ENGLISH_DEFAULT_COINCIDENCE_INDEX: Lazy<HashMap<usize, f64>> = Lazy::new(|| {
    let mut coincidence_index = HashMap::new();
    coincidence_index.insert(1, 0.0639);
    coincidence_index.insert(2, 0.0511);
    coincidence_index.insert(3, 0.0468);
    coincidence_index.insert(4, 0.0446);
    coincidence_index.insert(5, 0.0438);
    coincidence_index.insert(6, 0.0426);

    coincidence_index
  }
);
