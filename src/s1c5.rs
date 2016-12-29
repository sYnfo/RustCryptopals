use s1c2::fixed_xor;
use utils::bytes_to_hex;


pub fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    fixed_xor(plaintext, &key.iter().cloned().cycle().take(plaintext.len()).collect::<Vec<u8>>())
}

#[test]
fn test_repeating_key_xor() {
    let input = Vec::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    assert_eq!(bytes_to_hex(&repeating_key_xor(&input, &Vec::from("ICE"))),
               "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
}
