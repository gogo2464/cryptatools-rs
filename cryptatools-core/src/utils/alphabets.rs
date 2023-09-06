use itertools::Itertools;
use bimap::btree::BiBTreeMap;
//use std::assert_matches::assert_matches;

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

#[derive(PartialEq, Eq)]
#[derive(Debug)]
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

    pub fn intel_x86_32_encoding(&mut self) -> Self {
        let mut encoding = BiBTreeMap::new();
        encoding.insert(String::from("mov rax, rax"), vec![0x00]);
        encoding.insert(String::from("mov rax, rbx"), vec![0x01]);
    
        self.encoding = encoding.clone();

        Alphabet {
            encoding: encoding
        }
    }

    /// Unknow opcodes
    ///
    /// Use this alphabet to make statistics on stream cipher when you ignore the assembly langauge or natural language.
    pub fn unknow_opcodes(&mut self) -> Self {
        let mut encoding = BiBTreeMap::new();
        for i in 0..255 {
            let mut invalid_instruction = String::from("invalid");
            invalid_instruction.push_str(&i.to_string());
            encoding.insert(invalid_instruction, vec![i]);
        }
    
        self.encoding = encoding.clone();

        Alphabet {
            encoding: encoding
        }
    }


    /// Full Hexadecimal (0x00-0xff)
    ///
    /// Use this alphabet if you are working on hash because the hash are often hexadecimal. The alphabet is from hex string to the corresponding ascii value in hex byte value.
    ///
    /// This method has no argument.
    /// Returns an alphabet made with hexadecimal and lowercase only values.
    pub fn full_hexadecimal_alphabet() -> Self {
        let mut encoding = BiBTreeMap::new();
        encoding.insert(String::from("00"), vec![0x00]);
        encoding.insert(String::from("01"), vec![0x01]);
        encoding.insert(String::from("02"), vec![0x02]);
        encoding.insert(String::from("03"), vec![0x03]);
        encoding.insert(String::from("04"), vec![0x04]);
        encoding.insert(String::from("05"), vec![0x05]);
        encoding.insert(String::from("06"), vec![0x06]);
        encoding.insert(String::from("07"), vec![0x07]);
        encoding.insert(String::from("08"), vec![0x08]);
        encoding.insert(String::from("09"), vec![0x09]);
        encoding.insert(String::from("0a"), vec![0x0a]);
        encoding.insert(String::from("0b"), vec![0x0b]);
        encoding.insert(String::from("0c"), vec![0x0c]);
        encoding.insert(String::from("0d"), vec![0x0d]);
        encoding.insert(String::from("0e"), vec![0x0e]);
        encoding.insert(String::from("0f"), vec![0x0f]);

        encoding.insert(String::from("10"), vec![0x10]);
        encoding.insert(String::from("11"), vec![0x11]);
        encoding.insert(String::from("12"), vec![0x12]);
        encoding.insert(String::from("13"), vec![0x13]);
        encoding.insert(String::from("14"), vec![0x14]);
        encoding.insert(String::from("15"), vec![0x15]);
        encoding.insert(String::from("16"), vec![0x16]);
        encoding.insert(String::from("17"), vec![0x17]);
        encoding.insert(String::from("18"), vec![0x18]);
        encoding.insert(String::from("19"), vec![0x19]);
        encoding.insert(String::from("1a"), vec![0x1a]);
        encoding.insert(String::from("1b"), vec![0x1b]);
        encoding.insert(String::from("1c"), vec![0x1c]);
        encoding.insert(String::from("1d"), vec![0x1d]);
        encoding.insert(String::from("1e"), vec![0x1e]);
        encoding.insert(String::from("1f"), vec![0x1f]);

        encoding.insert(String::from("20"), vec![0x20]);
        encoding.insert(String::from("21"), vec![0x21]);
        encoding.insert(String::from("22"), vec![0x22]);
        encoding.insert(String::from("23"), vec![0x23]);
        encoding.insert(String::from("24"), vec![0x24]);
        encoding.insert(String::from("25"), vec![0x25]);
        encoding.insert(String::from("26"), vec![0x26]);
        encoding.insert(String::from("27"), vec![0x27]);
        encoding.insert(String::from("28"), vec![0x28]);
        encoding.insert(String::from("29"), vec![0x29]);
        encoding.insert(String::from("2a"), vec![0x2a]);
        encoding.insert(String::from("2b"), vec![0x2b]);
        encoding.insert(String::from("2c"), vec![0x2c]);
        encoding.insert(String::from("2d"), vec![0x2d]);
        encoding.insert(String::from("2e"), vec![0x2e]);
        encoding.insert(String::from("2f"), vec![0x2f]);

        encoding.insert(String::from("30"), vec![0x30]);
        encoding.insert(String::from("31"), vec![0x31]);
        encoding.insert(String::from("32"), vec![0x32]);
        encoding.insert(String::from("33"), vec![0x33]);
        encoding.insert(String::from("34"), vec![0x34]);
        encoding.insert(String::from("35"), vec![0x35]);
        encoding.insert(String::from("36"), vec![0x36]);
        encoding.insert(String::from("37"), vec![0x37]);
        encoding.insert(String::from("38"), vec![0x38]);
        encoding.insert(String::from("39"), vec![0x39]);
        encoding.insert(String::from("3a"), vec![0x3a]);
        encoding.insert(String::from("3b"), vec![0x3b]);
        encoding.insert(String::from("3c"), vec![0x3c]);
        encoding.insert(String::from("3d"), vec![0x3d]);
        encoding.insert(String::from("3e"), vec![0x3e]);
        encoding.insert(String::from("3f"), vec![0x3f]);

        encoding.insert(String::from("40"), vec![0x40]);
        encoding.insert(String::from("41"), vec![0x41]);
        encoding.insert(String::from("42"), vec![0x42]);
        encoding.insert(String::from("43"), vec![0x43]);
        encoding.insert(String::from("44"), vec![0x44]);
        encoding.insert(String::from("45"), vec![0x45]);
        encoding.insert(String::from("46"), vec![0x46]);
        encoding.insert(String::from("47"), vec![0x47]);
        encoding.insert(String::from("48"), vec![0x48]);
        encoding.insert(String::from("49"), vec![0x49]);
        encoding.insert(String::from("4a"), vec![0x4a]);
        encoding.insert(String::from("4b"), vec![0x4b]);
        encoding.insert(String::from("4c"), vec![0x4c]);
        encoding.insert(String::from("4d"), vec![0x4d]);
        encoding.insert(String::from("4e"), vec![0x4e]);
        encoding.insert(String::from("4f"), vec![0x4f]);

        encoding.insert(String::from("50"), vec![0x50]);
        encoding.insert(String::from("51"), vec![0x51]);
        encoding.insert(String::from("52"), vec![0x52]);
        encoding.insert(String::from("53"), vec![0x53]);
        encoding.insert(String::from("54"), vec![0x54]);
        encoding.insert(String::from("55"), vec![0x55]);
        encoding.insert(String::from("56"), vec![0x56]);
        encoding.insert(String::from("57"), vec![0x57]);
        encoding.insert(String::from("58"), vec![0x58]);
        encoding.insert(String::from("59"), vec![0x59]);
        encoding.insert(String::from("5a"), vec![0x5a]);
        encoding.insert(String::from("5b"), vec![0x5b]);
        encoding.insert(String::from("5c"), vec![0x5c]);
        encoding.insert(String::from("5d"), vec![0x5d]);
        encoding.insert(String::from("5e"), vec![0x5e]);
        encoding.insert(String::from("5f"), vec![0x5f]);

        encoding.insert(String::from("60"), vec![0x60]);
        encoding.insert(String::from("61"), vec![0x61]);
        encoding.insert(String::from("62"), vec![0x62]);
        encoding.insert(String::from("63"), vec![0x63]);
        encoding.insert(String::from("64"), vec![0x64]);
        encoding.insert(String::from("65"), vec![0x65]);
        encoding.insert(String::from("66"), vec![0x66]);
        encoding.insert(String::from("67"), vec![0x67]);
        encoding.insert(String::from("68"), vec![0x68]);
        encoding.insert(String::from("69"), vec![0x69]);
        encoding.insert(String::from("6a"), vec![0x6a]);
        encoding.insert(String::from("6b"), vec![0x6b]);
        encoding.insert(String::from("6c"), vec![0x6c]);
        encoding.insert(String::from("6d"), vec![0x6d]);
        encoding.insert(String::from("6e"), vec![0x6e]);
        encoding.insert(String::from("6f"), vec![0x6f]);

        encoding.insert(String::from("70"), vec![0x70]);
        encoding.insert(String::from("71"), vec![0x71]);
        encoding.insert(String::from("72"), vec![0x72]);
        encoding.insert(String::from("73"), vec![0x73]);
        encoding.insert(String::from("74"), vec![0x74]);
        encoding.insert(String::from("75"), vec![0x75]);
        encoding.insert(String::from("76"), vec![0x76]);
        encoding.insert(String::from("77"), vec![0x77]);
        encoding.insert(String::from("78"), vec![0x78]);
        encoding.insert(String::from("79"), vec![0x79]);
        encoding.insert(String::from("7a"), vec![0x7a]);
        encoding.insert(String::from("7b"), vec![0x7b]);
        encoding.insert(String::from("7c"), vec![0x7c]);
        encoding.insert(String::from("7d"), vec![0x7d]);
        encoding.insert(String::from("7e"), vec![0x7e]);
        encoding.insert(String::from("7f"), vec![0x7f]);

        encoding.insert(String::from("80"), vec![0x80]);
        encoding.insert(String::from("81"), vec![0x81]);
        encoding.insert(String::from("82"), vec![0x82]);
        encoding.insert(String::from("83"), vec![0x83]);
        encoding.insert(String::from("84"), vec![0x84]);
        encoding.insert(String::from("85"), vec![0x85]);
        encoding.insert(String::from("86"), vec![0x86]);
        encoding.insert(String::from("87"), vec![0x87]);
        encoding.insert(String::from("88"), vec![0x88]);
        encoding.insert(String::from("89"), vec![0x89]);
        encoding.insert(String::from("8a"), vec![0x8a]);
        encoding.insert(String::from("8b"), vec![0x8b]);
        encoding.insert(String::from("8c"), vec![0x8c]);
        encoding.insert(String::from("8d"), vec![0x8d]);
        encoding.insert(String::from("8e"), vec![0x8e]);
        encoding.insert(String::from("8f"), vec![0x8f]);

        encoding.insert(String::from("90"), vec![0x90]);
        encoding.insert(String::from("91"), vec![0x91]);
        encoding.insert(String::from("92"), vec![0x92]);
        encoding.insert(String::from("93"), vec![0x93]);
        encoding.insert(String::from("94"), vec![0x94]);
        encoding.insert(String::from("95"), vec![0x95]);
        encoding.insert(String::from("96"), vec![0x96]);
        encoding.insert(String::from("97"), vec![0x97]);
        encoding.insert(String::from("98"), vec![0x98]);
        encoding.insert(String::from("99"), vec![0x99]);
        encoding.insert(String::from("9a"), vec![0x9a]);
        encoding.insert(String::from("9b"), vec![0x9b]);
        encoding.insert(String::from("9c"), vec![0x9c]);
        encoding.insert(String::from("9d"), vec![0x9d]);
        encoding.insert(String::from("9e"), vec![0x9e]);
        encoding.insert(String::from("9f"), vec![0x9f]);

        encoding.insert(String::from("a0"), vec![0xa0]);
        encoding.insert(String::from("a1"), vec![0xa1]);
        encoding.insert(String::from("a2"), vec![0xa2]);
        encoding.insert(String::from("a3"), vec![0xa3]);
        encoding.insert(String::from("a4"), vec![0xa4]);
        encoding.insert(String::from("a5"), vec![0xa5]);
        encoding.insert(String::from("a6"), vec![0xa6]);
        encoding.insert(String::from("a7"), vec![0xa7]);
        encoding.insert(String::from("a8"), vec![0xa8]);
        encoding.insert(String::from("a9"), vec![0xa9]);
        encoding.insert(String::from("aa"), vec![0xaa]);
        encoding.insert(String::from("ab"), vec![0xab]);
        encoding.insert(String::from("ac"), vec![0xac]);
        encoding.insert(String::from("ad"), vec![0xad]);
        encoding.insert(String::from("ae"), vec![0xae]);
        encoding.insert(String::from("af"), vec![0xaf]);

        encoding.insert(String::from("b0"), vec![0xb0]);
        encoding.insert(String::from("b1"), vec![0xb1]);
        encoding.insert(String::from("b2"), vec![0xb2]);
        encoding.insert(String::from("b3"), vec![0xb3]);
        encoding.insert(String::from("b4"), vec![0xb4]);
        encoding.insert(String::from("b5"), vec![0xb5]);
        encoding.insert(String::from("b6"), vec![0xb6]);
        encoding.insert(String::from("b7"), vec![0xb7]);
        encoding.insert(String::from("b8"), vec![0xb8]);
        encoding.insert(String::from("b9"), vec![0xb9]);
        encoding.insert(String::from("ba"), vec![0xba]);
        encoding.insert(String::from("bb"), vec![0xbb]);
        encoding.insert(String::from("bc"), vec![0xbc]);
        encoding.insert(String::from("bd"), vec![0xbd]);
        encoding.insert(String::from("be"), vec![0xbe]);
        encoding.insert(String::from("bf"), vec![0xbf]);

        encoding.insert(String::from("c0"), vec![0xc0]);
        encoding.insert(String::from("c1"), vec![0xc1]);
        encoding.insert(String::from("c2"), vec![0xc2]);
        encoding.insert(String::from("c3"), vec![0xc3]);
        encoding.insert(String::from("c4"), vec![0xc4]);
        encoding.insert(String::from("c5"), vec![0xc5]);
        encoding.insert(String::from("c6"), vec![0xc6]);
        encoding.insert(String::from("c7"), vec![0xc7]);
        encoding.insert(String::from("c8"), vec![0xc8]);
        encoding.insert(String::from("c9"), vec![0xc9]);
        encoding.insert(String::from("ca"), vec![0xca]);
        encoding.insert(String::from("cb"), vec![0xcb]);
        encoding.insert(String::from("cc"), vec![0xcc]);
        encoding.insert(String::from("cd"), vec![0xcd]);
        encoding.insert(String::from("ce"), vec![0xce]);
        encoding.insert(String::from("cf"), vec![0xcf]);

        encoding.insert(String::from("d0"), vec![0xd0]);
        encoding.insert(String::from("d1"), vec![0xd1]);
        encoding.insert(String::from("d2"), vec![0xd2]);
        encoding.insert(String::from("d3"), vec![0xd3]);
        encoding.insert(String::from("d4"), vec![0xd4]);
        encoding.insert(String::from("d5"), vec![0xd5]);
        encoding.insert(String::from("d6"), vec![0xd6]);
        encoding.insert(String::from("d7"), vec![0xd7]);
        encoding.insert(String::from("d8"), vec![0xd8]);
        encoding.insert(String::from("d9"), vec![0xd9]);
        encoding.insert(String::from("da"), vec![0xda]);
        encoding.insert(String::from("db"), vec![0xdb]);
        encoding.insert(String::from("dc"), vec![0xdc]);
        encoding.insert(String::from("dd"), vec![0xdd]);
        encoding.insert(String::from("de"), vec![0xde]);
        encoding.insert(String::from("df"), vec![0xdf]);

        encoding.insert(String::from("e0"), vec![0xe0]);
        encoding.insert(String::from("e1"), vec![0xe1]);
        encoding.insert(String::from("e2"), vec![0xe2]);
        encoding.insert(String::from("e3"), vec![0xe3]);
        encoding.insert(String::from("e4"), vec![0xe4]);
        encoding.insert(String::from("e5"), vec![0xe5]);
        encoding.insert(String::from("e6"), vec![0xe6]);
        encoding.insert(String::from("e7"), vec![0xe7]);
        encoding.insert(String::from("e8"), vec![0xe8]);
        encoding.insert(String::from("e9"), vec![0xe9]);
        encoding.insert(String::from("ea"), vec![0xea]);
        encoding.insert(String::from("eb"), vec![0xeb]);
        encoding.insert(String::from("ec"), vec![0xec]);
        encoding.insert(String::from("ed"), vec![0xed]);
        encoding.insert(String::from("ee"), vec![0xee]);
        encoding.insert(String::from("ef"), vec![0xef]);

        encoding.insert(String::from("f0"), vec![0xf0]);
        encoding.insert(String::from("f1"), vec![0xf1]);
        encoding.insert(String::from("f2"), vec![0xf2]);
        encoding.insert(String::from("f3"), vec![0xf3]);
        encoding.insert(String::from("f4"), vec![0xf4]);
        encoding.insert(String::from("f5"), vec![0xf5]);
        encoding.insert(String::from("f6"), vec![0xf6]);
        encoding.insert(String::from("f7"), vec![0xf7]);
        encoding.insert(String::from("f8"), vec![0xf8]);
        encoding.insert(String::from("f9"), vec![0xf9]);
        encoding.insert(String::from("fa"), vec![0xfa]);
        encoding.insert(String::from("fb"), vec![0xfb]);
        encoding.insert(String::from("fc"), vec![0xfc]);
        encoding.insert(String::from("fd"), vec![0xfd]);
        encoding.insert(String::from("fe"), vec![0xfe]);
        encoding.insert(String::from("ff"), vec![0xff]);

        Alphabet {
            encoding: encoding
        }
    }



    /// Ascii lowercase hexadecimal alphabet
    ///
    /// Use this alphabet if you are working on hash because the hash are often hexadecimal. The alphabet is from hex string to the corresponding ascii value in hex byte value.
    ///
    /// This method has no argument.
    /// Returns an alphabet made with hexadecimal and lowercase only values.
    pub fn hexadecimal_ascii_lowercase_sixteen_bits_alphabet() -> Self {
        let mut encoding = BiBTreeMap::new();
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
        encoding.insert(String::from("a"), vec![0x61]);
        encoding.insert(String::from("b"), vec![0x62]);
        encoding.insert(String::from("c"), vec![0x63]);
        encoding.insert(String::from("d"), vec![0x64]);
        encoding.insert(String::from("e"), vec![0x65]);
        encoding.insert(String::from("f"), vec![0x66]);

        //self.encoding = encoding.clone();

        Alphabet {
            encoding: encoding
        }
    }


    /// Extended ascii alphabet.
    ///
    /// Contains 256 values of ascii character.
    /// This is the default alphabet that you should use when working on networking packets, program file or assembly language.
    ///
    /// This method has no argument.
    /// Returns an alphabet made with extended ascii values..
    pub fn extended_ascii_encoding(&mut self) -> Self {
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
        encoding.insert(String::from("€"), vec![0x80]);//\\u128


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

    /// Convert opcodes to an human readable set of characters text in the same order as the original unconverted text.
    ///
    /// Once deciphered in plain characters, a cryptographic algorithm still need to be seen in a decodable format. This is why this method returns a Vector of `Encoded` struct.
    ///
    /// paramaters:
    ///   - encoding: opcode vector to convert to a set of character.
    ///
    /// Returns: a converted human readable set of character text in the same order as the original unconverted text.
    ///
    /// ```
    /// #![feature(assert_matches)]
    /// 
    /// use std::assert_matches::assert_matches;
    /// use cryptatools_core::utils::{convert::Encode, alphabets::split_bytes_by_characters_representation, alphabets::Alphabet};
    /// use cryptatools_core::utils::alphabets::Encoding;
    /// 
    ///
    /// let ascii_alphabet = Alphabet::new_empty().ascii_encoding();
    /// let alph = ascii_alphabet.decode(vec![0x41, 0x41, 0x42, 0x42]);
    /// let decoded = vec![Encoding{str: String::from("A"), bytes: vec![0x41]}, Encoding{str: String::from("A"), bytes: vec![0x41]}, Encoding{str: String::from("B"), bytes: vec![0x42]}, Encoding{str: String::from("B"), bytes: vec![0x42]}];
    /// assert_eq!(alph, decoded);
    /// ```
    pub fn decode(&self, encoded: Vec<u8>) -> Vec<Encoding> {
        let mut alway_iterate = 0;
        let mut iterate_foreach_character_decoded = 0;
        let mut out: Vec<Encoding> = vec![];
        while iterate_foreach_character_decoded < encoded.len() {
            let part: Vec<u8> = encoded[iterate_foreach_character_decoded..alway_iterate].to_vec();
            match self.encoding.contains_right(&part) {
                true => {
                    let decoded = self.encoding.get_by_right(&part);
                    out.push(Encoding{str: String::from(decoded.unwrap()),
                        bytes: part});
                    iterate_foreach_character_decoded += 1;
                },
                false => {

                }
            }
            alway_iterate += 1;
        }

        out
    }
}

/*
let decoded = encoded[j..i].into_iter().map(|e| (e.str, e.bytes))
            .collect::<BiBTreeMap<_, _>>();
*/