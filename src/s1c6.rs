use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;

use utils::{hamming, float_cmp};
use s1c3::decrypt_xor;


/// Concatenates all lines in a file and returns
/// them as array of bytes.
fn load_ciphertext(path: &str) -> Vec<u8> {
    let file = BufReader::new(File::open(path).expect("Error reading file for Challenge 6"));
    file.lines().flat_map(|line| line.unwrap().into_bytes()).collect()
}

/// Returns 3 most likely keysizes for a given ciphertext.
/// Most likely keysize is first.
fn find_likely_keysize(ciphertext: &[u8]) -> Vec<usize> {
    let mut edit_distances = (2..40).map(|keysize| {
        let mut chunks = ciphertext.chunks(keysize);
        let hamming_dist = hamming(chunks.next().unwrap(),
                                   chunks.next().unwrap()).unwrap();
        (keysize,
         hamming_dist as f32 / keysize as f32)
    }).collect::<Vec<_>>();
    edit_distances.sort_by(|a, b| float_cmp(&a.1, &b.1));
    edit_distances.iter().take(3).map(|&(ks, d)| ks).collect()
}

fn transpose(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let result: Vec<Vec<u8>> = Vec::new();
    for row in matrix {
        for (i, byte) in row.iter().enumerate() {
            match result.get(i) {
                Some(v) => v.push(*byte),
                None    => result.push(vec![*byte])
            };
        }
    }
    result
}

pub fn decrypt_repeating_xor(ciphertext: &[u8]) -> i32 {
    let a = find_likely_keysize(&ciphertext)
            .iter()
            .map(|ks| ciphertext.chunks(*ks)
                                .collect::<Vec<_>>())
            .map(|chunks| transpose(chunks.to_vec()))
            .map(|t_chunk| t_chunk.iter().map(|line| decrypt_xor(&line))
                                         .collect::<Vec<_>>());
            //.min_by(|a, b| float_cmp(&a.unwrap().score, &b.unwrap().score));
    1
}

#[test]
fn test_load_ciphertext() {
    let ctext = load_ciphertext("src/files/challenge6.txt");
    assert_eq!(ctext.len(), 3836);
}

#[test]
fn test_find_keysize() {
    let ctext = load_ciphertext("src/files/challenge6.txt");
    let result = find_likely_keysize(&ctext);
    assert_eq!(result, vec![3, 2, 7]);
}

#[test]
fn test_transpose() {
    assert_eq!(transpose(vec![vec![1,2,3], vec![1,2,3], vec![1,2,3]]),
               vec![vec![1,1,1], vec![2,2,2], vec![3,3,3]]);
}
