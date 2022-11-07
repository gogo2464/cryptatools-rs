use itertools::Itertools;
use bimap::btree::BiBTreeMap;

///```
/// use cryptatools_core::utils::{convert::Encode, alphabets::split_bytes_by_characters_representation, alphabets::Alphabet};
/// 
/// let ascii_alphabet = Alphabet::new_empty().ascii_encoding();
/// let plaintext = split_bytes_by_characters_representation(&ascii_alphabet, Encode::from_ascii_to_u8(String::from("the quick brown roman fox jumped over the lazy ostrogoth dog")));
/// assert_eq!(plaintext, [[116], [104], [101], [32], [113], [117], [105], [99], [107], [32], [98], [114], [111], [119], [110], [32], [114], [111], [109], [97], [110], [32], [102], [111], [120], [32], [106], [117], [109], [112], [101], [100], [32], [111], [118], [101], [114], [32], [116], [104], [101], [32], [108], [97], [122], [121], [32], [111], [115], [116], [114], [111], [103], [111], [116], [104], [32], [100], [111], [103]]);
/// assert_eq!(plaintext.len(), String::from("the quick brown roman fox jumped over the lazy ostrogoth dog").len());
/// ```
pub fn split_bytes_by_characters_representation(encoding: &Alphabet, text: Vec<u8>) -> Vec<Vec<u8>> {
    let mut set_of_chars: Vec<Vec<u8>> = vec![];

    let mut stack_of_chars = vec![];
    for opcode in text {
        stack_of_chars.push(opcode);
        if encoding.encoding.contains_right(&stack_of_chars) {
            set_of_chars.push(stack_of_chars);
            stack_of_chars = vec![];
        } else {
            stack_of_chars.push(opcode);
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

#[derive(Clone)]
pub struct Encoding {
    pub str: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone)]
pub struct Alphabet {
    /// Alphabet encoding.
    pub encoding: BiBTreeMap<String, Vec<u8>>,
}

impl Alphabet {
    pub fn new(encoding: Vec<Encoding>) -> Self {
        let encoding = encoding.into_iter().map(|e| (e.str, e.bytes))
            .collect::<BiBTreeMap<_, _>>();
        Alphabet {
            encoding,
        }
    }

    pub fn new_empty() -> Self {
        Alphabet {
            encoding: BiBTreeMap::new(),
        }
    }

    /// Builder to add encoding to the encoding.
    pub fn ascii_printable_only_encoding(&mut self) -> Self {
        self.encoding = BiBTreeMap::from_iter(vec![
        (String::from(" "), vec![0x20]),
        (String::from("!"), vec![0x21]),
        (String::from("\""), vec![0x22]),
        (String::from("#"), vec![0x23]),
        (String::from("$"), vec![0x24]),
        (String::from("%"), vec![0x25]),
        (String::from("&"), vec![0x26]),
        (String::from("'"), vec![0x27]),
        (String::from("("), vec![0x28]),
        (String::from(")"), vec![0x29]),
        (String::from("*"), vec![0x2a]),
        (String::from("+"), vec![0x2b]),
        (String::from(","), vec![0x2c]),
        (String::from("-"), vec![0x2d]),
        (String::from("."), vec![0x2e]),
        (String::from("/"), vec![0x2f]),
        (String::from("0"), vec![0x30]),
        (String::from("1"), vec![0x31]),
        (String::from("2"), vec![0x32]),
        (String::from("3"), vec![0x33]),
        (String::from("4"), vec![0x34]),
        (String::from("5"), vec![0x35]),
        (String::from("6"), vec![0x36]),
        (String::from("7"), vec![0x37]),
        (String::from("8"), vec![0x38]),
        (String::from("9"), vec![0x39]),
        (String::from(":"), vec![0x3a]),
        (String::from(";"), vec![0x3b]),
        (String::from("<"), vec![0x3c]),
        (String::from("="), vec![0x3d]),
        (String::from(">"), vec![0x3e]),
        (String::from("?"), vec![0x3f]),
        (String::from("@"), vec![0x40]),
        (String::from("A"), vec![0x41]),
        (String::from("B"), vec![0x42]),
        (String::from("C"), vec![0x43]),
        (String::from("D"), vec![0x44]),
        (String::from("E"), vec![0x45]),
        (String::from("F"), vec![0x46]),
        (String::from("G"), vec![0x47]),
        (String::from("H"), vec![0x48]),
        (String::from("I"), vec![0x49]),
        (String::from("J"), vec![0x4a]),
        (String::from("K"), vec![0x4b]),
        (String::from("L"), vec![0x4c]),
        (String::from("M"), vec![0x4d]),
        (String::from("N"), vec![0x4e]),
        (String::from("O"), vec![0x4f]),
        (String::from("P"), vec![0x50]),
        (String::from("Q"), vec![0x51]),
        (String::from("R"), vec![0x52]),
        (String::from("S"), vec![0x53]),
        (String::from("T"), vec![0x54]),
        (String::from("U"), vec![0x55]),
        (String::from("V"), vec![0x56]),
        (String::from("W"), vec![0x57]),
        (String::from("X"), vec![0x58]),
        (String::from("Y"), vec![0x59]),
        (String::from("Z"), vec![0x5a]),
        (String::from("["), vec![0x5b]),
        (String::from("\\"), vec![0x5c]),
        (String::from("]"), vec![0x5d]),
        (String::from("^"), vec![0x5e]),
        (String::from("_"), vec![0x5f]),
        (String::from("`"), vec![0x60]),
        (String::from("a"), vec![0x61]),
        (String::from("b"), vec![0x62]),
        (String::from("c"), vec![0x63]),
        (String::from("d"), vec![0x64]),
        (String::from("e"), vec![0x65]),
        (String::from("f"), vec![0x66]),
        (String::from("g"), vec![0x67]),
        (String::from("h"), vec![0x68]),
        (String::from("i"), vec![0x69]),
        (String::from("j"), vec![0x6a]),
        (String::from("k"), vec![0x6b]),
        (String::from("l"), vec![0x6c]),
        (String::from("m"), vec![0x6d]),
        (String::from("n"), vec![0x6e]),
        (String::from("o"), vec![0x6f]),
        (String::from("p"), vec![0x70]),
        (String::from("q"), vec![0x71]),
        (String::from("r"), vec![0x72]),
        (String::from("s"), vec![0x73]),
        (String::from("t"), vec![0x74]),
        (String::from("u"), vec![0x75]),
        (String::from("v"), vec![0x76]),
        (String::from("w"), vec![0x77]),
        (String::from("x"), vec![0x78]),
        (String::from("y"), vec![0x79]),
        (String::from("z"), vec![0x7a]),
        (String::from("{"), vec![0x7b]),
        (String::from("|"), vec![0x7c]),
        (String::from("}"), vec![0x7d]),
        (String::from("~"), vec![0x7e])]);

        let encoding = self.encoding.clone();

        Alphabet {
            encoding: encoding
        }
    }

    pub fn ascii_encoding(&mut self) -> Self {
        let mut encoding = BiBTreeMap::new();
        encoding.insert(String::from("\x00"), vec![0x00]);
        encoding.insert(String::from("\x01"), vec![0x01]);
        encoding.insert(String::from("\x02"), vec![0x02]);
        encoding.insert(String::from("\x03"), vec![0x03]);
        encoding.insert(String::from("\x04"), vec![0x04]);
        encoding.insert(String::from("\x05"), vec![0x05]);
        encoding.insert(String::from("\x06"), vec![0x06]);
        encoding.insert(String::from("\x07"), vec![0x07]);
        encoding.insert(String::from("\x08"), vec![0x08]);
        encoding.insert(String::from("\x09"), vec![0x09]);
        encoding.insert(String::from("\x0a"), vec![0x0a]);
        encoding.insert(String::from("\x0b"), vec![0x0b]);
        encoding.insert(String::from("\x0c"), vec![0x0c]);
        encoding.insert(String::from("\x0d"), vec![0x0d]);
        encoding.insert(String::from("\x0e"), vec![0x0e]);
        encoding.insert(String::from("\x0f"), vec![0x0f]);
        encoding.insert(String::from("\x10"), vec![0x10]);
        encoding.insert(String::from("\x11"), vec![0x11]);
        encoding.insert(String::from("\x12"), vec![0x12]);
        encoding.insert(String::from("\x13"), vec![0x13]);
        encoding.insert(String::from("\x14"), vec![0x14]);
        encoding.insert(String::from("\x15"), vec![0x15]);
        encoding.insert(String::from("\x16"), vec![0x16]);
        encoding.insert(String::from("\x17"), vec![0x17]);
        encoding.insert(String::from("\x18"), vec![0x18]);
        encoding.insert(String::from("\x19"), vec![0x19]);
        encoding.insert(String::from("\x1a"), vec![0x1a]);
        encoding.insert(String::from("\x1b"), vec![0x1b]);
        encoding.insert(String::from("\x1c"), vec![0x1c]);
        encoding.insert(String::from("\x1d"), vec![0x1d]);
        encoding.insert(String::from("\x1e"), vec![0x1e]);
        encoding.insert(String::from("\x1f"), vec![0x1f]);
        encoding.insert(String::from(" "), vec![0x20]);
        encoding.insert(String::from("!"), vec![0x21]);
        encoding.insert(String::from("\""), vec![0x22]);
        encoding.insert(String::from("#"), vec![0x23]);
        encoding.insert(String::from("$"), vec![0x24]);
        encoding.insert(String::from("%"), vec![0x25]);
        encoding.insert(String::from("&"), vec![0x26]);
        encoding.insert(String::from("'"), vec![0x27]);
        encoding.insert(String::from("("), vec![0x28]);
        encoding.insert(String::from(")"), vec![0x29]);
        encoding.insert(String::from("*"), vec![0x2a]);
        encoding.insert(String::from("+"), vec![0x2b]);
        encoding.insert(String::from(","), vec![0x2c]);
        encoding.insert(String::from("-"), vec![0x2d]);
        encoding.insert(String::from("."), vec![0x2e]);
        encoding.insert(String::from("/"), vec![0x2f]);
        encoding.insert(String::from("0"), vec![0x30]);
        encoding.insert(String::from("1"), vec![0x31]);
        encoding.insert(String::from("2"), vec![0x32]);
        encoding.insert(String::from("3"), vec![0x33]);
        encoding.insert(String::from("4"), vec![0x34]);
        encoding.insert(String::from("5"), vec![0x35]);
        encoding.insert(String::from("6"), vec![0x36]);
        encoding.insert(String::from("7"), vec![0x37]);
        encoding.insert(String::from("8"), vec![0x38]);
        encoding.insert(String::from("9"), vec![0x39]);
        encoding.insert(String::from(":"), vec![0x3a]);
        encoding.insert(String::from(";"), vec![0x3b]);
        encoding.insert(String::from("<"), vec![0x3c]);
        encoding.insert(String::from("="), vec![0x3d]);
        encoding.insert(String::from(">"), vec![0x3e]);
        encoding.insert(String::from("?"), vec![0x3f]);
        encoding.insert(String::from("@"), vec![0x40]);
        encoding.insert(String::from("A"), vec![0x41]);
        encoding.insert(String::from("B"), vec![0x42]);
        encoding.insert(String::from("C"), vec![0x43]);
        encoding.insert(String::from("D"), vec![0x44]);
        encoding.insert(String::from("E"), vec![0x45]);
        encoding.insert(String::from("F"), vec![0x46]);
        encoding.insert(String::from("G"), vec![0x47]);
        encoding.insert(String::from("H"), vec![0x48]);
        encoding.insert(String::from("I"), vec![0x49]);
        encoding.insert(String::from("J"), vec![0x4a]);
        encoding.insert(String::from("K"), vec![0x4b]);
        encoding.insert(String::from("L"), vec![0x4c]);
        encoding.insert(String::from("M"), vec![0x4d]);
        encoding.insert(String::from("N"), vec![0x4e]);
        encoding.insert(String::from("O"), vec![0x4f]);
        encoding.insert(String::from("P"), vec![0x50]);
        encoding.insert(String::from("Q"), vec![0x51]);
        encoding.insert(String::from("R"), vec![0x52]);
        encoding.insert(String::from("S"), vec![0x53]);
        encoding.insert(String::from("T"), vec![0x54]);
        encoding.insert(String::from("U"), vec![0x55]);
        encoding.insert(String::from("V"), vec![0x56]);
        encoding.insert(String::from("W"), vec![0x57]);
        encoding.insert(String::from("X"), vec![0x58]);
        encoding.insert(String::from("Y"), vec![0x59]);
        encoding.insert(String::from("Z"), vec![0x5a]);
        encoding.insert(String::from("["), vec![0x5b]);
        encoding.insert(String::from("\\"), vec![0x5c]);
        encoding.insert(String::from("]"), vec![0x5d]);
        encoding.insert(String::from("^"), vec![0x5e]);
        encoding.insert(String::from("_"), vec![0x5f]);
        encoding.insert(String::from("`"), vec![0x60]);
        encoding.insert(String::from("a"), vec![0x61]);
        encoding.insert(String::from("b"), vec![0x62]);
        encoding.insert(String::from("c"), vec![0x63]);
        encoding.insert(String::from("d"), vec![0x64]);
        encoding.insert(String::from("e"), vec![0x65]);
        encoding.insert(String::from("f"), vec![0x66]);
        encoding.insert(String::from("g"), vec![0x67]);
        encoding.insert(String::from("h"), vec![0x68]);
        encoding.insert(String::from("i"), vec![0x69]);
        encoding.insert(String::from("j"), vec![0x6a]);
        encoding.insert(String::from("k"), vec![0x6b]);
        encoding.insert(String::from("l"), vec![0x6c]);
        encoding.insert(String::from("m"), vec![0x6d]);
        encoding.insert(String::from("n"), vec![0x6e]);
        encoding.insert(String::from("o"), vec![0x6f]);
        encoding.insert(String::from("p"), vec![0x70]);
        encoding.insert(String::from("q"), vec![0x71]);
        encoding.insert(String::from("r"), vec![0x72]);
        encoding.insert(String::from("s"), vec![0x73]);
        encoding.insert(String::from("t"), vec![0x74]);
        encoding.insert(String::from("u"), vec![0x75]);
        encoding.insert(String::from("v"), vec![0x76]);
        encoding.insert(String::from("w"), vec![0x77]);
        encoding.insert(String::from("x"), vec![0x78]);
        encoding.insert(String::from("y"), vec![0x79]);
        encoding.insert(String::from("z"), vec![0x7a]);
        encoding.insert(String::from("{"), vec![0x7b]);
        encoding.insert(String::from("|"), vec![0x7c]);
        encoding.insert(String::from("}"), vec![0x7d]);
        encoding.insert(String::from("~"), vec![0x7e]);
        encoding.insert(String::from("\x7f"), vec![0x7f]);
        encoding.insert(String::from("\\u128"), vec![0x80]);


        self.encoding = encoding.clone();

        Alphabet {
            encoding: encoding
        }
    }

    pub fn pokered_charset_encoding(&mut self) -> Self {
        let mut encoding = BiBTreeMap::new();
        encoding.insert(String::from("<NULL>"), vec![0x00]);
        encoding.insert(String::from("<PAGE>"), vec![0x49]);
        encoding.insert(String::from("<PKMN>"), vec![0x4a]);
    
        self.encoding = encoding.clone();

        Alphabet {
            encoding: encoding
        }
    }

    pub fn uppercase_no_space_ascii_alphabet_encoding(&mut self) -> Self {
        let mut encoding = BiBTreeMap::new();
        encoding.insert(String::from("A"), vec![0x41]);
        encoding.insert(String::from("B"), vec![0x42]);
        encoding.insert(String::from("C"), vec![0x43]);
        encoding.insert(String::from("D"), vec![0x44]);
        encoding.insert(String::from("E"), vec![0x45]);
        encoding.insert(String::from("F"), vec![0x46]);
        encoding.insert(String::from("G"), vec![0x47]);
        encoding.insert(String::from("H"), vec![0x48]);
        encoding.insert(String::from("I"), vec![0x49]);
        encoding.insert(String::from("J"), vec![0x4a]);
        encoding.insert(String::from("K"), vec![0x4b]);
        encoding.insert(String::from("L"), vec![0x4c]);
        encoding.insert(String::from("M"), vec![0x4d]);
        encoding.insert(String::from("N"), vec![0x4e]);
        encoding.insert(String::from("O"), vec![0x4f]);
        encoding.insert(String::from("P"), vec![0x50]);
        encoding.insert(String::from("Q"), vec![0x51]);
        encoding.insert(String::from("R"), vec![0x52]);
        encoding.insert(String::from("S"), vec![0x53]);
        encoding.insert(String::from("T"), vec![0x54]);
        encoding.insert(String::from("U"), vec![0x55]);
        encoding.insert(String::from("V"), vec![0x56]);
        encoding.insert(String::from("W"), vec![0x57]);
        encoding.insert(String::from("X"), vec![0x58]);
        encoding.insert(String::from("Y"), vec![0x59]);
        encoding.insert(String::from("Z"), vec![0x5a]);
    
        self.encoding = encoding.clone();

        Alphabet { 
            encoding: encoding
        }
    }

    pub fn get_encoding(&self) -> Vec<Encoding> {
        self.encoding.iter()
            .map(|(str, bytes)| Encoding { str: str.clone(), bytes: bytes.clone() })
            .collect_vec()
    }
}