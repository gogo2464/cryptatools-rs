use std::collections::HashMap;

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

    /*pub fn decode(alphabet: HashMap<String, Vec<u8>>, encoded: Vec<u8>) -> String {

    }*/
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


    /// Encode the input argument `unencoded` to a byte according the `alphabet`.
    /// 
    /// ```
    /// use once_cell::sync::Lazy;
    /// use cryptatools_core::utils::convert;
    /// use cryptatools_core::utils::alphabets::{ASCII_ALPHABET, POKERED_ALPHABET};
    /// let encoded: Vec<u8> = convert::Encode::encode(Lazy::force(&ASCII_ALPHABET).to_owned(), String::from("ABCDEFGH"));
    /// assert_eq!(encoded, vec![65, 66, 67, 68, 69, 70, 71, 72]);
    /// 
    /// let pokered_encoded = convert::Encode::encode(Lazy::force(&POKERED_ALPHABET).to_owned(), String::from("<NULL><PAGE><PKMN>"));
    /// assert_eq!(pokered_encoded, vec![0, 73, 74]);
    /// ```
    pub fn encode(alphabet: HashMap<String, Vec<u8>>, unecoded: String) -> Vec<u8> {
        let mut encoded = vec![];
        let mut stack = String::from("");

        for c in unecoded.chars() {
            if alphabet.contains_key(&String::from(c)) {
                for encoded_byte in alphabet[&String::from(c)].clone() {
                    encoded.push(encoded_byte);
                }
                stack = String::from("");
            } else {
                stack.push_str(&String::from(c));
                if alphabet.contains_key(&stack) {
                    for encoded_byte in alphabet[&stack].clone() {
                        encoded.push(encoded_byte);
                    }
                    stack = String::from("");
                }
            }
        }

        encoded
    }
}