pub struct Decode {

}

impl Decode {
    ///  Decode a vector of u8 to ascii text string.
    ///
    ///  ```
    ///  use cryptatools_core::utils::convert;
    ///  let ascii: String = convert::Decode::from_u8_to_ascii(vec![0x41, 0x41, 0x41]);
    ///  assert_eq!("AAA", ascii);
    ///  ```
    /// 
    ///  ```
    ///  use cryptatools_core::utils::convert;
    ///  let ascii: String = convert::Decode::from_u8_to_ascii(vec![0x48, 0x69, 0x21]);
    ///  assert_eq!("Hi!", ascii);
    ///  ```
    pub fn from_u8_to_ascii(u8_input: Vec<u8>) -> String {
        String::from(std::str::from_utf8(u8_input.as_slice()).unwrap())
    }
}
pub struct Encode {

}
impl Encode {
    ///  Encode a string to a vector of u8 bytes.
    ///
    ///  ```
    ///  use cryptatools_core::utils::convert;
    ///  let bytes: Vec<u8> = convert::Encode::from_ascii_to_u8(String::from("AAA"));
    ///  assert_eq!(vec![0x41, 0x41, 0x41], bytes);
    ///  ```
    pub fn from_ascii_to_u8(ascii_input: String) -> Vec<u8> {
        let mut bytes = vec![];
        for character in ascii_input.chars() {
            bytes.push(char::from(character) as u8)
        }

        bytes
    }
}