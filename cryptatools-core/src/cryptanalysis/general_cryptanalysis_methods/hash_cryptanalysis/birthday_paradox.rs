use once_cell::sync::Lazy;
use std::sync::Arc;
use crate::utils::alphabets::Alphabet;

struct BirtdayParadox {
    alphabet: Arc<Alphabet>,
}

impl BirtdayParadox {
    pub fn new(alphabet: Arc<Alphabet>) -> Self {
        BirtdayParadox {
            alphabet: alphabet,
        }
    }
}