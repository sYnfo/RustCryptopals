extern crate num;
use self::num::traits::Float;
use std::cmp::Ordering;


/// Decode the input string from hex into individual bytes
pub fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    let input_chars: Vec<_> = hex_string.chars().collect();

    input_chars.chunks(2).map(|chunk| {
        let first_byte = chunk[0].to_digit(16).unwrap();
        let second_byte = chunk[1].to_digit(16).unwrap();
        ((first_byte << 4) | second_byte) as u8
    }).collect()
}

/// Convert array of bytes to it's hex representation
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Computes the hamming distance of two arrays of bytes
pub fn hamming(a: &[u8], b: &[u8]) -> Result<u32, &'static str> {
    if a.len() == b.len() {
        Ok(a.iter().zip(b).map(|(f, s)| (f ^ s).count_ones()).sum())
    } else {
        Err("Hamming distance is undefined for sequences of unequal lengths")
    }
}

pub fn float_cmp<T: Float>(a: &T, b: &T) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

#[test]
fn test_hex_to_bytes() {
    assert_eq!(hex_to_bytes("ff"), [255]);
}

#[test]
fn test_bytes_to_hex() {
    assert_eq!(bytes_to_hex(&[255]), "ff");
}

#[test]
fn test_bytes_to_hex_prefix() {
    assert_eq!(bytes_to_hex(&[1]), "01");
}

#[test]
fn test_hamming() {
    let input1 = "this is a test";
    let input2 = "wokka wokka!!!";
    assert_eq!(hamming(&Vec::from(input1), &Vec::from(input2)), Ok(37));
}
