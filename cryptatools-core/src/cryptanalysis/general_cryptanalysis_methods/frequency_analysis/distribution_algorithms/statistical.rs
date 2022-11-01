use std::collections::HashMap;

pub struct Statistical {
    alphabet: HashMap<String, Vec<u8>>,
}

impl Statistical {
    pub fn new(alphabet: HashMap<String, Vec<u8>>) -> Self {
        Statistical {
            alphabet: alphabet,
        }
    }
    /// Catch statistical distribution (percentage) from a encrypted text.
    /// 
    /// Simply count the occurences of a byte in a cipher text. 
    /// Then divide the result by the cipher text size.
    /// ```
    /// use std::collections::HashMap;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::{Statistical};
    /// use once_cell::sync::Lazy;
    /// use cryptatools_core::utils::alphabets::ASCII_ALPHABET;
    /// let stat = Statistical::new(Lazy::force(&ASCII_ALPHABET).to_owned());
    /// let stat_percentage = stat.guess_statistical_distribution(vec![0x41, 0x41, 0x41, 0x41, 0x41, 0x41]);
    /// assert_eq!(stat_percentage, HashMap::from([(vec![0x41 as u8], 1.0)]));
    /// ```
    /// 
    /// ```
    /// use std::collections::HashMap;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::{Statistical};
    /// use once_cell::sync::Lazy;
    /// use cryptatools_core::utils::alphabets::ASCII_ALPHABET;
    /// let stat = Statistical::new(Lazy::force(&ASCII_ALPHABET).to_owned());
    /// let stat_percentage = stat.guess_statistical_distribution(vec![0x41, 0x41, 0x41, 0x41, 0x41, 0x42]);
    /// assert_eq!(stat_percentage, HashMap::from([(vec![0x42 as u8], 0.16666666666666666), (vec![0x41 as u8], 0.8333333333333334)]));
    /// ```
    pub fn guess_statistical_distribution(&self, encrypted_input: Vec<u8>) -> HashMap<Vec<u8>, f64> {
        let mut distribution: HashMap<Vec<u8>, f64> = HashMap::new();
        let u8_alphabet = self.alphabet.values();

        for u8_encrypted_input in encrypted_input.iter() {
            *distribution.entry(vec![u8_encrypted_input.clone()]).or_insert(0.0) += 1.0;//or_default .or_insert(0.0)
        }

        for u8_alphabet_charcter in u8_alphabet {
            if distribution.get_mut(&u8_alphabet_charcter.clone()).is_some() {
                *distribution.get_mut(&u8_alphabet_charcter.clone()).unwrap() = distribution[u8_alphabet_charcter] / encrypted_input.len() as f64;
            }
        }
        
        distribution
    }
}