///  Convert a vector of u8 to ascii text string.
///
///  ```
///  use crate::cryptatools_core::utils::convert::from_u8_to_ascii;
///  let ascii: String = from_u8_to_ascii(vec![0x41, 0x41, 0x41]);
///  assert_eq!("AAA", ascii);
///  ```
/// 
///  ```
///  use crate::cryptatools_core::utils::convert::from_u8_to_ascii;
///  let ascii: String = from_u8_to_ascii(vec![0x48, 0x69, 0x21]);
///  assert_eq!("Hi!", ascii);
///  ```
pub fn from_u8_to_ascii(u8_input: Vec<u8>) -> String {
    String::from(std::str::from_utf8(u8_input.as_slice()).unwrap())
}