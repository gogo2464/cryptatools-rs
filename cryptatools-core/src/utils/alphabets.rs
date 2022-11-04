use std::collections::HashMap;
use once_cell::sync::Lazy;

///```
/// use cryptatools_core::utils::{convert::Encode, alphabets::split_bytes_by_characters_representation, alphabets::ASCII_ALPHABET};
/// use once_cell::sync::Lazy;
/// let plaintext = split_bytes_by_characters_representation(Lazy::force(&ASCII_ALPHABET).to_owned(), Encode::from_ascii_to_u8(String::from("the quick brown roman fox jumped over the lazy ostrogoth dog")));
/// assert_eq!(plaintext, [[116], [104], [101], [32], [113], [117], [105], [99], [107], [32], [98], [114], [111], [119], [110], [32], [114], [111], [109], [97], [110], [32], [102], [111], [120], [32], [106], [117], [109], [112], [101], [100], [32], [111], [118], [101], [114], [32], [116], [104], [101], [32], [108], [97], [122], [121], [32], [111], [115], [116], [114], [111], [103], [111], [116], [104], [32], [100], [111], [103]]);
/// assert_eq!(plaintext.len(), String::from("the quick brown roman fox jumped over the lazy ostrogoth dog").len());
/// ```

pub fn split_bytes_by_characters_representation(alphabet: HashMap<String, Vec<u8>>, text: Vec<u8>) -> Vec<Vec<u8>> {
    let mut set_of_chars: Vec<Vec<u8>> = vec![];
    let mut character_byte_representation_size: usize = 0;
    for byte in text {
        for alphabet_character in alphabet.keys() {
            if character_byte_representation_size > 0 {
                character_byte_representation_size = character_byte_representation_size-1;
                continue;
            }

            if alphabet.get(alphabet_character).unwrap().len() > 1 {
                set_of_chars.push(alphabet.get(alphabet_character).unwrap().to_vec());
                character_byte_representation_size = alphabet.get(alphabet_character).unwrap().len() - 1;
                continue;
            } else if alphabet.get(alphabet_character).unwrap().len() == 1 {
                set_of_chars.push(vec![byte]);
                break;
            }
        }
    }

    set_of_chars
}

pub fn uniffy_opcode_group(text: Vec<Vec<u8>>) -> Vec<u8> {
    text.concat()
}

pub const PRINTABLE: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c";
//pub const ASCII_ALPHABET: &'static str = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f\\u128";
pub const UU_ENCODING_ALPHABET: &'static str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`";


pub static POKERED_ALPHABET: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(|| {
    let mut alphabet = HashMap::new();
    alphabet.insert(String::from("<NULL>"), vec![0x00]);
    alphabet.insert(String::from("<PAGE>"), vec![0x49]);
    alphabet.insert(String::from("<PKMN>"), vec![0x4a]);

    alphabet
  }
);

pub static ASCII_ALPHABET: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(|| {
    let mut alphabet = HashMap::new();
    alphabet.insert(String::from("\x00"), vec![0x00]);
    alphabet.insert(String::from("\x01"), vec![0x01]);
    alphabet.insert(String::from("\x02"), vec![0x02]);
    alphabet.insert(String::from("\x03"), vec![0x03]);
    alphabet.insert(String::from("\x04"), vec![0x04]);
    alphabet.insert(String::from("\x05"), vec![0x05]);
    alphabet.insert(String::from("\x06"), vec![0x06]);
    alphabet.insert(String::from("\x07"), vec![0x07]);
    alphabet.insert(String::from("\x08"), vec![0x08]);
    alphabet.insert(String::from("\x09"), vec![0x09]);
    alphabet.insert(String::from("\x0a"), vec![0x0a]);
    alphabet.insert(String::from("\x0b"), vec![0x0b]);
    alphabet.insert(String::from("\x0c"), vec![0x0c]);
    alphabet.insert(String::from("\x0d"), vec![0x0d]);
    alphabet.insert(String::from("\x0e"), vec![0x0e]);
    alphabet.insert(String::from("\x0f"), vec![0x0f]);
    alphabet.insert(String::from("\x10"), vec![0x10]);
    alphabet.insert(String::from("\x11"), vec![0x11]);
    alphabet.insert(String::from("\x12"), vec![0x12]);
    alphabet.insert(String::from("\x13"), vec![0x13]);
    alphabet.insert(String::from("\x14"), vec![0x14]);
    alphabet.insert(String::from("\x15"), vec![0x15]);
    alphabet.insert(String::from("\x16"), vec![0x16]);
    alphabet.insert(String::from("\x17"), vec![0x17]);
    alphabet.insert(String::from("\x18"), vec![0x18]);
    alphabet.insert(String::from("\x19"), vec![0x19]);
    alphabet.insert(String::from("\x1a"), vec![0x1a]);
    alphabet.insert(String::from("\x1b"), vec![0x1b]);
    alphabet.insert(String::from("\x1c"), vec![0x1c]);
    alphabet.insert(String::from("\x1d"), vec![0x1d]);
    alphabet.insert(String::from("\x1e"), vec![0x1e]);
    alphabet.insert(String::from("\x1f"), vec![0x1f]);
    alphabet.insert(String::from(" "), vec![0x20]);
    alphabet.insert(String::from("!"), vec![0x21]);
    alphabet.insert(String::from("\""), vec![0x22]);
    alphabet.insert(String::from("#"), vec![0x23]);
    alphabet.insert(String::from("$"), vec![0x24]);
    alphabet.insert(String::from("%"), vec![0x25]);
    alphabet.insert(String::from("&"), vec![0x26]);
    alphabet.insert(String::from("'"), vec![0x27]);
    alphabet.insert(String::from("("), vec![0x28]);
    alphabet.insert(String::from(")"), vec![0x29]);
    alphabet.insert(String::from("*"), vec![0x2a]);
    alphabet.insert(String::from("+"), vec![0x2b]);
    alphabet.insert(String::from(","), vec![0x2c]);
    alphabet.insert(String::from("-"), vec![0x2d]);
    alphabet.insert(String::from("."), vec![0x2e]);
    alphabet.insert(String::from("/"), vec![0x2f]);
    alphabet.insert(String::from("0"), vec![0x30]);
    alphabet.insert(String::from("1"), vec![0x31]);
    alphabet.insert(String::from("2"), vec![0x32]);
    alphabet.insert(String::from("3"), vec![0x33]);
    alphabet.insert(String::from("4"), vec![0x34]);
    alphabet.insert(String::from("5"), vec![0x35]);
    alphabet.insert(String::from("6"), vec![0x36]);
    alphabet.insert(String::from("7"), vec![0x37]);
    alphabet.insert(String::from("8"), vec![0x38]);
    alphabet.insert(String::from("9"), vec![0x39]);
    alphabet.insert(String::from(":"), vec![0x3a]);
    alphabet.insert(String::from(";"), vec![0x3b]);
    alphabet.insert(String::from("<"), vec![0x3c]);
    alphabet.insert(String::from("="), vec![0x3d]);
    alphabet.insert(String::from(">"), vec![0x3e]);
    alphabet.insert(String::from("?"), vec![0x3f]);
    alphabet.insert(String::from("@"), vec![0x40]);
    alphabet.insert(String::from("A"), vec![0x41]);
    alphabet.insert(String::from("B"), vec![0x42]);
    alphabet.insert(String::from("C"), vec![0x43]);
    alphabet.insert(String::from("D"), vec![0x44]);
    alphabet.insert(String::from("E"), vec![0x45]);
    alphabet.insert(String::from("F"), vec![0x46]);
    alphabet.insert(String::from("G"), vec![0x47]);
    alphabet.insert(String::from("H"), vec![0x48]);
    alphabet.insert(String::from("I"), vec![0x49]);
    alphabet.insert(String::from("J"), vec![0x4a]);
    alphabet.insert(String::from("K"), vec![0x4b]);
    alphabet.insert(String::from("L"), vec![0x4c]);
    alphabet.insert(String::from("M"), vec![0x4d]);
    alphabet.insert(String::from("N"), vec![0x4e]);
    alphabet.insert(String::from("O"), vec![0x4f]);
    alphabet.insert(String::from("P"), vec![0x50]);
    alphabet.insert(String::from("Q"), vec![0x51]);
    alphabet.insert(String::from("R"), vec![0x52]);
    alphabet.insert(String::from("S"), vec![0x53]);
    alphabet.insert(String::from("T"), vec![0x54]);
    alphabet.insert(String::from("U"), vec![0x55]);
    alphabet.insert(String::from("V"), vec![0x56]);
    alphabet.insert(String::from("W"), vec![0x57]);
    alphabet.insert(String::from("X"), vec![0x58]);
    alphabet.insert(String::from("Y"), vec![0x59]);
    alphabet.insert(String::from("Z"), vec![0x5a]);
    alphabet.insert(String::from("["), vec![0x5b]);
    alphabet.insert(String::from("\\"), vec![0x5c]);
    alphabet.insert(String::from("]"), vec![0x5d]);
    alphabet.insert(String::from("^"), vec![0x5e]);
    alphabet.insert(String::from("_"), vec![0x5f]);
    alphabet.insert(String::from("`"), vec![0x60]);
    alphabet.insert(String::from("a"), vec![0x61]);
    alphabet.insert(String::from("b"), vec![0x62]);
    alphabet.insert(String::from("c"), vec![0x63]);
    alphabet.insert(String::from("d"), vec![0x64]);
    alphabet.insert(String::from("e"), vec![0x65]);
    alphabet.insert(String::from("f"), vec![0x66]);
    alphabet.insert(String::from("g"), vec![0x67]);
    alphabet.insert(String::from("h"), vec![0x68]);
    alphabet.insert(String::from("i"), vec![0x69]);
    alphabet.insert(String::from("j"), vec![0x6a]);
    alphabet.insert(String::from("k"), vec![0x6b]);
    alphabet.insert(String::from("l"), vec![0x6c]);
    alphabet.insert(String::from("m"), vec![0x6d]);
    alphabet.insert(String::from("n"), vec![0x6e]);
    alphabet.insert(String::from("o"), vec![0x6f]);
    alphabet.insert(String::from("p"), vec![0x70]);
    alphabet.insert(String::from("q"), vec![0x71]);
    alphabet.insert(String::from("r"), vec![0x72]);
    alphabet.insert(String::from("s"), vec![0x73]);
    alphabet.insert(String::from("t"), vec![0x74]);
    alphabet.insert(String::from("u"), vec![0x75]);
    alphabet.insert(String::from("v"), vec![0x76]);
    alphabet.insert(String::from("w"), vec![0x77]);
    alphabet.insert(String::from("x"), vec![0x78]);
    alphabet.insert(String::from("y"), vec![0x79]);
    alphabet.insert(String::from("z"), vec![0x7a]);
    alphabet.insert(String::from("{"), vec![0x7b]);
    alphabet.insert(String::from("|"), vec![0x7c]);
    alphabet.insert(String::from("}"), vec![0x7d]);
    alphabet.insert(String::from("~"), vec![0x7e]);
    alphabet.insert(String::from("\x7f"), vec![0x7f]);
    alphabet.insert(String::from("\\u128"), vec![0x80]);

    alphabet
  }
);

pub static MAJ_NO_SPACE_ASCII_ALPHABET: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(|| {
    let mut alphabet = HashMap::new();
    alphabet.insert(String::from("A"), vec![0x41]);
    alphabet.insert(String::from("B"), vec![0x42]);
    alphabet.insert(String::from("C"), vec![0x43]);
    alphabet.insert(String::from("D"), vec![0x44]);
    alphabet.insert(String::from("E"), vec![0x45]);
    alphabet.insert(String::from("F"), vec![0x46]);
    alphabet.insert(String::from("G"), vec![0x47]);
    alphabet.insert(String::from("H"), vec![0x48]);
    alphabet.insert(String::from("I"), vec![0x49]);
    alphabet.insert(String::from("J"), vec![0x4a]);
    alphabet.insert(String::from("K"), vec![0x4b]);
    alphabet.insert(String::from("L"), vec![0x4c]);
    alphabet.insert(String::from("M"), vec![0x4d]);
    alphabet.insert(String::from("N"), vec![0x4e]);
    alphabet.insert(String::from("O"), vec![0x4f]);
    alphabet.insert(String::from("P"), vec![0x50]);
    alphabet.insert(String::from("Q"), vec![0x51]);
    alphabet.insert(String::from("R"), vec![0x52]);
    alphabet.insert(String::from("S"), vec![0x53]);
    alphabet.insert(String::from("T"), vec![0x54]);
    alphabet.insert(String::from("U"), vec![0x55]);
    alphabet.insert(String::from("V"), vec![0x56]);
    alphabet.insert(String::from("W"), vec![0x57]);
    alphabet.insert(String::from("X"), vec![0x58]);
    alphabet.insert(String::from("Y"), vec![0x59]);
    alphabet.insert(String::from("Z"), vec![0x5a]);

    alphabet
  }
);

pub static PRINTABLE_ASCII_ALPHABET: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(|| {
    let mut alphabet = HashMap::new();
    alphabet.insert(String::from(" "), vec![0x20]);
    alphabet.insert(String::from("!"), vec![0x21]);
    alphabet.insert(String::from("\""), vec![0x22]);
    alphabet.insert(String::from("#"), vec![0x23]);
    alphabet.insert(String::from("$"), vec![0x24]);
    alphabet.insert(String::from("%"), vec![0x25]);
    alphabet.insert(String::from("&"), vec![0x26]);
    alphabet.insert(String::from("'"), vec![0x27]);
    alphabet.insert(String::from("("), vec![0x28]);
    alphabet.insert(String::from(")"), vec![0x29]);
    alphabet.insert(String::from("*"), vec![0x2a]);
    alphabet.insert(String::from("+"), vec![0x2b]);
    alphabet.insert(String::from(","), vec![0x2c]);
    alphabet.insert(String::from("-"), vec![0x2d]);
    alphabet.insert(String::from("."), vec![0x2e]);
    alphabet.insert(String::from("/"), vec![0x2f]);
    alphabet.insert(String::from("0"), vec![0x30]);
    alphabet.insert(String::from("1"), vec![0x31]);
    alphabet.insert(String::from("2"), vec![0x32]);
    alphabet.insert(String::from("3"), vec![0x33]);
    alphabet.insert(String::from("4"), vec![0x34]);
    alphabet.insert(String::from("5"), vec![0x35]);
    alphabet.insert(String::from("6"), vec![0x36]);
    alphabet.insert(String::from("7"), vec![0x37]);
    alphabet.insert(String::from("8"), vec![0x38]);
    alphabet.insert(String::from("9"), vec![0x39]);
    alphabet.insert(String::from(":"), vec![0x3a]);
    alphabet.insert(String::from(";"), vec![0x3b]);
    alphabet.insert(String::from("<"), vec![0x3c]);
    alphabet.insert(String::from("="), vec![0x3d]);
    alphabet.insert(String::from(">"), vec![0x3e]);
    alphabet.insert(String::from("?"), vec![0x3f]);
    alphabet.insert(String::from("@"), vec![0x40]);
    alphabet.insert(String::from("A"), vec![0x41]);
    alphabet.insert(String::from("B"), vec![0x42]);
    alphabet.insert(String::from("C"), vec![0x43]);
    alphabet.insert(String::from("D"), vec![0x44]);
    alphabet.insert(String::from("E"), vec![0x45]);
    alphabet.insert(String::from("F"), vec![0x46]);
    alphabet.insert(String::from("G"), vec![0x47]);
    alphabet.insert(String::from("H"), vec![0x48]);
    alphabet.insert(String::from("I"), vec![0x49]);
    alphabet.insert(String::from("J"), vec![0x4a]);
    alphabet.insert(String::from("K"), vec![0x4b]);
    alphabet.insert(String::from("L"), vec![0x4c]);
    alphabet.insert(String::from("M"), vec![0x4d]);
    alphabet.insert(String::from("N"), vec![0x4e]);
    alphabet.insert(String::from("O"), vec![0x4f]);
    alphabet.insert(String::from("P"), vec![0x50]);
    alphabet.insert(String::from("Q"), vec![0x51]);
    alphabet.insert(String::from("R"), vec![0x52]);
    alphabet.insert(String::from("S"), vec![0x53]);
    alphabet.insert(String::from("T"), vec![0x54]);
    alphabet.insert(String::from("U"), vec![0x55]);
    alphabet.insert(String::from("V"), vec![0x56]);
    alphabet.insert(String::from("W"), vec![0x57]);
    alphabet.insert(String::from("X"), vec![0x58]);
    alphabet.insert(String::from("Y"), vec![0x59]);
    alphabet.insert(String::from("Z"), vec![0x5a]);
    alphabet.insert(String::from("["), vec![0x5b]);
    alphabet.insert(String::from("\\"), vec![0x5c]);
    alphabet.insert(String::from("]"), vec![0x5d]);
    alphabet.insert(String::from("^"), vec![0x5e]);
    alphabet.insert(String::from("_"), vec![0x5f]);
    alphabet.insert(String::from("`"), vec![0x60]);
    alphabet.insert(String::from("a"), vec![0x61]);
    alphabet.insert(String::from("b"), vec![0x62]);
    alphabet.insert(String::from("c"), vec![0x63]);
    alphabet.insert(String::from("d"), vec![0x64]);
    alphabet.insert(String::from("e"), vec![0x65]);
    alphabet.insert(String::from("f"), vec![0x66]);
    alphabet.insert(String::from("g"), vec![0x67]);
    alphabet.insert(String::from("h"), vec![0x68]);
    alphabet.insert(String::from("i"), vec![0x69]);
    alphabet.insert(String::from("j"), vec![0x6a]);
    alphabet.insert(String::from("k"), vec![0x6b]);
    alphabet.insert(String::from("l"), vec![0x6c]);
    alphabet.insert(String::from("m"), vec![0x6d]);
    alphabet.insert(String::from("n"), vec![0x6e]);
    alphabet.insert(String::from("o"), vec![0x6f]);
    alphabet.insert(String::from("p"), vec![0x70]);
    alphabet.insert(String::from("q"), vec![0x71]);
    alphabet.insert(String::from("r"), vec![0x72]);
    alphabet.insert(String::from("s"), vec![0x73]);
    alphabet.insert(String::from("t"), vec![0x74]);
    alphabet.insert(String::from("u"), vec![0x75]);
    alphabet.insert(String::from("v"), vec![0x76]);
    alphabet.insert(String::from("w"), vec![0x77]);
    alphabet.insert(String::from("x"), vec![0x78]);
    alphabet.insert(String::from("y"), vec![0x79]);
    alphabet.insert(String::from("z"), vec![0x7a]);
    alphabet.insert(String::from("{"), vec![0x7b]);
    alphabet.insert(String::from("|"), vec![0x7c]);
    alphabet.insert(String::from("}"), vec![0x7d]);
    alphabet.insert(String::from("~"), vec![0x7e]);

    alphabet
  }
);