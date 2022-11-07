<!-- GETTING STARTED -->
## Getting Started

This is a cryptanalysis tool for exploit developers and ctf players.

The name come from pwntools a similar tool to exploit memory corruption vulnerabilities. This software aims to work like pwntools but for cryptanalysis.

Then this program include a library like pwnlib. And it will expose some command line tools. Like pwntools. This is a rewrite of the python version in rust in order to be able to be fast and portable and usable in the following various languages rust, python, ruby, javascript and kotlin together.

### Philosophy

This tool aims to be professionnal. Not only a learning tool. It is for realistic exploitation and code breaking.

You can "plug-in" your script to any protocol. Man in the midle as well as blockchain core as well as anything. Example:

  -You are able to use `pypcap` python library to read packets and then `dpkt` python library to parse these and then you can use `cryptatools` to break encryption on these packets. This is why this library is avaible in many bindings such as python.

  -You are able to use rust-web3 to parse a vulnerable cryptocurrency (shitcoin) address to lead to a double spend attack.

The library is very very flexible.

You can automatize any task. There is a command line interface.

### Installation

### 1-Rust library installation.
Installation of `cryptatools-core` for rust is same for any OS. In `Cargo.toml`, just write:
```shell
[dependencies]
cryptatools-core = { git = "https://github.com/gogo2464/cryptatools-rs", package = 'cryptatools-core' }
```

### 2-Python binding installation.
To install the python Bindings,

If you are on windows, with powershell do:
```powershell
python -m venv myenv
.\myenv\Script\activate
cargo install uniffi_bindgen
pip install setuptools wheel
git clone https://github.com/gogo2464/cryptatools-rs ;
cd cryptatools-rs
python .\cryptatools-core\setup.py bdist_wheel --verbose ;
$wheelFile = Get-ChildItem -Path .\dist\ -Recurse -Include * ;
pip3 install $wheelFile --force-reinstall ;
```

If you are on Linux, do:
```shell
virtualenv -p python3 myenv
source myenv/bin/activate
cargo install uniffi_bindgen
pip install setuptools wheel
git clone https://github.com/gogo2464/cryptatools-rs ;
cd cryptatools-rs
python3 ./cryptatools-core/setup.py bdist_wheel --verbose ;
pip3 install ./dist/* --force-reinstall ;
```

If you are on MacOs, do:
```shell
virtualenv -p python3 myenv
source myenv/bin/activate
cargo install uniffi_bindgen
pip install setuptools wheel
git clone https://github.com/gogo2464/cryptatools-rs ;
cd cryptatools-rs
python3 ./cryptatools-core/setup.py bdist_wheel --verbose ;
pip3 install ./dist/* --force-reinstall ;
```

### 3-cryptatools-cli the cli intreface

Crytptatools command line interface is split into various program in order to follow the Linux philosophy. To install each one, do:

```shell
git clone https://github.com/gogo2464/cryptatools-rs/ &&
cargo install --path .\cryptatools-rs\cryptatools-cli\ ;
```

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```rust
```

crypytatools-cli also offers a way to script the library in command line. Do not forget to install each program. See how to install these [here](https://github.com/gogo2464/cryptatools#installation) for more informations.

`cryptatools-cli-stats` does cryptanalysis statistical attacks.

```shell
cryptatools-cli-stats frequency-analysis "123234" '{\"a\": [\"12\"], \"b\": [\"32\"], \"c\": [\"34\"]}'
```

`cryptatools-cli-encrypt` uses cryptography algorithm to encrypt data. Obviously you can use it for brute force cryptanalysis attack. But it is not the main philosophy of `cryptatools-rs`.

```shell
```

For more examples, please refer to the [Tutorial](https://github.com/gogo2464/cryptatools-rs/blob/master/TUTORIAL.md) or to the documentation [Documentation](https://gogo2464.github.io/cryptatools-rs/cryptatools_core/).

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- TESTING EXAMPLES -->
## Testing

In order to run unit tests, you MUST be in the directory cryptatools-rs.
Run unit tests with doctests with the command:

```shell
cargo test --all
```

Unit test are made with doctests.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- DOCUMENTATION EXAMPLE -->
## Documentation

The documentation is generated by doctstring with rustdoc. To edit the documentation, go to the code and modify the doctstring after `///`.

Then, in order to generate documentation from root folder do:

```shell
cargo doc
```

If you are a developper, fell free to view your own doc with

```
cargo doc --open
```

This project uses `cargo doc`.

The documentation is auto-generated at each pull request.

To see the cryptatools documentation. See: [the API documentation](https://gogo2464.github.io/cryptatools-rs/cryptatools_core/)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- BENCHMARKING EXAMPLE -->
## Benchmark

```shell
cargo bench
```

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Document your work. See [the how to make documentation chapter](https://github.com/gogo2464/cryptatools#documentation)
2. For each method or object implemented, do not forget to make tests with doctest.
3. Create Python binding.
4. Open your Pull Request.

## Create Python bindings.

Cryptatools relies on uniffi to provide bindings to Python3. Ensure to provide Python3 bindings before making your Pull Request.

To create Python Bindings, edit the file `cryptatools-rs/src/cryptatools.udl`. Edit it to add your newly created object as mentionned in the official uniffi documentation at this address: https://mozilla.github.io/uniffi-rs/udl_file_spec.html.

Then do not forget to edit the file `cryptatools-rs/cryptatools-core/src/lib.rs` and just before the:

```
uniffi_macros::include_scaffolding!("cryptatools");
```

import your own crate.

Once this is done, edit the task `Test python bindings` on the pipeline of the file `cryptatools-rs/.github/workflows/windows.yml`. Fell free to import and use your work here.


<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->
## License

This software has a free software (libre) license. The license is GPL3. 

Fell free to ask me if you absolutely want to get a double licensing name. Then I could choose.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

gogo - gogo246475@gmail.com

Project Link: [https://github.com/gogo2464/cryptatools-rs](https://github.com/gogo2464/cryptatools-rs)

<p align="right">(<a href="#top">back to top</a>)</p>