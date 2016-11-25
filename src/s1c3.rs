use std::ascii::AsciiExt;
use std::str;

use utils::hex_to_bytes;
use s1c2::fixed_xor;

fn score_text(text: &[u8]) -> f32 {
    let frequencies = "xzqkjupnlgeyihrmfsdcbwaot";
    let text = str::from_utf8(text).unwrap();

    let score: usize = text.chars().map(|letter| {
        frequencies.find(letter.to_ascii_lowercase()).unwrap_or(0)
    }).sum();

    score as f32/text.len() as f32
}


pub fn decrypt_xor(ciphertext: &str) -> Option<(char, String)> {
    let cipherbytes = hex_to_bytes(ciphertext);
    let mut max = 0.0;
    let mut best_solution = None;

    for character in 32..128 {
        let cipher = vec![character; cipherbytes.len()];
        let plaintext = fixed_xor(&cipherbytes, &cipher);
        let score = score_text(&plaintext);
        if score > max {
            max = score;
            best_solution = Some((character as char, String::from_utf8(plaintext).unwrap()));
        }
    }

    best_solution
}

#[test]
fn test_score_text() {
    assert_eq!(score_text(b"x"), 0.0);
    assert_eq!(score_text(b"Z"), 1.0);
    assert_eq!(score_text(b"$"), 0.0);
    assert_eq!(score_text(b"zZz"), 1.0);
}

#[test]
fn test_decrypt_xor() {
    assert_eq!(decrypt_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
               Some(('X', "Cooking MC's like a pound of bacon".to_string())));
}
