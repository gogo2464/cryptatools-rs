# 1 Intro

In order to connect with smart contracts, you might need to code in nodejs using cryptatools-rs bindings in order to interact with hardhat/web3js.

The full code example is avaible here: [Click Here](https://github.com/gogo2464/cryptatools-rs/tree/master/docs/doc-examples/ethereum-wallet-collision-with-web3js-node).

# 2 Create bindings

Sadly uniffi-rs does not currently implement javascript bindings, then we have to code these on our own using `wasm-bindgen` rust assembly tool. See this [link](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html).

./src/lib.rs
```rust
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
```

# 3 Call bindings

Then we have to call bindings.

index.js
```javascript

import { birthday } from './pkg';

let attempts = birthday('71C7656EC7ab88b098defB751B7401B5f6d8976F');

console.log(attempts);
```

package.json
```json
{
  "scripts": {
    "build": "webpack",
    "serve": "webpack serve"
  },
  "devDependencies": {
    "@wasm-tool/wasm-pack-plugin": "1.5.0",
    "html-webpack-plugin": "^5.3.2",
    "text-encoding": "^0.7.0",
    "webpack": "^5.88.2",
    "webpack-cli": "^4.7.2",
    "webpack-dev-server": "^4.15.1"
  },
  "dependencies": {
    "express": "^4.18.2"
  }
}
```

# 4 Configuration Files

Our code above can not work if we do not include the configuration files. Read:

webpack.config.js

```json
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
   }
};
```