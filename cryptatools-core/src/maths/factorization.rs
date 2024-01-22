use num_bigint::BigUint;
use num_bigint::BigInt;
use num::FromPrimitive;

pub struct Factorization {

}

impl Factorization {

  /// ```
  /// use cryptatools_core::maths::factorization;
  /// use num_bigint::BigUint;
  /// 
  /// let mut factor = factorization::factor(742449129124467073921545687640895127535705902454369756401331);
  /// assert_eq!(factor, vec![123]);
  ///
  /// ```
  pub fn factor(n_input: BigInt) -> Vec<BigInt> {

    let mut factors = vec![];
    let mut divisor = 2;


    let mut n_input = n_input;
    while n_input >= BigInt::from_u8(2).unwrap() {
        if n_input.clone() % divisor == BigInt::from_u8(0).unwrap() {
          factors.push(BigInt::from(divisor));
          n_input = n_input / divisor;
        } else {
          divisor += 1;
        }
      }
      
      factors
    }
}


