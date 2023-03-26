<!-- GETTING STARTED -->
## Getting Started

This is a cryptanalysis tool for cybersecurity researcher, exploit developers and ctf players.

The name come from pwntools a similar tool to exploit memory corruption vulnerabilities. This software aims to work like pwntools but for cryptanalysis.

Then this program include a library like pwnlib. And it will expose some command line tools. Like pwntools. This is a rewrite of the python version in rust in order to be able to be fast and portable and usable in the following various languages rust, python, ruby, javascript and kotlin together.

### Philosophy

This tool aims to be professionnal. Not only a learning tool. It is for realistic exploitation and code breaking.

You can "plug-in" your script to any protocol. Man in the midle as well as blockchain core as well as anything. Example:
  - You are able to use `pypcap` python library to read packets and then `dpkt` python library to parse these and then you can use `cryptatools` to break encryption on these packets. This is why this library is avaible in many bindings such as python.
  - You are able to use `rust-web3` to parse a vulnerable cryptocurrency (shitcoin) blockchain hash tree to steal money using collision attack to forge a signature. See [this reference](https://github.com/mit-dci/tangled-curl/blob/master/vuln-iota.md#steal-money-attack).

You can also work on programs obfuscated by encryption such as malware. In this case, you can decipher program data (eg: data contained in a dropper) as well as self-encrypyted code. In this way you can plug cryptatools with your favorite reverse engineering framwork. Eg:
  - Install radare2. Then do `radare2 -AA -i <yourscriptname>.rs <yourmalwaretoreverse>`. If you work with python bindings, `radare2 -AA -i <yourscriptname>.py <yourmalwaretoreverse>`
  - You can also work on extracted code from the malware:
```shell
radare2 <malwaretoreverse>; 
=> s sym.encrypted;
pr 12345 > ciphertext.bin
```

Where 12345 is the size of the encrypted function or code of the malware.

The library is very very flexible. One of the main concept is to break custom cryptography. That is why you can meet classic cryptanalysis in `cryptatools`. This flexibility also aim to break obfuscation/encrypted malware. These are often written in assembly language because they deal with the system and then need to reimplement a lot of things and so their encryption method are often poorly written.

You can automatize any task. There is a command line interface.

### Installation

### 1-Rust library installation.
Installation of `cryptatools-core` for rust is same for any OS. In `Cargo.toml`, just write:
```shell
[dependencies]
cryptatools-core = { git = "https://github.com/gogo2464/cryptatools-rs", package = 'cryptatools-core' }
```

Works on rust stable, unstable as well as nightly toolchains.

### 2-Python binding installation.
To install the python Bindings you can use pip or build from source:

### 2.1-Install python Bindings from pip:

The name `cryptatools-python3` is the name of the package used to install cryptatools core python bindings. In order to install it, do:

```
pip install cryptatools-python3
```

It is updated of 1 subversion on each Pull Request and is then often update by the previous version.

### 2.2-Build Python Bindings from sources

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

`cryptatools-cli-stats` does cryptanalysis statistical attacks. It takes as first argument the statistical attack algorithm (example coincidence index, frequency analysis, etc...), as second argument the source encrypted opcodes to make attack on, at last but not least, it takes a json value of the corresponding alphabet corresponding to each set of opcodes.

On windows:
```powershell
cryptatools-cli-stats frequency-analysis "123234" '{\"a\": [\"12\"], \"b\": [\"32\"], \"c\": [\"34\"]}'
```

On Linux:
```
cryptatools-cli-stats frequency-analysis "123234" '{"a": ["12"], "b": ["32"], "c": ["34"]}'
```

`cryptatools-cli-encrypt` uses cryptography algorithm to encrypt data. Obviously you can use it for brute force cryptanalysis attack. But it is not the main philosophy of `cryptatools-rs`.

```shell
```

For more examples, please refer to the [Tutorial](https://github.com/gogo2464/cryptatools-rs/blob/master/TUTORIAL.md) or to the documentation [Documentation](https://gogo2464.github.io/cryptatools-rs/cryptatools_core/).

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- DOCUMENTATION EXAMPLE -->
## Documentation

`cryptatools-core` is a well documented project. 

In order to see the cryptatools documentation. See: [the API documentation](https://gogo2464.github.io/cryptatools-rs/cryptatools_core/)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Document your work. See [the how to make good documentation](https://github.com/gogo2464/cryptatools-rs#1-documentation) chapter.
2. For each method or object implemented, do not forget to make tests with doctest. See [here](https://github.com/gogo2464/cryptatools-rs#2-testing).
3. Create Python binding. See [this link](https://github.com/gogo2464/cryptatools-rs#3-create-python-bindings).
4. Benchmark your work. See [this link](https://github.com/gogo2464/cryptatools-rs#4-benchmarking).
5. Create cli interface. See [This link](https://github.com/gogo2464/cryptatools-rs#5-create-cli).
6. Open PR.

<!-- DOCUMENTATION EXAMPLE -->
## 1-Documentation

The documentation is generated by doctstring with rustdoc. To edit the documentation, go to the code and modify the doctstring after `///`.

Then, in order to generate documentation from root folder do:

```shell
cargo doc --all --no-deps
```

Fell free to view your own doc with

```
cargo doc --open --all --no-deps
```

This project uses `cargo doc`.

The documentation is self-generated on each pull request.

To check if the cryptatools documentation has been updated after a merge request, see: [the API documentation](https://gogo2464.github.io/cryptatools-rs/cryptatools_core/).

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- TESTING EXAMPLES -->
## 2-Testing

In order to run unit tests, you MUST be in the directory cryptatools-rs.
Run unit tests with doctests with the command:

```shell
cargo test --all
```

Unit test are made with doctests.

<p align="right">(<a href="#top">back to top</a>)</p>


## 3-Create Python bindings.

Cryptatools relies on uniffi to provide bindings to Python3. Ensure to provide Python3 bindings before making your Pull Request.

To create Python Bindings, edit the file `cryptatools-rs/src/cryptatools.udl`. Edit it to add your newly created object as mentionned in the official uniffi documentation at this address: https://mozilla.github.io/uniffi-rs/udl_file_spec.html.

Then do not forget to edit the file `cryptatools-rs/cryptatools-core/src/lib.rs` and just before the:

```
uniffi_macros::include_scaffolding!("cryptatools");
```

This step will generate a single python file that you could import directly. Sadly the good pratice you must do to import these is a bit more complicated.

`cryptatools-rs\cryptatools-core\setup.py`

In order to import your own crate, create the corresponding python file or folders under `cryptatools-rs\cryptatools-core\bindings\python3\cryptatools-core\cryptanalysis\`. Here you need to import the necessary objects from from `cryptatools_core.python3_bindings`. Example, in the file `cryptatools-rs\cryptatools-core\bindings\python3\cryptatools-core\cryptography\encryption\monoalphabetic_cipher\caesar_number.py` we just have written: `from cryptatools_core.python3_bindings import CaesarNumberAlgorithm`.

Once this is done, fell free to write unit tests. At least one for each method implemented. The tests are writtenh in the file `cryptatools-rs\cryptatools-core\binding-testing\testing.py`.

You are now free to test and compile your code with [the documentation at this link](https://github.com/gogo2464/cryptatools-rs#2-python-binding-installation).

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- BENCHMARKING EXAMPLE -->
## 4-Benchmarking

Sometimes, a function could sepnd too much time. In this case, you can debug your specific function from the template in the file `benches\cryptatools_benchmark\main.rs`.

Then test your function with:

```shell
cargo bench
```

In the current state of cryptatools, it is not mandatory from a pull request to benchmark all the code. But could be considered as a good improvment.

<p align="right">(<a href="#top">back to top</a>)</p>


## 5-Create cli.

Each feature from cryptatools-core will be implemented to the cli in the folder `cryptatools-cli`. When you will have finish to implement your feature in incryptatools-core and when you have finish your feature in Python bindings, fell free to create the cli of your modifications.

Then test it with:

```shell
cargo install --path .\cryptatools-cli\ ;
```

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


<!-- COMMUNITY -->
## Community

We currently use discord as official cryptatool-rs community meeting. We may add an elements server.

Discord - https://discord.gg/6YY7HFNA

<p align="right">(<a href="#top">back to top</a>)</p>
