use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Calculate Coincidence Index of a cipher text
    GetCoincidenceIndex { cipher_text: Option<String> },
    /// Encrypt a plain text
    FrequencyAnalysis { opcodes_cli_string: Option<String>, alphabet: Option<String> },
}