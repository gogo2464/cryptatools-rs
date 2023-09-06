## Evaluating Ethereum address collision probabilities with birthday paradox method using ether-rs and cryptatools-rs

# I - Intro And Challenges

# 1. Intro

Ethereum is a decentralized cryptocurrency. Each people has his own wallet authenticated with a public/private keyset. As ethereum is decentralized, if someone create the adress of somebody else, then he will be able to spoof his identity. Hopefully addresses are random, then we need to create a lot of identities.

But how many identities do we need to create? A paper at this address [here](https://download.wpsoftware.net/bitcoin-birthday.pdf) has already answered to this question.

Today we are going to implement a cryptanalys attack with `cryptatools` to automatically know the attempts required instead of calculating manually.

# 2. Challenges And Terms

In the origin, the Birtday Paradox Attack is an attack where the attackant wants to check if the has output is tall enough hash output in order to find collision using pure brute force.
It is effective in web2 hashing algorithm in order to know if collision are possible on hash algorithm.
This attack supposes that the hash output generation is purely perfectly random. Tough this methd seems very very brute, the birtday paradox uses math probabilities in order to evaluate if a hash outpute is long enough.

Today we are going to implement a birtday paradox evalution on bitcoin cryptocurrency using cryptatools-rs, we are able to use birtday paradox, not in order to run hash attack but to evalutate how many time we will need to try in order to have 50% of chances to reach a collision between two wallets.

# II let's do it

# 1. Fecthing A Random Author Adress

We need to fetch a random hash in order to check if the hash is strong enough. We will use the library `ether-rs`:

```rust
use ethers::prelude::*;
```

then let's use it in order to really fetch the block:

```rust
use ethers::prelude::*;

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";

#[tokio::main]
async fn main() -> eyre::Result<()> {    
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let mut stream = provider.subscribe_blocks().await?.take(1);
    let mut wallet_block: Option<Vec<u8>> = None;
    while let Some(block) = stream.next().await {
        if let Some(author) = block.author {
            wallet_block = Some(author.0.to_vec());

            println!("random hash author: {:?}", author);
        }
    }

    Ok(())
}
```

# 2. Configuring cryptatools-rs


In `cryptatools-rs`, the alphabet is a concept that applies not only for letters but also to any format. We need to tells cryptatools-rs to use hexadecimal alphabet because it will calculate the length in bits of the alphabet in order to occur the birthday paradox with the right number of bits.

We need to select the alphabet. We will use `full_hexadecimal_alphabet` struct because it is a struct that contains 255 values.

```rust
let hexadecimal_alphabet = Alphabet::new_empty().full_hexadecimal_alphabet();
let mut bp = BirtdayParadox::new(hexadecimal_alphabet.into());
```

Now we will need to call the method `calculate_birtday_paradox_expecting_percent_focusing_on_speed_with_taylor`.

This is a method that is speed enough to be calculated on more than let's say 10 characters hash. This is sadly less precise than the method  `calculate_birtday_paradox_expecting_percent_focusing_on_precision` that we will not user here.

Let's write:

```rust
bp.calculate_birtday_paradox_expecting_percent_focusing_on_speed_with_taylor(target_hash.clone(), 0.50)
```

in

```rust
use ethers::prelude::*;

use cryptatools_core::utils::{convert::Encode, alphabets::split_bytes_by_characters_representation, alphabets::Alphabet, alphabets::Encoding};
use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::coincidence_index::CoincidenceIndexGuesser;
use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::hash_cryptanalysis::birthday_paradox::BirtdayParadox;

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let hexadecimal_alphabet = Alphabet::new_empty().full_hexadecimal_alphabet();
    let bp = BirtdayParadox::new(hexadecimal_alphabet.into());
    
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
        println!("after {:?} attempts, there is 50% of chances to get a collision on ethereum addresses.", bp.calculate_birtday_paradox_expecting_percent_focusing_on_speed_with_taylor(target_hash.clone(), 0.50));
    } else {
        println!("Error: wallet not found. Check your internet connection.");
    }

    Ok(())
}
```

# 4. Running a Birtday paradox evaluation.

In the paper they mentionned:
```
For e = 50%, this gives n = 1.41 × 10^24.
```

Let's run...
```
After 1.4234013764919992e24 attempts, there is 50% of chances to get a collision on ethereum addresses.
```

Sounds perfect! It is exactly the same answer than the paper!

Of course there is a difference of 0.01e24 but the birthday paradox is an approximation.

# Conclusion

ethereum wallet hash and alphabet length provide same resistance as the bitcoin.

We do not have to worry about the probability to find a wallet collision. It is very safe!

The source code of this course is [here](https://github.com/gogo2464/cryptatools-rs/tree/master/docs/doc-examples/ethereum-colision-evaluation).