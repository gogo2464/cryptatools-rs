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