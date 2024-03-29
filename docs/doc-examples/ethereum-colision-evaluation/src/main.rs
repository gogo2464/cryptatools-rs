use ethers::prelude::*;

use cryptatools_core::utils::alphabets::Alphabet;
use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::hash_cryptanalysis::birthday_paradox::BirthdayParadox;

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let hexadecimal_alphabet = Alphabet::full_hexadecimal_alphabet();
    let bp = BirthdayParadox::new(hexadecimal_alphabet.into());
    
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let mut stream = provider.subscribe_blocks().await?.take(1);
    let mut wallet_block: Option<Vec<u8>> = None;
    while let Some(block) = stream.next().await {
        if let Some(author) = block.author {
            wallet_block = Some(author.0.to_vec());
        }
    }

    if let Some(wallet_block) = wallet_block {
        let target_hash = wallet_block;
        println!("After {:?} attempts, there is 50% of chances to get a collision on ethereum addresses.", bp.calculate_birthday_paradox_expecting_percent_focusing_on_speed_with_taylor(target_hash.clone(), 0.50));
    } else {
        println!("Error: wallet not found. Check your internet connection.");
    }

    Ok(())
}