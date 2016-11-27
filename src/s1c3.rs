use std::ascii::AsciiExt;
use std::str;

use utils::hex_to_bytes;
use s1c2::fixed_xor;

/// Scores ASCII test represented by byte array. The higher the score, the more common
/// English characters the text contains. Letter frequencies are taken from
/// https://en.wikipedia.org/wiki/Letter_frequency.
fn score_text(text: &str) -> usize {
    let frequencies = "zqxjkvbpygfwmucldrhsnioate ";
    text.chars().map(|letter| {
        frequencies.find(letter.to_ascii_lowercase()).map_or(0, |score| score + 1)
    }).sum()
}

/// Tries to decrypt text encrypted with a single character XOR
/// encryption.
pub fn decrypt_xor(ciphertext: &str) -> Option<(char, String)> {
    let cipherbytes = hex_to_bytes(ciphertext);

    // 32 to 127 should cover printable ASCII characters
    (32..128).map(|character| {
        let cipher = vec![character; cipherbytes.len()];
        let plaintext = fixed_xor(&cipherbytes, &cipher);
        (character as char, String::from_utf8(plaintext).expect("Wasn't UTF-8"))
    }).max_by_key(|a| score_text(&a.1))
}

#[test]
fn test_score_text() {
    assert!(score_text(" ") > score_text("e"));
    assert_eq!(score_text("Z"), score_text("z"));
    assert!(score_text("$") < score_text("a"));
}

#[test]
fn test_decrypt_xor() {
    assert_eq!(decrypt_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
               Some(('X', "Cooking MC's like a pound of bacon".to_string())));
}
