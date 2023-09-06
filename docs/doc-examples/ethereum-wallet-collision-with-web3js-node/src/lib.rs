use cryptatools_core::utils::alphabets::Alphabet;
use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::hash_cryptanalysis::birthday_paradox::BirthdayParadox;

use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn birthday(block: &str) -> f64 {
    let hexadecimal_alphabet = Alphabet::hexadecimal_ascii_lowercase_sixteen_bits_alphabet();
    let bp = BirthdayParadox::new(hexadecimal_alphabet.into());
    let target_hash = block.as_bytes().to_vec();
    bp.calculate_birthday_paradox_expecting_percent_focusing_on_speed_with_taylor(target_hash.clone(), 0.50)
}