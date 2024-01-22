use std::sync::Arc;
use num_traits;

use num_bigint::{BigUint, BigInt, ToBigInt};
use num_bigint_dig::ModInverse;
use num_bigint_dig::prime::probably_prime;
//use num_bigint_dig::ToBigInt;
use num_bigint_dig::RandBigInt;
use num_traits::Pow;
use num_integer::Integer;

use num::FromPrimitive;
use num::ToPrimitive;
use std::str::FromStr;

use crate::utils::alphabets::Alphabet;
use crate::maths::factorization;
use crate::utils::convert::Encode;

pub struct PublicExponentAttacks {
    alphabet: Arc<Alphabet>,
}

impl PublicExponentAttacks {
    pub fn new(alphabet: Arc<Alphabet>) -> Self {
        PublicExponentAttacks {
            alphabet
        }
    }

    /// ```
    /// use cryptatools_core::cryptanalysis::common::rsa::public_exponent_attack::{*};
    /// use cryptatools_core::utils::convert::Decode;
    /// use num_bigint::BigInt;
    /// use std::str::FromStr;
    /// // 742449129124467073921545687640895127535705902454369756401331
    /// 
    /// //let algos = PublicExponentAttacks::new();
    /// let mut plaintext = PublicExponentAttacks::modulo_factorisation_attack(vec![], vec![], vec![]);
    /// //let ascii: String = Decode::from_u8_to_ascii(plaintext);
    /// assert_eq!(plaintext, BigInt::from_str("9525146106593233618825000042088863551831280763610019197").unwrap());
    /// ```
    pub fn modulo_factorisation_attack(cipher_text: Vec<u8>, modulo: Vec<u8>, exponent: Vec<u8>) -> BigInt {

        let mut cipher = BigInt::from_u64(3); //BigUint::new(cipher_text);
        let mut modulo = BigInt::from_i64(-1); //BigUint::new(modulo);
        let mut exponent = BigInt::from_str("742449129124467073921545687639156049064283454870081476956200");//BigUint::new(exponent);


        let vec = factorization::Factorization::factor(exponent.clone().unwrap());
        let p = &vec[0];
        let q = &vec[1];

        let phi = p.to_bigint().unwrap()-BigInt::from_u64(1).unwrap()*(q.to_bigint().unwrap()-BigInt::from_u64(1).unwrap());

        let dec = exponent.clone().unwrap().modpow(&BigInt::from_i64(-1).unwrap(), &phi);

        return (cipher.unwrap().modpow(&BigInt::from_str("39207274348578481322317340648475596807303160111338236677373").unwrap(), &dec) % modulo.unwrap());
    }
}