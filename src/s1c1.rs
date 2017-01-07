use utils::hex_to_bytes;

/// Encode the decoded bytes into Base64
pub fn bytes_to_base64(decoded_bytes: &[u8]) -> String {
    let mut output = String::new();
    let alphabet: Vec<_> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();

    for chunk in decoded_bytes.chunks(3) {
        let c0 = chunk[0];
        let b = c0 >> 2;
        output.push(alphabet[b as usize]);

        let mut b = (c0 & 0x03) << 4;

        if let Some(c1) = chunk.get(1) {
            b |= c1 >> 4;
            output.push(alphabet[b as usize]);

            let mut b = (c1 & 0x0F) << 2;

            if let Some(c2) = chunk.get(2) {
                b |= c2 >> 6;
                output.push(alphabet[b as usize]);

                let b = c2 & 0x3F;
                output.push(alphabet[b as usize]);
            } else {
                output.push(alphabet[b as usize]);
                output.push('=');
            }
        } else {
            output.push(alphabet[b as usize]);
            output.push_str("==");
        }
    }

    output
}

/// Returns base64 representation of a hex string
pub fn hex_to_base64(hex_string: &str) -> String {
    bytes_to_base64(&hex_to_bytes(hex_string))
}

#[test]
fn test_hex_to_base64() {
    assert_eq!(
        hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}
