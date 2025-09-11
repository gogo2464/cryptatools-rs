use std::sync::Arc;
use std::{collections::HashMap, fs, path::Path};
use once_cell::sync::Lazy;
use crate::utils::alphabets::Alphabet;
use crate::utils::convert::{Encode};

use rand::Rng;

pub struct CoincidenceIndexGuesser {
    alphabet: Arc<Alphabet>,
}

impl CoincidenceIndexGuesser {
    pub fn new(alphabet: Arc<Alphabet>) -> Self {
        CoincidenceIndexGuesser {
            alphabet: alphabet,
        }
    }

    /// Guess coincidence index of `cipher_text_input`.
    /// 
    /// ```
    /// use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::frequency_analysis::coincidence_index::engine::coincidence_index::CoincidenceIndexGuesser;
    /// use cryptatools_core::utils::alphabets::Alphabet;
    /// use cryptatools_core::utils::convert::Encode;
    /// 
    /// use approx::assert_abs_diff_eq;
    /// 
    /// let alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// 
    /// let plain_text = String::from("Hello! How are you? I am fine and you?");
    /// let pseudo_cipher_text = Encode::from_ascii_to_u8(plain_text);
    /// let c = CoincidenceIndexGuesser::new(alphabet.into());
    /// let coincidence_index: f64 = c.guess_coincidence_index(pseudo_cipher_text);
    /// assert_abs_diff_eq!(coincidence_index, 0.06543385490753911, epsilon = 1e-16);
    /// ```
    pub fn guess_coincidence_index(&self, cipher_text_input: Vec<u8>) -> f64 {
        let cipher_text_size: f64 = cipher_text_input.len() as f64;
        if cipher_text_size <= 200.0 {
            println!("Warning: The cipher text input is {0} bytes. Probably {0} characters. It may be too short. You should provide more input characters.", cipher_text_size);
        }

        let mut coincidence_index: f64 = 0.0;
        for u8_byte_alphabet in self.alphabet.encoding.right_values() {//TODO: convert to f64
            let apparition_count: f64 = cipher_text_input.iter().filter(|&n| *n == u8_byte_alphabet[0]).count() as f64;// the [0] is a quick hack to avoid to find an algorithm to compare a set of bytes with some bytes of different size.
            let sum_apparition_alphabet: f64 = apparition_count * (apparition_count - 1.0);
            let divide_characters: f64 = cipher_text_size * (cipher_text_size - 1.0);
            let result: f64 = sum_apparition_alphabet / divide_characters;
            coincidence_index += result;
        }

        coincidence_index
    }

    /// Guess coincidence index of an unencrypted plain text external file.
    /// 
    /// This function does not only aim to be used against cipher text.
    /// The goal of this function is also to provide statistics about plain text coincidence index in order to decipher another encrypted text.
    /// ```
    /// use once_cell::sync::Lazy;
    /// use cryptatools_core::utils::alphabets::Alphabet;
    ///  use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::frequency_analysis::coincidence_index::engine::coincidence_index::CoincidenceIndexGuesser;
    /// use approx::assert_abs_diff_eq;
    /// 
    /// let alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// 
    /// let path = String::from("./data/text-corpus-for-statistics/gutenberg/austen-emma.txt");
    /// let mut c = CoincidenceIndexGuesser::new(alphabet.into());
    /// assert_abs_diff_eq!(c.guess_coincidence_index_statistics_from_file(path.clone()), 0.06742905478018858, epsilon = 1e-15);
    /// ```
    pub fn guess_coincidence_index_statistics_from_file(&self, file_name: String) -> f64 {
        let file_path = Path::new(&file_name);
        let file_content = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(error) => panic!("{0}", error),
        };

        let bytes_file_content = Encode::encode(&self.alphabet, file_content);
        let coincidence_index = self.guess_coincidence_index(bytes_file_content);
        
        coincidence_index
    }
}