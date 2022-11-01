mod cli_structure;
use core::panic;

use clap::Parser;
use cli_structure::{Cli, Commands};

fn main() {
    let caesar_algo_name = String::from("caesar");
    let cli = Cli::parse();
    match &cli.command {
        Commands::GetCoincidenceIndex { cipher_text } => {
            print!("here is the coincidence index.")
        },
        Commands::Encrypt {encryption_cipher, plain_opcodes, alphabet } => { 
            match encryption_cipher.clone().as_deref() {
                Some("caesar") => println!("Encrypting with: {:}.", caesar_algo_name),
                None => println!("Encryption algotithm not set."),   
                _ => println!("Unknow encryption algorithm."),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}