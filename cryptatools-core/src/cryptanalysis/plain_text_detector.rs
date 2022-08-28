use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub struct PlainTextDetector {
}

impl PlainTextDetector {
    pub fn new() -> Self {
        PlainTextDetector {
        }
    }

     ///  Detect if plain text if the text correspond to a set of specified know languages.
     ///
     ///  The presumed plain text is passed as argument. The `minimum_confidence_value` variable is 1 if we are sure that any word exactly correspond to the corresponding language.
     ///  The `minimum_confidence_value` is 0 if we are sure it does not correspond to the corresponding language at all.
     ///  Return true if it is plain text. Else return false.
     /// 
     ///  ```
     ///  use lingua::Language::*;
     ///  use lingua::Language;
     ///  use crate::cryptatools_core::cryptanalysis::plain_text_detector::PlainTextDetector;
     ///  let mut ptd: PlainTextDetector = PlainTextDetector::new();
     ///  let text: String = String::from("The ennemies will attack at midnight!");
     ///  let is_plain_text = ptd.is_plain_text(text, vec![], 0.0);
     ///  assert_eq!(is_plain_text, true);
     ///  ```
     pub fn is_plain_text(self, plain_or_cipher_text: String, languages: Vec<lingua::Language>, minimum_confidence_value: f64) -> bool {
        let mut languages_confidence_values = self.catch_confidence_values(plain_or_cipher_text, languages);

        let strongest_language = languages_confidence_values.unwrap().into_iter().max_by(|a, b| a.1.total_cmp(&b.1));
        if strongest_language.is_none() != true {
            let most_probably_detect_language_confidence_value: f64 = strongest_language.unwrap().1;
            if most_probably_detect_language_confidence_value >= minimum_confidence_value {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }


     ///  For each `languages` set, return a tuple with confidence value.
     ///
     ///  The confidence value is a value attributed to a text and a language.
     ///  More the text corresponds to the corresponding language, more the confidence value will be hight.
     /// 
     ///  ```
     ///  use lingua::Language::*;
     ///  use lingua::Language;
     ///  use crate::cryptatools_core::cryptanalysis::plain_text_detector::PlainTextDetector;
     ///  let mut ptd: PlainTextDetector = PlainTextDetector::new();
     ///  let text: String = String::from("The ennemies will attack at midnight!");
     ///  let is_plain_text = ptd.is_plain_text(text, vec![], 0.0);
     ///  assert_eq!(is_plain_text, true);
     ///  ```
    pub fn catch_confidence_values(self, plain_or_cipher_text: String, languages: Vec<lingua::Language>) -> Option<Vec<(Language, f64)>> {
        let detector: LanguageDetector = match languages.len() {
            0 => LanguageDetectorBuilder::from_all_languages().build(),
            _ => LanguageDetectorBuilder::from_languages(languages.as_slice()).build(),
        };

        let detected_languages: Vec<(Language, f64)> = detector.compute_language_confidence_values(plain_or_cipher_text);
    
        Some(detected_languages)
    }

     ///  Detect the language used in a plain text using the confidence value algorithm.
     /// 
     ///  
     /// 
     ///  ```
     ///  use lingua::Language::*;
     ///  use lingua::Language;
     ///  use crate::cryptatools_core::cryptanalysis::plain_text_detector::PlainTextDetector;
     ///  let mut ptd: PlainTextDetector = PlainTextDetector::new();
     ///  let text: String = String::from("languages are awesome");
     ///  let detected_language: Option<Language> = ptd.detect_language(text, vec![]);
     ///  assert_eq!(detected_language, Some(English));
     ///  ```
     /// 
     /// 
     ///  ```
     ///  use lingua::Language::*;
     ///  use lingua::Language;
     ///  use crate::cryptatools_core::cryptanalysis::plain_text_detector::PlainTextDetector;
     ///  let mut ptd: PlainTextDetector = PlainTextDetector::new();
     ///  let text: String = String::from("languages are awesome");
     ///  let detected_language: Option<Language> = ptd.detect_language(text, vec![lingua::Language::English, lingua::Language::French]);
     ///  assert_eq!(detected_language, Some(English));
     ///  ```

    pub fn detect_language(self, plain_or_cipher_text: String, languages: Vec<lingua::Language>) -> Option<Language> {
        let most_probably_detected_language = self.catch_confidence_values(plain_or_cipher_text, languages).unwrap().into_iter().max_by(|a, b| a.1.total_cmp(&b.1)).unwrap().0;
        
        Some(most_probably_detected_language)
    }


}