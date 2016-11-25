use utils::hex_to_bytes;
use utils::bytes_to_hex;

/// Returns a hex representation of XOR of two hex values
pub fn fixed_xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first.iter().zip(second).map(|(f, s)| {
        f ^ s
    }).collect::<Vec<u8>>()
}

#[test]
fn test_fixed_xor() {
    let first = hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let second = hex_to_bytes("686974207468652062756c6c277320657965");
    assert_eq!(fixed_xor(&first, &second),
               hex_to_bytes("746865206b696420646f6e277420706c6179"));
}
