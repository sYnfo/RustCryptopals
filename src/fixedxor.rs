use utils::hex_to_bytes;
use utils::bytes_to_hex;

/// Returns a hex representation of XOR of two hex values
pub fn fixed_xor(first: &str, second: &str) -> String {
    let first_bytes = hex_to_bytes(first);
    let second_bytes = hex_to_bytes(second);

    let xored_bytes: Vec<_> = first_bytes.iter().zip(second_bytes).map(|(f, s)| {
        f ^ s
    }).collect();
    
    bytes_to_hex(&xored_bytes)
}

#[test]
fn test_fixed_xor() {
    assert_eq!(fixed_xor("1c0111001f010100061a024b53535009181c",
                         "686974207468652062756c6c277320657965"),
               "746865206b696420646f6e277420706c6179");
}
