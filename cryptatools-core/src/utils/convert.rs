///  Convert a vector of u8 to ascii text string.
///
///  ```
///  use crate::cryptatools_core::utils::convert::from_u8_to_ascii;
///  let ascii: = from_u8_to_ascii(vec![0x41, 0x41, 0x41])
///  assert_eq!("AAA", ascii);
///  ```
pub fn from_u8_to_ascii(u8_input: Vec<u8>) -> String {
    let output_ascii: String = String::from(""); 
    for character_integer in u8_input {
        output_ascii.push_str(character_integer.into());
    }

    output_ascii
}