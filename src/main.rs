pub mod hex2base64;
pub mod fixedxor;
pub mod utils;

fn main() {
    println!("Set 1 Challenge 1: Convert hex to base64");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("    Base64 representation of 0x{} is {}.", input, hex2base64::hex_to_base64(input));
    println!("");

    println!("Set 1 Challenge 2: Fixed XOR");
    let xord = fixedxor::fixed_xor("1c0111001f010100061a024b53535009181c",
                                   "686974207468652062756c6c277320657965");
    println!("    0x1c0111001f010100061a024b53535009181c XOR 0x686974207468652062756c6c277320657965 is 0x{}.",
             xord);
}
