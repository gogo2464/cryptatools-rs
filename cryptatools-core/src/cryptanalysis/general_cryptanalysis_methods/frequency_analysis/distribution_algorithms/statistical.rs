use std::collections::HashMap;
use crate::utils::convert;

pub struct Statistical {

}

impl Statistical {
    pub fn new() -> Self {
        Statistical {  

        }
    }
    /// Catch statistical distribution (percentage) from a encrypted text.
    /// 
    /// Simply count the occurences of a byte in a cipher text. 
    /// Then divide the result by the cipher text size.
    /// ```
    /// use std::collections::HashMap;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::{Statistical};
    /// use cryptatools_core::utils::alphabets;
    /// let stat_percentage = Statistical::guess_statistical_distribution(vec![0x41, 0x41, 0x41, 0x41, 0x41, 0x41], String::from(alphabets::ASCII_ALPHABET));
    /// assert_eq!(HashMap::from([(0x41 as u8, 1.0)]), stat_percentage);
    /// ```
    /// 
    /// ```
    /// use std::collections::HashMap;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::{Statistical};
    /// use cryptatools_core::utils::alphabets;
    /// let stat_percentage = Statistical::guess_statistical_distribution(vec![0x41, 0x41, 0x41, 0x41, 0x41, 0x42], String::from(alphabets::ASCII_ALPHABET));
    /// assert_eq!(HashMap::from([(0x42 as u8, 0.16666666666666666), (0x41 as u8, 0.8333333333333334)]), stat_percentage);
    /// ```
    pub fn guess_statistical_distribution(encrypted_input: Vec<u8>, alphabet: String) -> HashMap<u8, f64> {
        let mut distribution: HashMap<u8, f64> = HashMap::new();
        let u8_alphabet = convert::Encode::from_ascii_to_u8(alphabet.clone()).clone();

        for u8_encrypted_input in encrypted_input.iter() {
            *distribution.entry(u8_encrypted_input.clone()).or_insert(0.0) += 1.0;//or_default .or_insert(0.0)
        }

        for u8_alphabet_charcter in u8_alphabet.iter() {
            if distribution.get_mut(&u8_alphabet_charcter).is_some() {
                *distribution.get_mut(&u8_alphabet_charcter).unwrap() = distribution[u8_alphabet_charcter] / encrypted_input.len() as f64;
            }
        }
        
        distribution
    }
}