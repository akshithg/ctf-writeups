use std::fs::File;
use std::io::BufRead;

extern crate hex;
extern crate base64;

// Set 1: Basics

// Challenge 1: Convert hex to base64
// Input: "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
// Output: "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::encode(&bytes)
}

// Challenge 2: Fixed XOR
// Input: "1c0111001f010100061a024b53535009181c" and "686974207468652062756c6c277320657965"
// Output: "746865206b696420646f6e277420706c6179"
fn xor(a: &str, b: &str) -> String {
    let a_bytes = hex::decode(a).unwrap();
    let b_bytes = hex::decode(b).unwrap();
    let mut result = Vec::new();
    for i in 0..a_bytes.len() {
        result.push(a_bytes[i] ^ b_bytes[i]);
    }
    hex::encode(result)
}

// Challenge 3: Single-byte XOR cipher
// Input: "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
// Output: "Cooking MC's like a pound of bacon"
fn single_byte_xor(cipher: &str) -> String {
    let cipher_bytes = hex::decode(cipher).unwrap();
    let mut result = Vec::new();
    let mut max_score = 0;
    let mut key = 0;

    for i in 0..=255 {
        let mut score = 0;
        let mut decrypted = Vec::new();

        for &byte in &cipher_bytes {
            decrypted.push(byte ^ i);
        }

        // Check if the decrypted bytes form valid UTF-8
        if let Ok(text) = String::from_utf8(decrypted.clone()) {
            // Score based on English text characteristics
            for c in text.chars() {
                if c.is_ascii_alphabetic() {
                    score += 3;
                } else if c.is_ascii_whitespace() {
                    score += 2;
                } else if c.is_ascii_punctuation() {
                    score += 1;
                } else if !c.is_ascii() {
                    score -= 5; // Penalize non-ASCII characters
                }
            }

            if score > max_score {
                max_score = score;
                result = decrypted;
                key = i;
            }
        }
    }

    // println!("Key: {} (0x{:02x})", key as char, key);
    String::from_utf8(result).unwrap_or_else(|_| "Invalid UTF-8".to_string())
}

// Challenge 4: Detect single-character XOR
// Input: A file containing multiple hex-encoded strings, each encrypted with a single-byte XOR cipher
// Output: The decrypted string with the highest score
#[allow(dead_code)]
fn detect_single_byte_xor(ciphers: Vec<&str>) -> String {
    let mut result = String::new();
    let mut max_score = 0;
    for cipher in ciphers {
        let decrypted = single_byte_xor(cipher);
        let score = decrypted.chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).count();
        if score > max_score {
            max_score = score;
            result = decrypted;
        }
    }
    result
}

// Challenge 5: Repeating-key XOR
// Input: "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
// Key: "ICE"
// Output: "0b363b69736f6d657a616c756d656e7420636f6e76657273696f6e20616e6420627265616b696e67206120706f69736f6e6f7573206d757368726f6f6d"
fn repeating_key_xor(plain_text: &str, key: &str) -> String {
    let plain_text_bytes = plain_text.as_bytes();
    let key_bytes = key.as_bytes();
    let mut result = Vec::new();
    for i in 0..plain_text_bytes.len() {
        result.push(plain_text_bytes[i] ^ key_bytes[i % key_bytes.len()]);
    }
    hex::encode(result)
}

// Hamming distance
fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    let mut distance = 0;
    for i in 0..a.len() {
        let mut x = a[i] ^ b[i];
        while x > 0 {
            distance += (x & 1) as usize;
            x >>= 1;
        }
    }
    distance
}

// Challenge 6: Break repeating-key XOR
// Input: A base64-encoded string (from src/6.txt)
// Output: The decrypted string
// This challenge is more complex and involves finding the key size, breaking the cipher, and decrypting it.
fn break_repeating_key_xor(cipher: &str) -> String {
    let cipher_bytes = hex::decode(cipher).unwrap();
    let mut key = Vec::new();
    let mut min_distance: usize = usize::MAX;
    for key_size in 2..40 {
        let mut distance = 0usize;
        let mut count = 0;
        for i in 0..(cipher_bytes.len() / key_size) - 1 {
            let a = &cipher_bytes[i * key_size..(i + 1) * key_size];
            let b = &cipher_bytes[(i + 1) * key_size..(i + 2) * key_size];
            distance += hamming_distance(a, b);
            count += 1;
        }
        if count > 0 {
            distance /= count;
            distance /= key_size; // Normalize by key size
        }
        if distance < min_distance {
            min_distance = distance;
            key = Vec::new();
            for i in 0..key_size {
                let mut block = Vec::new();
                for j in 0..(cipher_bytes.len() / key_size) {
                    if i + j * key_size < cipher_bytes.len() {
                        block.push(cipher_bytes[i + j * key_size]);
                    }
                }
                let block_hex = hex::encode(block);
                let single_byte_xor_result = single_byte_xor(&block_hex);
                key.push(single_byte_xor_result.as_bytes()[0]);
            }
        }
    }
    let key_str = String::from_utf8(key).unwrap();
    repeating_key_xor(&cipher, &key_str)
}

fn main() {

    print!("Challenge 1: ");
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64 = hex_to_base64(hex);
    println!("{}", base64);

    print!("Challenge 2: ");
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let xor_result = xor(a, b);
    println!("{}", xor_result);

    print!("Challenge 3: ");
    let c = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let single_byte_xor_result = single_byte_xor(c);
    println!("{}", single_byte_xor_result);

    print!("Challenge 4: ");
    // read from file at src/4.txt
    let ciphers: Vec<String> = std::io::BufReader::new(File::open("src/4.txt").unwrap()).lines().map(|line| line.unwrap()).collect();
    let ciphers: Vec<&str> = ciphers.iter().map(|s| s.as_str()).collect();
    let detect_single_byte_xor_result = detect_single_byte_xor(ciphers);
    println!("{}", detect_single_byte_xor_result);

    print!("Challenge 5: ");
    let plain = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let repeating_key_xor_result = repeating_key_xor(plain, key);
    println!("{}", repeating_key_xor_result);

    print!("Challenge 6: ");
    // read from file at src/6.txt
    let base64 = std::io::BufReader::new(File::open("src/6.txt").unwrap()).lines().map(|line| line.unwrap()).collect::<String>();
    let bytes = base64::decode(&base64).unwrap();
    let plain_text = String::from_utf8(bytes).unwrap();
    println!("{}", plain_text);

}
