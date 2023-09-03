mod cli_structure;
use cli_structure::{Cli, Commands};
use cryptatools_core::utils::alphabets::{Alphabet, Encoding};

use std::collections::HashMap;
use clap::Parser;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
//use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::Statistical;


#[serde_as]
#[derive(Deserialize, Debug)]
struct Freq (
    #[serde_as(as = "HashMap<_, Vec<DisplayFromStr>>")] 
    HashMap<String, Vec<u8>>
);

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::GetCoincidenceIndex { cipher_text: _ } => {
            println!("here is the coincidence index.")
            //let c = CoincidenceIndexGuesser::new(alphabet);
            //let coincidence_index: f64 = c.guess_coincidence_index(opcodes.as_bytes().to_vec());
        },
        Commands::FrequencyAnalysis { opcodes_cli_string, alphabet } => {
            let alphabet_object: HashMap<String, Vec<u8>> = match alphabet.clone().as_deref() {
                Some(alphabet_string_json) => {
                    let deserialize_object: Freq = serde_json::from_str(alphabet_string_json).unwrap();
                    deserialize_object.0
                },
                None => panic!("error"),
            };

            let opcodes = match opcodes_cli_string.as_deref() {
                Some(opcodes_string) => opcodes_string,
                None => panic!("cipher text opcodes not set."),
            };

            let mut opcodes_u8: Vec<u8> = vec![];
            let mut current_opcode: String = String::from("");
            for o in 0..opcodes.len() {
                if current_opcode.len() < 2 {
                    current_opcode.push_str(&String::from(opcodes.chars().nth(o).unwrap()).clone());
                }

                if current_opcode.len() == 2 {
                    let my_int = current_opcode.parse::<u8>().unwrap();
                    opcodes_u8.push(my_int);
                    current_opcode = String::from("");
                }
            }

            let mut encoding_map_object: Vec<Encoding> = vec![];
            for i in alphabet_object {
                encoding_map_object.push(Encoding {
                    str: i.0,
                    bytes: i.1,
                })
            }

            let alphabet = Alphabet::new(encoding_map_object);

            let stat = Statistical::new(alphabet);
            let result = stat.guess_statistical_distribution(opcodes_u8);

            let mut val = result.iter().collect::<Vec<_>>();
            val.sort_by_key(|value| value.0);

            for (k, v) in val {
                let mut str = String::from("");
                for opcode in k.iter() {
                    str.push_str(&String::from(opcode.to_string()));
                }
                println!("{}: {}", str, v);
            }
        },
    }
}
