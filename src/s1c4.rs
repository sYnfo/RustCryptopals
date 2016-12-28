use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;

use s1c3::{decrypt_xor, DecryptionResult};

use utils::hex_to_bytes;

pub fn find_encrypted_string(file_name: &str) -> io::Result<Option<DecryptionResult>> {
    let file = BufReader::new(try!(File::open(file_name)));
    let result = file.lines().filter_map(|line| {
        let line = line.expect("Something went wrong when parsing the file!");
        decrypt_xor(&hex_to_bytes(&line))
    }).min_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal));
    Ok(result)
}

#[test]
fn test_find_excrypted_string() {
    let result = find_encrypted_string("src/files/challenge4.txt").unwrap().unwrap();
    assert_eq!(result.plaintext, Vec::from("Now that the party is jumping\n"));
    assert_eq!(result.key, Vec::from("5"));
}
