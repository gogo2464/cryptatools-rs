//use rust_bert::pipelines::translation::TranslationModelBuilder;
//use rust_bert::pipelines::translation::Language;
//use rust_bert::pipelines::pos_tagging::POSModel::*;

pub struct PlainTextDetector {
    language: String
}

impl PlainTextDetector {
    pub fn new(language: String) -> Self {
        PlainTextDetector {
            language
        }
    }

     ///  Detect plain text if the text correspond to a set of know language words.
     ///
     ///  The presumed plain text is passed as argument. Each character in the plain text is shifted of `key` ranges in the alphabet.
     ///  If the alphabet overflows, then the cipher text continues from the start of the alphabet.
     ///  The custom alphabet has been put in the constructor of the struct CaesarNumberAlgorithm.
     /// 
     ///  ```
     ///  use crate::cryptatools_core::cryptanalysis::plain_text_detector::PlainTextDetector;
     ///  use crate::cryptatools_core::cryptography::encoding::alphabets::ASCII_ALPHABET;
     ///  let mut ptd: PlainTextDetector = PlainTextDetector::new(String::from(ASCII_ALPHABET));
     ///  let text: String = String::from("My name is Bob");
     ///  let character_integer_vector: Vec<u8> = vec![];
     ///  ptd.detect_plain_text(text, 0.25);
     ///  ```
    pub fn detect_plain_text(self, plain_or_cipher_text: String, similarity_level: f32) {
        //let pos_model = POSModel::new(default::default())?;
        //let output = pos_model.predict(&plain_or_cipher_text.as_slice());

        //println!("{0}", output)
    }
}