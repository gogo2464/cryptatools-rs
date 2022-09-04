use std::collections::HashMap;
use bimap::BiMap;
use crate::utils::{convert, alphabets::{self, ASCII_ALPHABET}};

pub struct CoincidenceIndex {
    alphabet: BiMap<&'static str, Vec<u8>>,
}

impl CoincidenceIndex {
    pub fn new(alphabet: BiMap<&'static str, Vec<u8>>) -> Self {
        CoincidenceIndex {
            alphabet: alphabet,
        }
    }

    /// Guess coincidence index of `cipher_text_input`.
    /// 
    /// ```
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::{CoincidenceIndex};
    /// use cryptatools_core::utils::{alphabets::ASCII_ALPHABET, convert};
    /// use once_cell::sync::Lazy;
    /// use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};
    /// let plain_text = String::from("Hello! How are you? I am fine and you?");
    /// let pseudo_cipher_text = convert::Encode::from_ascii_to_u8(plain_text);
    /// let c = CoincidenceIndex::new(Lazy::force(&ASCII_ALPHABET).to_owned());
    /// let coincidence_index: f64 = c.guess_coincidence_index(pseudo_cipher_text);
    /// assert_f64_near!(0.06543385490753913, coincidence_index);
    /// ```
    pub fn guess_coincidence_index(self, cipher_text_input: Vec<u8>) -> f64{
        let cipher_text_size: f64 = cipher_text_input.len() as f64;
        if cipher_text_size <= 200.0 {
            println!("Warning: The cipher text input is {0} bytes. Probably {0} characters. It may be too short. You should provide more input characters.", cipher_text_size);
        }

        let mut iteration: HashMap<u8, f64> = HashMap::new();


        for u8_byte_alphabet in self.alphabet.right_values() {//TODO: convert to f64
            let apparition_count: f64 = cipher_text_input.iter().filter(|&n| *n == u8_byte_alphabet[0]).count() as f64;// the [0] is a quick hack to avoid to find an algorithm to compare a set of bytes with some bytes of different size.
            let sum_apparition_alphabet: f64 = apparition_count * (apparition_count - 1.0);
            let divide_characters: f64 = cipher_text_size * (cipher_text_size - 1.0);
            let result: f64 = sum_apparition_alphabet / divide_characters;
            iteration.insert(u8_byte_alphabet[0] as u8, result);// the [0] is a quick hack to avoid to find an algorithm to compare a set of bytes with some bytes of different size.
        }

        let coincidence_index: f64 = iteration.iter().map(|x| x.1).sum();

        coincidence_index
    }
}