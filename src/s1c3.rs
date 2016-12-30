use std::collections::HashMap;
use std::cmp::Ordering;
use std::ascii::AsciiExt;

use utils::{hex_to_bytes, float_cmp};
use s1c2::fixed_xor;

pub struct DecryptionResult {
    pub ciphertext: Vec<u8>,
    pub plaintext: Vec<u8>,
    pub key: Vec<u8>,
    pub score: f32
}

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
pub fn decrypt_xor(ciphertext: &[u8]) -> Option<DecryptionResult> {
    (0..128).map(|key| {
        let cipher = vec![key; ciphertext.len()];
        let plaintext = fixed_xor(ciphertext, &cipher);
        DecryptionResult { score: score_text(&plaintext), key: vec![key],
                           plaintext: plaintext, ciphertext: ciphertext.to_vec() }
    }).min_by(|a, b| float_cmp(&a.score, &b.score))
}

#[test]
fn test_score_bytes() {
    assert!(score_text(&[b' ']) < score_text(&[b'e']));
    assert_eq!(score_text(&[b'Z']), score_text(&[b'z']));
    assert!(score_text(&[b'$']) > score_text(&[b'a']));
}

#[test]
fn test_decrypt_xor() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let result = decrypt_xor(&hex_to_bytes(input)).unwrap();
    assert_eq!(result.ciphertext, hex_to_bytes(input));
    assert_eq!(result.plaintext, Vec::from("Cooking MC's like a pound of bacon"));
    assert_eq!(result.key, vec![b'X']);
}
