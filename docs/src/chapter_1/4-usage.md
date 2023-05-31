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