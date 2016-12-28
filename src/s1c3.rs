use std::str;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::ascii::AsciiExt;

use utils::hex_to_bytes;
use s1c2::fixed_xor;

pub fn score_text(data: &[u8]) -> f32 {
    // Relative frequencies of English ASCII character
    let mut english = HashMap::new();
    english.insert(b'a', 8.167/100.0);
    english.insert(b'b', 1.492/100.0);
    english.insert(b'c', 2.782/100.0);
    english.insert(b'd', 4.253/100.0);
    english.insert(b'e', 12.702/100.0);
    english.insert(b'f', 2.228/100.0);
    english.insert(b'g', 2.015/100.0);
    english.insert(b'h', 6.094/100.0);
    english.insert(b'i', 6.966/100.0);
    english.insert(b'j', 0.153/100.0);
    english.insert(b'k', 0.772/100.0);
    english.insert(b'l', 4.025/100.0);
    english.insert(b'm', 2.406/100.0);
    english.insert(b'n', 6.749/100.0);
    english.insert(b'o', 7.507/100.0);
    english.insert(b'p', 1.929/100.0);
    english.insert(b'q', 0.095/100.0);
    english.insert(b'r', 5.987/100.0);
    english.insert(b's', 6.327/100.0);
    english.insert(b't', 9.056/100.0);
    english.insert(b'u', 2.758/100.0);
    english.insert(b'v', 0.978/100.0);
    english.insert(b'w', 2.360/100.0);
    english.insert(b'x', 0.15/100.0);
    english.insert(b'y', 1.974/100.0);
    english.insert(b'z', 0.074/100.0);
    english.insert(b' ', 13.702/100.0); // This one is a guess

    let lower: Vec<u8> = data.iter().map(|b| (*b as char).to_ascii_lowercase() as u8).collect();
    score_bytes(&lower, &english)
}

/// Scores ASCII test represented by byte array. A score of 0 means input frequencies
/// are identical to the expected frequencies, higher score means more difference.
/// Letter frequencies are taken from https://en.wikipedia.org/wiki/Letter_frequency.
pub fn score_bytes(data: &[u8], expected_freq: &HashMap<u8, f32>) -> f32 {
    data.iter().map(|b| 1.0 - expected_freq.get(b).unwrap_or(&0.0))
               .sum::<f32>() / data.len() as f32
}

/// Tries to decrypt text encrypted with a single character XOR
/// encryption.
pub fn decrypt_xor(ciphertext: &str) -> Option<(char, Vec<u8>)> {
    let cipherbytes = hex_to_bytes(ciphertext);

    // 32 to 127 should cover printable ASCII characters
    (32..128).map(|character| {
        let cipher = vec![character; cipherbytes.len()];
        let plaintext = fixed_xor(&cipherbytes, &cipher);
        (character as char, plaintext)
    }).min_by(|a, b| score_text(&a.1).partial_cmp(&score_text(&b.1))
                                     .unwrap_or(Ordering::Equal))
}

#[test]
fn test_score_bytes() {
    // assert!(score_bytes(" ", relative_frequencies) < score_bytes("e", relative_frequencies));
    assert_eq!(score_text(&['Z' as u8]), score_text(&['z' as u8]));
    assert!(score_text(&['$' as u8]) > score_text(&['a' as u8]));
}

#[test]
fn test_decrypt_xor() {
    assert_eq!(decrypt_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
               Some(('X', Vec::from("Cooking MC's like a pound of bacon"))));
}
