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
    bytes.iter().map(|b| format!("{:x}", b)).collect()
}

#[test]
fn test_hex_to_bytes() {
    assert_eq!(hex_to_bytes("ff"), [255]);
}

#[test]
fn test_bytes_to_hex() {
    assert_eq!(bytes_to_hex(&[255]), "ff");
}
