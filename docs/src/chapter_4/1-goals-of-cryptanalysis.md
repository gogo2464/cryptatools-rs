# Why Doing Cryptanalysis

## 1 - What Is Cryptanalysis?

Cryptography is a way to make an information impossible to understand from an attackant (generally not legitimate to read the information) that wants to read it.

Cryptography is often done by compagnies or end users who wants to protect these informations such as password, private conversation out of scope of an adversarial spy.

Cryptanalysis is then the method to break cryptographic algorithm.

## 2 - Goals Of Cryptanalysis

Sometimes, the person who wants to read information is not illegitimate.

## 2.1 Malware Analysis

### 2.1.1 - Ransomware Decryption

It could be the case if an hacker has encrypted computer files of a compagny in order to ask for a ransom. If the files are still present on the disk, then the cyber security analyst could make reverse engineering of the ransomware executable to determine the encryption algorithm and then write a script with `cryptatools-rs` in order to attack the encryption algoritm of the malware to decrypt the encrypted files of the victim with no paying the ransom to the malicious hacker.

### 2.1.2 - Antivirus Updating

Malware are malicious softwares errorly know as "virus" (as virus designated more specific malware that necessaily spreads over files).

Sometimes malwares are encrypted. It could be done by malicious malware writter to bypass the anti-virus or in order to stop a malware analyst to know how the malware works to stop the anti-virus developper from updating the anti virus.

## 2.2 Bug Bounty And Pentest

## 2.2.1 Bug Bounty

Softwares need to use encryption to stop an attackant to listen to them. They may use Bug Bounty to ask searchers to find vulnerabilities and reward them legally to fix them. It is a way to proove the security of a software openly to users.

Some are:
- [immunify](https://immunefi.com/)
- [zero day initiative](https://www.zerodayinitiative.com/)

### 2.2.1.1 On Network Encryption

When network is intercepted on the wifi/4G/5G or any way, you could try to code cryptanalysis exploits on software encryption in order to decipher traffic.

### 2.2.1.2 On Blockchain

There could be huge vulnerabilities on the blockchain if the blockchain is not codded correctly. As example:

- If the hash algorithm contains collisions, we could forge a fake signature and take over any account of the blockchain and steal all the money.
- If a bad random number generator is present on the blockchain, we could guess all the RSA private key of any future users. It implies that we could steal all their money.
- And so on...

See [this reference](https://github.com/slowmist/Cryptocurrency-Security-Audit-Guide/blob/main/Blockchain-Common-Vulnerability-List.md#encryption) to have a non exhaustive list of blockchain possible cyber attacks.

### 2.2.2 Pentest

Once an exploit is Public or has been reached by your pentest compagny, you can use it after having signed a contract in order to secure the compagny from cyber attacks. This process is know as pentest.

In the case of a pentest, each cryptanalysis exploit could be used to proove the lack of security of a network.

## 2.3 Cryptographic Research

In order to test a cryptographic algorithm, if you are researcher, you could need to test your cryptographic/cryptanalytic algorithm.

## 2.4 - Secrets Agencies

Each modern gouvernement use hacking as cyberwar method today. They may use electronic war method forbidden in the civil that we will not discuss here in order to steal information to other governments to watch them and prevent economical or militiry war. So even the most offensive cryptanalysis method could be done for them. This may include:

- Listenning to encrypted wifi over a hacked wifi (example cache poisonning).
- Listenning to 4/5G in order to interecept encrypted communication of smart phones.

Even if you have no contract with compagny, you can sell your exploit to a reseller of your governemental secret agency legally with zerodium.

[zerodium](https://zerodium.com/) is a kind of bug bounty platform for zero day exploit. Zerodium will exploit the vulnerability instead of fix it.