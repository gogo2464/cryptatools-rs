use std::{collections::HashMap, hash::Hash};
use once_cell::sync::Lazy;
use std::fs;
use crate::utils::{convert, alphabets::ASCII_ALPHABET};
use std::path::Path;

pub struct CoincidenceIndexGuesser {
    alphabet: HashMap<String, Vec<u8>>,
}

impl CoincidenceIndexGuesser {
    pub fn new(alphabet: HashMap<String, Vec<u8>>) -> Self {
        CoincidenceIndexGuesser {
            alphabet: alphabet,
        }
    }

    /// Guess coincidence index of `cipher_text_input`.
    /// 
    /// ```
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::{CoincidenceIndexGuesser};
    /// use cryptatools_core::utils::{alphabets::ASCII_ALPHABET, convert};
    /// use once_cell::sync::Lazy;
    /// use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};
    /// let plain_text = String::from("Hello! How are you? I am fine and you?");
    /// let pseudo_cipher_text = convert::Encode::from_ascii_to_u8(plain_text);
    /// let c = CoincidenceIndexGuesser::new(Lazy::force(&ASCII_ALPHABET).to_owned());
    /// let coincidence_index: f64 = c.guess_coincidence_index(pseudo_cipher_text);
    /// assert_f64_near!(0.06543385490753913, coincidence_index);
    /// ```
    pub fn guess_coincidence_index(self, cipher_text_input: Vec<u8>) -> f64 {
        let cipher_text_size: f64 = cipher_text_input.len() as f64;
        if cipher_text_size <= 200.0 {
            println!("Warning: The cipher text input is {0} bytes. Probably {0} characters. It may be too short. You should provide more input characters.", cipher_text_size);
        }

        let mut iteration: HashMap<u8, f64> = HashMap::new();


        for u8_byte_alphabet in self.alphabet.values() {//TODO: convert to f64
            let apparition_count: f64 = cipher_text_input.iter().filter(|&n| *n == u8_byte_alphabet[0]).count() as f64;// the [0] is a quick hack to avoid to find an algorithm to compare a set of bytes with some bytes of different size.
            let sum_apparition_alphabet: f64 = apparition_count * (apparition_count - 1.0);
            let divide_characters: f64 = cipher_text_size * (cipher_text_size - 1.0);
            let result: f64 = sum_apparition_alphabet / divide_characters;
            iteration.insert(u8_byte_alphabet[0], result);// the [0] is a quick hack to avoid to find an algorithm to compare a set of bytes with some bytes of different size.
        }

        let coincidence_index: f64 = iteration.iter().map(|x| x.1).sum();

        coincidence_index
    }



}

pub trait VigenereCoincidenceIndexGuesser {
    fn guess_coincidence_index_statistics_from_file(self, file: String) -> f64;
    fn guess_coincidence_indexes_statistics_from_file(self, file: String) -> f64;
}

impl VigenereCoincidenceIndexGuesser for CoincidenceIndexGuesser {
    /// Guess coincidence index of an unencrypted plain text external file.
    /// 
    /// This function does not only aim to be used against cipher text.
    /// The goal of this function is also to provide statistics about plain text coincidence index in order to decipher another encrypted text.
    /// ```
    /// use std::path::Path;
    /// use std::fs;
    /// use once_cell::sync::Lazy;
    /// use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};
    /// use cryptatools_core::utils::alphabets::ASCII_ALPHABET;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::{CoincidenceIndexGuesser, VigenereCoincidenceIndexGuesser};
    /// let path = String::from("./data/text-corpus-for-statistics/gutenberg/austen-emma.txt");
    /// let mut c = CoincidenceIndexGuesser::new(Lazy::force(&ASCII_ALPHABET).to_owned());
    /// 
    /// assert_f64_near!(c.guess_coincidence_index_statistics_from_file(path), 0.06525540393695795)
    /// ```
    fn guess_coincidence_index_statistics_from_file(self, file_name: String) -> f64 {
        let file_path = Path::new(&file_name);
        let file_content = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(error) => panic!("{0}", error),
        };

        let bytes_file_content = convert::Encode::from_ascii_to_u8(file_content);
        let coincidence_index = self.guess_coincidence_index(bytes_file_content);
        
        coincidence_index
    }

    /// Guess coincidence indexes corresponding to each keys size coincidence index values of an external file.
    /// 
    /// This function does not only aim to be used against cipher text.
    /// The goal of this function is also to provide statistics about plain text coincidence index in order to decipher another encrypted text.
    fn guess_coincidence_indexes_statistics_from_file(self, file: String) -> f64 {

        0.0
    }
}


pub static english_default_coincidence_index: Lazy<HashMap<usize, f64>> = Lazy::new(|| {
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