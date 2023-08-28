use std::sync::Arc;
use num_bigfloat::BigFloat;
use num_traits::Pow;
use num::FromPrimitive;
use num_traits::Float;
use num::One;



use crate::utils::alphabets::Alphabet;
use crate::utils::alphabets::split_bytes_by_characters_representation;

pub struct BirtdayParadox {
    alphabet: Arc<Alphabet>,
}

impl BirtdayParadox {
    pub fn new(alphabet: Arc<Alphabet>) -> Self {
        BirtdayParadox {
            alphabet: alphabet,
        }
    }

    /// Calculate Birthday Paradox from hash. Focus of precision.
    /// 
    /// Method to get the percent of chance to get a collision to on a hash.
    /// Take the len of the hash, 
    /// 
    /// ```
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::hash_cryptanalysis::birthday_paradox::BirtdayParadox;
    /// use cryptatools_core::utils::alphabets::Alphabet;
    /// use cryptatools_core::utils::convert::Encode;
    /// 
    /// let lowercase_hexadecimal = Alphabet::new_empty().hexadecimal_ascii_lowercase_sixteen_bits_alphabet();
    /// let bp = BirtdayParadox::new(lowercase_hexadecimal.into());
    /// let mut hash = Encode::from_ascii_to_u8(String::from("1"));
    /// assert_eq!(hash, vec![49]);
    /// assert_eq!(bp.calculate_birtday_paradox_expecting_percent_focusing_on_precision(hash.clone(), 0.5), 5);//20
    /// assert_eq!(bp.calculate_birtday_paradox_expecting_percent_focusing_on_precision(hash.clone(), 0.95), 10);//39
    /// hash = Encode::from_ascii_to_u8(String::from("1f9"));
    /// assert_eq!(bp.calculate_birtday_paradox_expecting_percent_focusing_on_precision(hash.clone(), 0.95), 16060);
    /// hash = Encode::from_ascii_to_u8(String::from("1f90"));
    /// assert_eq!(bp.calculate_birtday_paradox_expecting_percent_focusing_on_precision(hash.clone(), 0.95), 160416);
    /// //let hash = Encode::from_ascii_to_u8(String::from("00000000"));
    /// //assert_eq!(bp.calculate_birtday_paradox_expecting_percent_focusing_on_precision(hash.clone(), 0.50), 500);
    /// //let hash = Encode::from_ascii_to_u8(String::from("1f9090aae28b8a3dceadf281b0f12828e676c326"));
    /// //assert_eq!(hash, vec![49, 102, 57, 48, 57, 48, 97, 97, 101, 50, 56, 98, 56, 97, 51, 100, 99, 101, 97, 100, 102, 50, 56, 49, 98, 48, 102, 49, 50, 56, 50, 56, 101, 54, 55, 54, 99, 51, 50, 54]);
    /// //assert_eq!(bp.calculate_birtday_paradox_expecting_percent_focusing_on_precision(hash.clone(), 0.5), 20);
    /// ```
    pub fn calculate_birtday_paradox_expecting_percent_focusing_on_precision(&self, hash_to_process: Vec<u8>, probability_expectation: f64) -> u64 {
        let organized_hash = split_bytes_by_characters_representation(&self.alphabet, hash_to_process); 
        let iter: usize = organized_hash.len() ;
        let alphabet_size = self.alphabet.get_encoding().len();

        let possibilities: BigFloat;
        if organized_hash.len() == 1 {
            possibilities = BigFloat::from_usize(iter).unwrap() * BigFloat::from_usize(alphabet_size).unwrap();
        } else {
            possibilities = BigFloat::from_usize(iter).unwrap().pow(BigFloat::from_usize(alphabet_size).unwrap());
        }
        
        let mut i = 0;
        let probability_expectation: BigFloat = BigFloat::from_f64(probability_expectation);
        let mut c = BigFloat::from_f64(1.0);
        while c > BigFloat::from_f64(1.0)-probability_expectation {
            c *= ((possibilities.clone()) - BigFloat::from(i as u32)) / possibilities.clone();
            i += 1;
        }

        i as u64
    }

    /// Calculate Birthday Paradox from hash in order to get number of attempts to try to get `probability_expectation` percent of chance of finding a colision. Focus on speed.
    /// 
    /// Method to get the percent of chance to get a collision to on a hash.
    /// Take the len of the hash, 
    /// Does not convert byte array to representation yet. 
    ///
    /// ```
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::hash_cryptanalysis::birthday_paradox::BirtdayParadox;
    /// use cryptatools_core::utils::alphabets::Alphabet;
    /// use cryptatools_core::utils::convert::Encode;
    /// 
    /// let lowercase_hexadecimal = Alphabet::new_empty().hexadecimal_ascii_lowercase_sixteen_bits_alphabet();
    /// let mut bp = BirtdayParadox::new(lowercase_hexadecimal.into());
    /// let hash = Encode::from_ascii_to_u8(String::from("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0w"));
    /// assert_eq!(hash, vec![98, 99, 49, 113, 120, 121, 50, 107, 103, 100, 121, 103, 106, 114, 115, 113, 116, 122, 113, 50, 110, 48, 121, 114, 102, 50, 52, 57, 51, 112, 56, 51, 107, 107, 102, 106, 104, 120, 48, 119]);
    /// assert_eq!(bp.calculate_birtday_paradox_expecting_percent_focusing_on_speed_with_taylor(hash.clone(), 0.50), 1.4234013764919992e24);// should be exactly 
    /// ```
    pub fn calculate_birtday_paradox_expecting_percent_focusing_on_speed_with_taylor(&self, hash_to_process: Vec<u8>, probability_expectation: f64) -> f64 {
        let alphabet_size = self.alphabet.get_encoding().len();
        let hash_output_bytes_len: u32 = hash_to_process.len().clone() as u32;

        let bits = BigFloat::from_usize(alphabet_size).unwrap()
            .pow(BigFloat::from_u32(hash_output_bytes_len));

        let probability = BigFloat::from_f64(probability_expectation);
        let days_in_year = bits;

        ((BigFloat::from(-2) * days_in_year * (BigFloat::one() - probability).ln()).sqrt()).to_f64()
    }


    pub fn calculate_birtday_paradox(&self, objects: u64, times: u64) -> u64 {
        self.calculate_permuted_choice_number(objects, times) / times
    }

    /// Calculate the number of possible choices of `objects` objects chosen `times` times.
    ///
    /// Amount of combination you could do at maximum if you have `times` attempts to try. Choose `objects` choices.
    /// The permuted choice method ignore when two choices are identical in a different order. Example: (1,2) has only one permuted choice because (1,2) and (2,1) are the same amount of choice in a different order.
    /// It is not a simple multiplication of two object.
    /// "`times` choose `objects`".
    ///
    /// arguments:
    ///   - `times`: number of attempts to try
    ///   - `object`: number of stuff to choose between
    ///
    /// Returns: the number of possible choices.
    ///
    /// ```
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
    /// use cryptatools_core::utils::alphabets::Alphabet;
    /// use cryptatools_core::utils::convert::Encode;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::hash_cryptanalysis::birthday_paradox::BirtdayParadox;
    /// 
    /// let ascii_alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// let bp = BirtdayParadox::new(ascii_alphabet.into());
    /// assert_eq!(bp.calculate_permuted_choice_number(5, 2), 10);
    /// assert_eq!(bp.calculate_permuted_choice_number(2, 2), 1);
    /// ```
    pub fn calculate_permuted_choice_number(&self, objects: u64, times: u64) -> u64 {
        self.factorial(objects) / (self.factorial(objects - times) * self.factorial(times))
    }

    /// Get number factorial.
    ///
    /// ```
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
    /// use cryptatools_core::utils::alphabets::Alphabet;
    /// use cryptatools_core::utils::convert::Encode;
    /// use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::hash_cryptanalysis::birthday_paradox::BirtdayParadox;
    /// 
    /// let ascii_alphabet = Alphabet::new_empty().ascii_printable_only_encoding();
    /// let bp = BirtdayParadox::new(ascii_alphabet.into());
    /// assert_eq!(bp.factorial(0), 1);
    /// assert_eq!(bp.factorial(1), 1);
    /// assert_eq!(bp.factorial(2), 2);
    /// assert_eq!(bp.factorial(3), 6);
    ///```
    pub fn factorial(&self, number: u64) -> u64 {
        if number == 0 {
            return 1;
        }

        (1..=number).product()
    }
}