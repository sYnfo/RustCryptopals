pub mod hex2base64;
pub mod fixedxor;
pub mod singlebytexor;
pub mod utils;

use utils::hex_to_bytes;
use utils::bytes_to_hex;

fn main() {
    println!("Set 1 Challenge 1: Convert hex to base64");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("    Base64 representation of 0x{} is {}.", input, hex2base64::hex_to_base64(input));
    println!("");

    println!("Set 1 Challenge 2: Fixed XOR");
    let first = hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let second = hex_to_bytes("686974207468652062756c6c277320657965");
    let xord = bytes_to_hex(&fixedxor::fixed_xor(&first, &second));
    println!("    0x1c0111001f010100061a024b53535009181c XOR 0x686974207468652062756c6c277320657965 is 0x{}.",
             xord);
    println!("");

    println!("Set 1 Challenge 3: Single-byte XOR cipher");
    let solution =
    singlebytexor::decrypt_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
}
