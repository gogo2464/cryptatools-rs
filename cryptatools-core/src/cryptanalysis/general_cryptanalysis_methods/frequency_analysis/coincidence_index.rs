use std::{collections::HashMap, fs, path::Path};
use once_cell::sync::Lazy;
use crate::cryptography::encryption::polyalphabetic_ciphers::vigenere::Vigenere;
use crate::utils::convert::{Encode};

use rand::Rng;

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
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
    /// use cryptatools_core::utils::{alphabets::ASCII_ALPHABET, alphabets::PRINTABLE_ASCII_ALPHABET};
    /// use cryptatools_core::utils::convert::Encode;
    /// use once_cell::sync::Lazy;
    /// 
    /// use approx::assert_abs_diff_eq;
    /// 
    /// let plain_text = String::from("Hello! How are you? I am fine and you?");
    /// let pseudo_cipher_text = Encode::from_ascii_to_u8(plain_text);
    /// let c = CoincidenceIndexGuesser::new(Lazy::force(&PRINTABLE_ASCII_ALPHABET).to_owned());
    /// let coincidence_index: f64 = c.guess_coincidence_index(pseudo_cipher_text);
    /// assert_abs_diff_eq!(coincidence_index, 0.06543385490753911, epsilon = 1e-16);
    /// ```
    pub fn guess_coincidence_index(&self, cipher_text_input: Vec<u8>) -> f64 {
        let cipher_text_size: f64 = cipher_text_input.len() as f64;
        if cipher_text_size <= 200.0 {
            println!("Warning: The cipher text input is {0} bytes. Probably {0} characters. It may be too short. You should provide more input characters.", cipher_text_size);
        }

        let mut coincidence_index: f64 = 0.0;
        for u8_byte_alphabet in self.alphabet.values() {//TODO: convert to f64
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
    /// use cryptatools_core::utils::alphabets::ASCII_ALPHABET;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
    /// use approx::assert_abs_diff_eq;
    /// 
    /// let path = String::from("./data/text-corpus-for-statistics/gutenberg/austen-emma.txt");
    /// let mut c = CoincidenceIndexGuesser::new(Lazy::force(&ASCII_ALPHABET).to_owned());
    /// assert_abs_diff_eq!(c.guess_coincidence_index_statistics_from_file(path.clone()), 0.06525540393695795, epsilon = 1e-16);
    /// ```
    pub fn guess_coincidence_index_statistics_from_file(&self, file_name: String) -> f64 {
        let file_path = Path::new(&file_name);
        let file_content = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(error) => panic!("{0}", error),
        };

        let bytes_file_content = Encode::from_ascii_to_u8(file_content.replace(r"\r\n", r"\n"));
        let coincidence_index = self.guess_coincidence_index(bytes_file_content);
        
        coincidence_index
    }
}

pub struct CoincidenceIndexGenerator {
    alphabet: HashMap<String, Vec<u8>>,
}

impl CoincidenceIndexGenerator {
    pub fn new(alphabet: HashMap<String, Vec<u8>>) -> Self {
        CoincidenceIndexGenerator {
            alphabet: alphabet,
        }
    }
    /// Generate coincidence index of a specific key_size choosen poorly for a specified plain text
    /// 
    /// This function does not only aim to be used against cipher text.
    /// The goal of this function is also to provide statistics about plain text coincidence index in order to decipher another encrypted text.
    /// 
    /// ```
    /// use once_cell::sync::Lazy;
    /// use cryptatools_core::utils::{convert::Encode, alphabets::ASCII_ALPHABET, alphabets::PRINTABLE_ASCII_ALPHABET};
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGenerator;
    /// 
    /// use approx::assert_abs_diff_eq;
    /// 
    /// let mut vcig = CoincidenceIndexGenerator::new(Lazy::force(&PRINTABLE_ASCII_ALPHABET).to_owned());
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
            let random_value = rng.gen_range(0..self.alphabet.len());
            let random_byte = self.alphabet.values().nth(random_value).unwrap()[0];// TODO get multichars // .sorted() ?
            key.push(vec![random_byte]);
        }

        let vig: Vigenere = Vigenere::new(self.alphabet.to_owned());
        let vigenere_coincidence_index_guesser = CoincidenceIndexGuesser::new(self.alphabet.to_owned());
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
    /// use cryptatools_core::utils::{convert::Encode,  alphabets::PRINTABLE_ASCII_ALPHABET};
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGenerator;
    /// use approx::assert_abs_diff_eq;
    /// 
    /// let mut vcig = CoincidenceIndexGenerator::new(Lazy::force(&PRINTABLE_ASCII_ALPHABET).to_owned());
    /// let mut ci = vcig.generate_coincidence_index_for_key_from_file(5, String::from("./data/text-corpus-for-statistics/gutenberg/austen-emma.txt"));
    /// assert_abs_diff_eq!(ci, 0.01, epsilon = 1e-1);
    /// ```
    pub fn generate_coincidence_index_for_key_from_file(&self, key_size: usize, plain_text_ascii_file_input: String) -> f64 {
        let file_path = Path::new(&plain_text_ascii_file_input);
        let file_content = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(error) => panic!("{0}", error),
        };

        let coincidence_index_found = self.generate_coincidence_index_for_key(key_size, Encode::from_ascii_to_u8(file_content));
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