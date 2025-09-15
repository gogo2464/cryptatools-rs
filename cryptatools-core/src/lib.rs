pub mod cryptanalysis;
pub mod cryptography;
pub mod utils;

use crate::utils::alphabets::{Encoding, Alphabet, split_bytes_by_characters_representation, uniffy_opcode_group};
use crate::cryptography::classical::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
use crate::cryptography::classical::encryption::transpositional_ciphers::columnar_transposition::ColumnarTranspositionAlgorithm;
use crate::cryptanalysis::custom::general_cryptanalysis_methods::statistic_analysis::frequency_analysis::coincidence_index::engine::coincidence_index::CoincidenceIndexGuesser;
use crate::cryptanalysis::custom::general_cryptanalysis_methods::statistic_analysis::probability::birthday_paradox::BirthdayParadox;

uniffi_macros::include_scaffolding!("cryptatools");