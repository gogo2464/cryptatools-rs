## Use cryptanalysis attack method against a caesar encryption shellcode for Malware analysis with cryptatools and radare2 rust bindings (r2pipe)

### 1-Introduction

Knowledge required:
- cryptanalysis on caesar cipher
- [x86_32 assembly language programming. Check your language here.](https://pacman128.github.io/pcasm/)

Shellstorm is a pentester ressource. They provide shellcode for cyber security researcher. Sadly their shellcode could be used by hackers in order to create computer trojan. Hackers could include the shellcode in an image or an executable to add trojan during the execution of the file.

One of these shellcode uses caesar to obfuscate his signature against anti-virus.

Today we are going to use cryptatools to break the caesar encryption algorithm to deobfuscate a malware for reverse engineering purpose. We will explore the different way than brute force to break this caesar encryption and we will break ceasar at least.

### 2-Reverse Engineering the shellcode

We want to see the machine code of the shellcode in order to see were the data is stocked and to do reverse engineering in order to ensure the encryption algorithm.

In the case of this analysis, we have access to the [Shellstorm shellcode sample link here](http://shell-storm.org/shellcode/files/shellcode-900.html). So we already have the opcode of the shellcode in the format `\x12\x34`. So let's copy the opcodes `\xeb\x25\x5e\x31\xc9\xb1\x1e\x80\x3e\x07\x7c\x05\x80\x2e\x07\xeb\x11\x31\xdb\x31\xd2\xb3\x07\xb2\xff\x66\x42\x2a\x1e\x66\x29\xda\x88\x16\x46\xe2\xe2\xeb\x05\xe8\xd6\xff\xff\xff\x38\xc7\x57\x6f\x69\x68\x7a\x6f\x6f\x69\x70\x75\x36\x6f\x36\x36\x36\x36\x90\xea\x57\x90\xe9\x5a\x90\xe8\xb7\x12\xd4\x87` to the file `shellcode.txt`.

Then we are now going to compile the shellcode in order to disassemble it.

First parse it to get an opcode format such as format `1234`:

```shell
echo "$(cat shellcode.txt | tr -d 'x' | tr -d '\\' | tr -d '\n')" > opcode.txt
```

Then let's compile it really:

```shell
xxd -r -p opcode.txt bin
```

The file bin is created. It is the assembled shellcode. We could as well run it with:

```shell
chmod u+x ./bin
./bin
```

But our goal is to break it. Not to let it infect us. So let's disassemble it with radare2:

```shell
radare2 -a x86 -b 32 ./bin
```

radare2 is a reverse engineering framwork. We could use it to read the shellcode machine code. The option `-a x86` tells radare2 to disassemble an x86 processor machine code. The option `-b 32` tells radare2 to read 32 bit code. We could use `-AA` to provide as much information as possible about the binary. We prefer manual documentation in this tutorial.

Now in the radare2 console, type `Vp` to switch to the disassembled machine code, then you can read the code of the shellcode.

```assembly
jmp 0x27
pop esi
xor ecx, ecx
mov cl, 0x1e
cmp byte [esi], 7
jl 0x11
sub byte [esi], 7
jmp 0x22
xor ebx, ebx
xor edx, edx
mov bl, 7
mov dl, 0xff
inc dx
sub bl, byte [esi]
sub dx, bx
mov byte [esi], dl
inc esi
loop 7
jmp 0x2c
call 0x02
cmp bh, al
push edi
outsd dx, dword [esi]
imul ebp, dword [eax + 0x7a], 0x70696f6f
jne 0x6f
outsd dx, dword [esi]
nop
ljmp 0xe890
mov bh, 0x12
aam 0x87
```

Let's add some labels to document it:

```assembly
_start:
    jmp call_decoder
decoder:
    pop esi
    xor ecx, ecx
    mov cl, 0x1e
decode:
    cmp byte [esi], 7
    jl lowbound
    sub byte [esi], 7
    jmp 0x22
lowbound:
    xor ebx, ebx
    xor edx, edx
    mov bl, 7
    mov dl, 0xff
    inc dx
    sub bl, byte [esi]
    sub dx, bx
    mov byte [esi], dl
common_command:
    inc esi
    loop 7
    jmp shellcode
call_decoder:
    call decoder
shellcode:
    cmp bh, al
    push edi
    outsd dx, dword [esi]
    imul ebp, dword [eax + 0x7a], 0x70696f6f
    jne 0x6f
    outsd dx, dword [esi]
    nop
    ljmp 0xe890
    mov bh, 0x12
    aam 0x87
```

Let's read the assembly code of the shellcode!

The instruction `jmp call_decoder` go to the label call_decoder. Then the instruction `call decoder` is called. In order to really understsand the shellcode we have to dig deeply on what the instruction `call decoder` does. Even if you know assembly language, do not miss this step.

In x86 assembly language, the instruction call does not only jump to the label. It pushes the next instruction address to the stack and then, when the instruction left, uses a `pop eip` instruction to come back to the next instruction.

In shellcoding, it is a very intresting feature because, in this shellcode, the address of the label `shellcode` is pushed to the stack.

```assembly
decoder:
    pop esi
```

As first instruction of the called function we could see a very tricky and interesting property of the assembly language. The shellcode address is poped on the esi!!!

`cl` is the register that stores the size of the encrypted content. It is then `0x1e`.

Let's read the encrypted content:

```shell
p8 0x1e @ 0x2c ;
38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487
```

`38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487` is the cipher text. 

Now that we know where the shellcode is (at the label shellcode), we will break it using in a first time only cryptatools cryptanalysis method. Then we will explore each way to solve the problem. We will start by the most cryptanalystic method and finish with the less cryptanalystic method.


### 3.1 Caesar statistical analysis: Try when the shellcode size is too little.

The global idea is to see at which frequency each opcode appears in the code and then compare with the frequency distribution of plain text opcodes in a shellcode database or a malware database as well as a goodware database (trough less accurate).

According to the work of `Babak Bashari Rad`, we already have the statistical distribution for main opcodes in x86:

[![image to statistics](https://www.researchgate.net/profile/Babak-Bashari-Rad/publication/235641144/figure/tbl2/AS:669345520828438@1536595816633/Opcodes-Statistics-for-each-class-of-our-data-set.png)](https://www.researchgate.net/profile/Babak-Bashari-Rad/publication/235641144/figure/tbl2/AS:669345520828438@1536595816633/Opcodes-Statistics-for-each-class-of-our-data-set.png)

[See his work here](https://www.researchgate.net/figure/Opcodes-Statistics-for-each-class-of-our-data-set_tbl2_235641144).

So let's calculate frequency distribution in order to compare the plain with the cipher text and see similarities.

```rust
[package]
name = "caesar_shellcode_1_statistical_analysis"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
cryptatools-core = { git = "https://github.com/gogo2464/cryptatools-rs", package = 'cryptatools-core' }
serde_json = "1.0.91"
r2pipe = { git = "https://github.com/RHL120/r2pipe.rs", branch = "windows_bad" }
itertools = "0.10.5"
```

```rust
use r2pipe::R2Pipe;
use r2pipe::open_pipe;
use cryptatools_core::utils::alphabets::Alphabet;
use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::Statistical;
use std::u8;

fn read_plain_text(cipher_text: String) -> Vec<u8> {
  let mut bytes = Vec::new();
  for o in (0..cipher_text.len()).step_by(2) {
	let left = cipher_text.chars().nth(o).unwrap();
	let right = cipher_text.chars().nth(o+1).unwrap();
	let mut opcode = String::from(left);
	opcode.push(right);
	bytes.push(u8::from_str_radix(&opcode, 16).unwrap());
  }	
  
  bytes
}

fn main() {
  let mut r2p = open_pipe!(Some("bin")).unwrap();
  let mut cipher_text = String::from(r2p.cmd("p8 0x1e @ 0x2c ;").unwrap());
  cipher_text.remove(cipher_text.len()-1);
  cipher_text.remove(cipher_text.len()-1);
  
  println!("cipher text: {:?}", cipher_text);
  
  let unknow_opcode_alphabet = Alphabet::new_empty().unknow_opcodes();
  
  let bytes = read_plain_text(cipher_text);
  
  let stat = Statistical::new(unknow_opcode_alphabet.clone());
  let stat_percentage = stat.guess_statistical_distribution(bytes);
  
  for character in stat_percentage {
	  for opcode in character.0 {
		  println!("opcode {:x}, statistic: {:?}", opcode, character.1);
	  }
	  
  }
  
  r2p.close();
}
```

Let's run it with:

```powershell
cargo run
   Compiling caesar_shellcode_1_statistical_analysis v0.1.0 (C:\Users\TRUNCATED\Desktop\cryptatools-rs\tutos\caesar_shellcode_1_statistical_analysis)
    Finished dev [unoptimized + debuginfo] target(s) in 1.15s
     Running `target\debug\caesar_shellcode_1_statistical_analysis.exe`
cipher text: "38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487"
opcode e9, statistic: 0.03333333333333333
opcode e8, statistic: 0.03333333333333333
opcode 5a, statistic: 0.03333333333333333
opcode b7, statistic: 0.03333333333333333
opcode 36, statistic: 0.16666666666666666
opcode ea, statistic: 0.03333333333333333
opcode 57, statistic: 0.06666666666666667
opcode 6f, statistic: 0.13333333333333333
opcode 69, statistic: 0.06666666666666667
opcode 87, statistic: 0.03333333333333333
opcode 38, statistic: 0.03333333333333333
opcode 12, statistic: 0.03333333333333333
opcode c7, statistic: 0.03333333333333333
opcode 68, statistic: 0.03333333333333333
opcode 75, statistic: 0.03333333333333333
opcode 70, statistic: 0.03333333333333333
opcode 90, statistic: 0.1
opcode 7a, statistic: 0.03333333333333333
opcode d4, statistic: 0.03333333333333333
```

The most frequent opcodes are: `36`, `6f` and `90`.

These could be either: `mov`, `push` or `call`.


### 3.1.1.1 Is 36 a mov (89)?

Let's manually try to replace `36` by `mov` and see:

```powershell
rasm2 -a x86 -b 32 "mov eax, eax"
89c0
```

so let's replace `36` by `89`.

```powershell
$text = "38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487" ; $text = $text -replace "36","89"
rasm2 -a x86 -b 32 -D $text

0x00000000   2                     38c7  cmp bh, al
0x00000002   1                       57  push edi
0x00000003   1                       6f  outsd dx, dword [esi]
0x00000004   7           69687a6f6f6970  imul ebp, dword [eax + 0x7a], 0x70696f6f
0x0000000b   2                     7589  jne 0xffffff96
0x0000000d   1                       6f  outsd dx, dword [esi]
0x0000000e   6             8989898990ea  mov dword [ecx - 0x156f7677], ecx
0x00000014   1                       57  push edi
0x00000015   1                       90  nop
0x00000016   5               e95a90e8b7  jmp 0xb7e89075
0x0000001b   2                     12d4  adc dl, ah
0x0000001d   1                       87  invalid
```

We have an invalid opcode. We can only be wrong.


### 3.1.1.2 Is 36 a push (50) ?

Let's manually try to replace `36` by `push` and see:

```powershell
rasm2 -a x86 -b 32 "push eax"
50
```
As I guess there could be many opcode for `push` dependig of the second operand. We will use guess it is a call of we do not find better before.

### 3.1.1.3 Is 36 a call (e8)?

```powershell
rasm2 -a x86 -b 32 "call 0x00"
e8fbffffff
```

```powershell
$text = "38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487" ; $text = $text -replace "36","89"
rasm2 -a x86 -b 32 -D $text
0x00000000   2                     38c7  cmp bh, al
0x00000002   1                       57  push edi
0x00000003   1                       6f  outsd dx, dword [esi]
0x00000004   7           69687a6f6f6970  imul ebp, dword [eax + 0x7a], 0x70696f6f
0x0000000b   2                     7589  jne 0xffffff96
0x0000000d   1                       6f  outsd dx, dword [esi]
0x0000000e   6             8989898990ea  mov dword [ecx - 0x156f7677], ecx
0x00000014   1                       57  push edi
0x00000015   1                       90  nop
0x00000016   5               e95a90e8b7  jmp 0xb7e89075
0x0000001b   2                     12d4  adc dl, ah
0x0000001d   1                       87  invalid
```
Invalid. It is a fail!

### 3.1.1.4 Is 36 another push ?

Now we have eliminated any other possibility, we know that `36` is probably register or stack data. let's see latter.

### 3.1.2.1 is `6f` a `mov`, `push` or a `call`. Let's try a `mov`

```powershell
rasm2 -a x86 -b 32 "mov eax, eax"
89c0
```

```powershell
$text = "38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487" ; $text = $text -replace "6f","89"
rasm2 -a x86 -b 32 -D $text
0x00000000   2                     38c7  cmp bh, al
0x00000002   1                       57  push edi
0x00000003   3                   896968  mov dword [ecx + 0x68], ebp
0x00000006   2                     7a89  jp 0xffffff91
0x00000008   3                   896970  mov dword [ecx + 0x70], ebp
0x0000000b   2                     7536  jne 0x43
0x0000000d   2                     8936  mov dword [esi], esi
0x0000000f   4                 36363690  nop
0x00000013   7           ea5790e95a90e8  ljmp 0xe890:0x5ae99057
0x0000001a   2                     b712  mov bh, 0x12
0x0000001c   2                     d487  aam 0x87
```

Seems to be possible. But the `jp` instruction is weird. Let's continue to see.

### 3.1.2.2 is `6f` a `mov`, `push` or a `call`. Let's try a `push`

```powershell
rasm2 -a x86 -b 32 "push 0x12345678"
6878563412
```

```powershell
$text = "38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487" ; $text = $text -replace "6f","68"
rasm2 -a x86 -b 32 -D $text
0x00000000   2                     38c7  cmp bh, al
0x00000002   1                       57  push edi
0x00000003   5               6869687a68  push 0x687a6869
0x00000008   5               6869707536  push 0x36757069
0x0000000d   5               6836363636  push 0x36363636
0x00000012   1                       90  nop
0x00000013   7           ea5790e95a90e8  ljmp 0xe890:0x5ae99057
0x0000001a   2                     b712  mov bh, 0x12
0x0000001c   2                     d487  aam 0x87
```

Seems more than likely possible. We remember the values `0x687a6869`, `0x36757069` and `0x36363636` as data for future analysis.

### 3.1.3.1 is `90` a `mov`, `push` or a `call`. Let's try a `push`.

```powershell
rasm2 -a x86 -b 32 "mov eax, eax"
89c0
```

```powershell
$text = "38c7576f69687a6f6f697075366f3636363690ea5790e95a90e8b712d487" ;
$text = $text -replace "6f","68"
$text = $text -replace "90","89"
rasm2 -a x86 -b 32 -D $text
0x00000000   2                     38c7  cmp bh, al
0x00000002   1                       57  push edi
0x00000003   5               6869687a68  push 0x687a6869
0x00000008   5               6869707536  push 0x36757069
0x0000000d   5               6836363636  push 0x36363636
0x00000012   2                     89ea  mov edx, ebp
0x00000014   1                       57  push edi
0x00000015   2                     89e9  mov ecx, ebp
0x00000017   1                       5a  pop edx
0x00000018   2                     89e8  mov eax, ebp
0x0000001a   2                     b712  mov bh, 0x12
0x0000001c   2                     d487  aam 0x87
```

The shellcode sample is too little... We can not go more far...

### 3.2 Caesar brute force

After reverse engineering of the shellcode decryptror, we now know that the encryption algorithm is caesar. So let's implement a brute forcer to decrypt caesar algorithm.

The real challenge is, how to detect plain text x86 32 bits opcodes?

The first idea is to disassemble the code and see if no instruction is invalid. Sadly this method is not perfect. the 8051 has 255 instructions! Even this x86_32 sample encrypted shellcode has not any invalid instruction.

Whe can then combine it to two tools:
1- the frequency analysis as previously seen. The idea is to see if the decrypted opcodes have the same repartition as plain text.
2- the coincidence index. Same concept but also considere the repartition with the previous instructions.


```powershell
cargo new caesar_shellcode_2_caesar_brute_forcer
```

Cargo.toml
```rust
[package]
name = "caesar_shellcode_2_caesar_brute_forcer"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
cryptatools-core = { git = "https://github.com/gogo2464/cryptatools-rs", package = 'cryptatools-core' }
serde_json = "1.0.91"
r2pipe = { git = "https://github.com/RHL120/r2pipe.rs", branch = "windows_bad" }
itertools = "0.10.5"
```


main.rs
```rust
use r2pipe::R2Pipe;
use r2pipe::open_pipe;
use cryptatools_core::utils::alphabets::Alphabet;
use cryptatools_core::cryptanalysis::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::Statistical;
use cryptatools_core::cryptography::classical::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
use std::u8;
use std::fmt::Write;

fn read_plain_text(cipher_text: String) -> Vec<u8> {
  let mut bytes = Vec::new();
  for o in (0..cipher_text.len()).step_by(2) {
	let left = cipher_text.chars().nth(o).unwrap();
	let right = cipher_text.chars().nth(o+1).unwrap();
	let mut opcode = String::from(left);
	opcode.push(right);
	bytes.push(u8::from_str_radix(&opcode, 16).unwrap());
  }	
  
  bytes
}

fn convert_u8_to_text(u8_vector: Vec<u8>) -> String {
  let mut string = String::new();
  for num in u8_vector {
	if num >= 0x0f {
		write!(&mut string, "{num:x}");
	} else {
		write!(&mut string, "0{num:x}");
	}
  }
  
  string
}

fn is_plain_text(probably_cipher_text: Vec<u8>) -> bool {
	  let mut r2p = open_pipe!(Some("-")).unwrap();
	  
	  r2p.cmd("e anal.arch=x86 ; e asm.bits=32 ;").unwrap();
	  
	  let string_probably_cipher_text = convert_u8_to_text(probably_cipher_text.clone());
	  let mut cmd = String::from("");
	  write!(&mut cmd, "wx {string_probably_cipher_text} ;");
	  
	  r2p.cmd(&cmd).unwrap();
	  let instructions = String::from(r2p.cmd("pI 0x1e @ 0x00 ;").unwrap());
	  
	  println!("instructions: {:?}", instructions);
	  
	  if instructions.find("invalid").is_none() == false {
		return false;  
	  }
	
	  let unknow_opcode_alphabet = Alphabet::new_empty().unknow_opcodes();
	  let stat = Statistical::new(unknow_opcode_alphabet.clone());
	  let stat_percentage = stat.guess_statistical_distribution(probably_cipher_text);
	  
	  for character in stat_percentage {
		  for opcode in character.0 {
			  println!("opcode {:x}, statistic: {:?}", opcode, character.1);
			  if opcode == 0x89 && character.1 > 20.0/100.0 && character.1 < 40.0/100.0 { //mov
			      return false;
		      }
		  }
	  }
	  
	  true
}

fn main() {
  let mut r2p = open_pipe!(Some("bin")).unwrap();
  let mut cipher_text = String::from(r2p.cmd("p8 0x1e @ 0x2c ;").unwrap());
  cipher_text.remove(cipher_text.len()-1);
  cipher_text.remove(cipher_text.len()-1);
  
  println!("cipher text: {:?}", cipher_text);
  
  let unknow_opcode_alphabet = Alphabet::new_empty().unknow_opcodes();
  let bytes = read_plain_text(cipher_text);
  let c = CaesarNumberAlgorithm::new(unknow_opcode_alphabet.into());
  let mut key = 0;
  let mut plain_text_found = false;
  let is_decrypted = c.decrypt_by_opcode_shift(bytes, key);
  key += 1;
  while plain_text_found == false {
	  let is_decrypted = c.decrypt_by_opcode_shift(is_decrypted.clone(), key);
	  plain_text_found = is_plain_text(is_decrypted.clone());
	  if plain_text_found == true {
		  break;
      }
	  key += 1;
  }
  
  println!("plain text:  {:?} decrypted with key {:?}", is_decrypted, key);
  r2p.close();
}```


### 3.3 Caesar breaking using reverse engineering analaysis and implementing a decryptor with cryptatools.

Now, let's determine the algorithm using reverse engineering method!

Sometimes assembly language could be hard to read and reverse due to counter-reverse engineering protections. It was not the case of this protection. Then let's continue to dig the reverse engineering.

```assembly
decode:
    cmp byte [esi], 7
    jl lowbound
    sub byte [esi], 7
    jmp 0x22
lowbound:
    xor ebx, ebx
    xor edx, edx
    mov bl, 7
    mov dl, 0xff
    inc dx
    sub bl, byte [esi]
    sub dx, bx
    mov byte [esi], dl
common_command:
    inc esi
    loop 7
```

After the reverse engineering, we already know that the caesar key is 7 from the instruction `loop 7`. Let's create a decryptor from cryptatools.


```shell
cargo new caesar_shellcode_decryptor ;
cd caesar_shellcode_decryptor ;
```

Let's edit the `Cargo.toml`:

```rust
[package]
name = "caesar_shellcode_decryptor"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
cryptatools-core = { git = "https://github.com/gogo2464/cryptatools-rs", package = 'cryptatools-core' }
serde_json = "1.0.91"
r2pipe = { git = "https://github.com/RHL120/r2pipe.rs", branch = "windows_bad" }
itertools = "0.10.5"
```

Now edit the `main.rs`

```rust
use r2pipe::R2Pipe;
use r2pipe::open_pipe;
use cryptatools_core::cryptography::classical::encryption::monoalphabetic_ciphers::caesar_number::CaesarNumberAlgorithm;
use cryptatools_core::utils::alphabets::Alphabet;
use std::u8;
use itertools::Itertools;
use std::fmt::Write;

fn read_plain_text(cipher_text: String) -> Vec<u8> {
  let mut bytes = Vec::new();
  for o in (0..cipher_text.len()).step_by(2) {
	let left = cipher_text.chars().nth(o).unwrap();
	let right = cipher_text.chars().nth(o+1).unwrap();
	let mut opcode = String::from(left);
	opcode.push(right);
	bytes.push(u8::from_str_radix(&opcode, 16).unwrap());
  }	
  
  bytes
}


fn main() {
  let mut r2p = open_pipe!(Some("bin")).unwrap();
  let mut cipher_text = String::from(r2p.cmd("p8 0x1e @ 0x2c ;").unwrap());
  cipher_text.remove(cipher_text.len()-1);
  cipher_text.remove(cipher_text.len()-1);
  
  println!("cipher text: {:?}", cipher_text);
  
  let unknow_opcode_alphabet = Alphabet::new_empty().unknow_opcodes();
  let mut c: CaesarNumberAlgorithm = CaesarNumberAlgorithm::new(unknow_opcode_alphabet.into());
  
  let bytes = read_plain_text(cipher_text);
  let decrypted = c.decrypt_by_opcode_shift(bytes, 7);

  let mut string = String::new();
  for num in decrypted {
	if num >= 0x0f {
		write!(&mut string, "{num:x}");
	} else {
		write!(&mut string, "0{num:x}");
	}
  }
  
  println!("plain text : {:?}", string);
  
  r2p.close();
}
```

```shell
$ rasm2 -a x86 -b 32 -D "31c05068626173686862696e2f682f2f2f2f89e35089e25389e1b00bcd80"
0x00000000   2                     31c0  xor eax, eax
0x00000002   1                       50  push eax
0x00000003   5               6862617368  push 0x68736162
0x00000008   5               6862696e2f  push 0x2f6e6962
0x0000000d   5               682f2f2f2f  push 0x2f2f2f2f
0x00000012   2                     89e3  mov ebx, esp
0x00000014   1                       50  push eax
0x00000015   2                     89e2  mov edx, esp
0x00000017   1                       53  push ebx
0x00000018   2                     89e1  mov ecx, esp
0x0000001a   2                     b00b  mov al, 0xb
0x0000001c   2                     cd80  int 0x80
> radare2 --
 -- In radare we trust
[0x00000000]> ? 0x68736162
segment 6873000:6162
string  "bash"
[0x00000000]> ? 0x2f6e6962
string  "bin/"
[0x00000000]> ? 0x2f2f2f2f
string  "////"
```

### 3.4 Caesar breaking using only reverse engineering analaysis by debugging. No cryptanalysis.

In this part we are going to debug the full shellcode in order to decrypt it. We need to have a Linux OS now.

```shell
radare2 -a x86 -b 32 -e esil.romem=true -e emu.write=true -e io.cache=true -s 0x0 -c "aei; aeip; aeim 0xffffd000 0x2000 stack; aesu 0x25; Vp" bin
```

```shell
> i | grep size
size     0x4a
> pI 0x4a @ 0x00
jmp 0x27
pop esi
xor ecx, ecx
mov cl, 0x1e
cmp byte [esi], 7
jl 0x11
sub byte [esi], 7
jmp 0x22
xor ebx, ebx
xor edx, edx
mov bl, 7
mov dl, 0xff
inc dx
sub bl, byte [esi]
sub dx, bx
mov byte [esi], dl
inc esi
loop 7
jmp 0x2c
call 2
xor eax, eax
push eax
push 0x68736162
push 0x2f6e6962
push 0x2f2f2f2f
mov ebx, esp
push eax
mov edx, esp
push ebx
mov ecx, esp
mov al, 0xb
int 0x80
```

The decrypted shellcode:

```shell
xor eax, eax
push eax
push 0x68736162
push 0x2f6e6962
push 0x2f2f2f2f
mov ebx, esp
push eax
mov edx, esp
push ebx
mov ecx, esp
mov al, 0xb
int 0x80
```

absolutely correspond to what we found using cryptanalysis!!!


### 4 Conclusion

Break caesar algorithm with statistic is trivial on plain text. It is different when the executables are encrypted because we have to deal with the opcode encryption as well as detect plain text assembly languages. Trough in this case, the esiest way was to implement a decryptor after reverse engineering the key, it is sometimes impossible in the case of a malware that deletes the key or provide access to the key only from a remote server that could be disconnected.

Hopefully, the byte parsing system of `cryptatools` allow to run cryptanalysis attack against these bytes as well as recognize plain text from bytes.

Once the plain text is found, we could use `radare2` emulation to emulate and run a single function: the one with plain text opcodes.

### Sources and references

[Shellcode coding tutorial](https://nasmland645.wordpress.com/2015/02/08/assignment-4-custom-shellcode-encoder/)

[Shellstorm shellcode sample](http://shell-storm.org/shellcode/files/shellcode-900.html)