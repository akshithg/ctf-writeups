# Crypto Attacks

A collection of crypto attacks that I encountered solving CTF challenges and reading research papers.


- [Crypto Attacks](#crypto-attacks)
  - [Block Cipher - Padding Oracle Attack](#block-cipher---padding-oracle-attack)
  - [RSA - Lazy Modulus Attack](#rsa---lazy-modulus-attack)
  - [RSA - Hastads Broadcast Attack](#rsa---hastads-broadcast-attack)


## Block Cipher - Padding Oracle Attack

The Padding Oracle Attack targets AES in CBC mode and requires encrypted messages using block ciphers in CBC mode, a padding scheme like PKCS#7 or PKCS#1 v1.5, and an oracle that reveals whether the padding of a decrypted ciphertext is valid or not.

The attack exploits a side-channel where an attacker can repeatedly send modified ciphertexts and observe whether a padding error occurs. This leakage allows the attacker to gradually decrypt the ciphertext **without knowing the key**.

The attack takes advantage of how padding is validated after decryption. If an oracle (like a server) tells you whether the padding is correct or not, you can learn information about the plaintext **byte-by-byte**.

This attack is powerful and has affected real-world protocols like SSL/TLS, ASP.NET, and Java crypto APIs. In 2010, Padding Oracle Attacks were used in the "Padding Oracle On Downgraded Legacy Encryption" ([POODLE](https://en.wikipedia.org/wiki/POODLE)) attack on SSL 3.0. [The ROBOT Attack](https://robotattack.org/) exploited a padding oracle vulnerability in the RSA encryption scheme used in TLS, allowing attackers to decrypt sensitive data.


## RSA - Lazy Modulus Attack

The Lazy Modulus Attack targets RSA & DSA cryptosystems and requires a public key with a modulus that is a product of two primes p and q, where p is much smaller than q.

The attacker can recover the private key from the public key if the modulus is poorly generated. If the modulus is a product of two primes p and q, where p is much smaller than q, the attacker can factorize the modulus using the GCD algorithm.

In 2012, research showed that 0.2% of the RSA keys on the internet were vulnerable to this attack. Reference: [Heninger et al. 2012](https://factorable.net/weakkeys12.extended.pdf)


## RSA - Hastads Broadcast Attack

The Hastad's Broadcast Attack targets RSA cryptosystems and requires three or more RSA public keys with the same modulus and different public exponents.

The attacker can recover the plaintext from the ciphertext if the same message is encrypted with three or more RSA public keys with the same modulus and different public exponents. The attacker can use the Chinese Remainder Theorem to recover the plaintext.

In 1999, Hastad showed that the attack can be used to break the RSA encryption of the Swedish Post. Reference: [Hastad 1999](https://www.cse.iitk.ac.in/users/manindra/algebra/lec11.pdf)
