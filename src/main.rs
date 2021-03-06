pub mod s1c1;
pub mod s1c2;
pub mod s1c3;
pub mod s1c4;
pub mod s1c5;
pub mod s1c6;
pub mod utils;

use utils::{hex_to_bytes, bytes_to_hex, DecryptionResult};

fn main() {
    println!("Set 1 Challenge 1: Convert hex to base64");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("    Base64 representation of 0x{} is {}.", input, s1c1::hex_to_base64(input));
    println!("");

    println!("Set 1 Challenge 2: Fixed XOR");
    let first = hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let second = hex_to_bytes("686974207468652062756c6c277320657965");
    let xord = bytes_to_hex(&s1c2::fixed_xor(&first, &second));
    println!("    0x1c0111001f010100061a024b53535009181c XOR 0x686974207468652062756c6c277320657965 is 0x{}.",
             xord);
    println!("");

    println!("Set 1 Challenge 3: Single-byte XOR cipher");
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    if let Some(DecryptionResult{plaintext, key, ..}) = s1c3::decrypt_xor(&hex_to_bytes(ciphertext)) {
        println!("    Encrypted with '{}': {}", key[0] as char,
                 String::from_utf8_lossy(&plaintext.expect("Decryption failed!")));
        println!("");
    } else {
        println!("    Found no solution!");
        println!("");
    }

    println!("Set 1 Challenge 4: Detect single-character XOR");
    let result = s1c4::find_encrypted_string("src/files/challenge4.txt").unwrap().unwrap();
    println!("    line {}", String::from_utf8(result.ciphertext).unwrap());
    println!("    decrypted with {}", result.key[0] as char);
    println!("    contains {}", String::from_utf8(result.plaintext.expect("Decryption failed!")).unwrap());
    println!("");

    println!("Set 1 Challenge 5: Implement repeating-key XOR");
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let result = s1c5::repeating_key_xor(&Vec::from(plaintext), &Vec::from(key));
    println!("    '{}'", plaintext);
    println!("    XOR encrypted with '{}'", key);
    println!("    is '{}'", bytes_to_hex(&result));
    println!("");

    println!("Set 1 Challenge 6: Break repeating-key XOR");
    let input = s1c6::load_ciphertext("src/files/challenge6.txt");
    let result = s1c6::decrypt_repeating_xor(&input);
    println!("    key is '{}'", String::from_utf8_lossy(&result.key));
    println!("");
}
