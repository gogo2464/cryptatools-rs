# Cryptatools-rs Philosophy

`cryptatools-rs` is a cryptanalysis tool against custom or wrongly implemented algorithm.

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