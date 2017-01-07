use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;

extern crate itertools;
use self::itertools::Itertools;

use utils::{hamming, float_cmp, DecryptionResult};
use s1c1::bytes_to_base64;
use s1c3::decrypt_xor;
use s1c5::repeating_key_xor;


/// Decodes base64 encoded array of bytes.
fn base64_to_bytes(input: &[u8]) -> Vec<u8> {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    let mut result = Vec::new();
    for c in input.chunks(4) {
        let d = |n| alphabet.find(c[n] as char)
                            .expect("Input is not a valid base64 string");
        result.push(((d(0) << 2) | (d(1) >> 4)) as u8);
        if d(2) < 64 {
            result.push(((d(1) << 4) | (d(2) >> 2)) as u8);
            if d(3) < 64 {
                result.push(((d(2)<< 6) | d(3)) as u8);
            }
        }
    }
    result
}

/// Concatenates all lines in a file and decodes them from base64.
pub fn load_ciphertext(path: &str) -> Vec<u8> {
    let file = BufReader::new(File::open(path).expect("Error reading file for Challenge 6"));
    base64_to_bytes(&file.lines().flat_map(|line| line.unwrap()
                                                      .into_bytes())
                                 .collect::<Vec<_>>())
}

/// Returns 3 most likely keysizes for a given ciphertext.
/// Most likely keysize is first.
fn find_likely_keysize(ciphertext: &[u8]) -> Vec<usize> {
    let mut edit_distances = (2..40).map(|keysize| {
        // We take the average of the hamming distance of all combinations
        // of the first 4 chunks to improve accuracy
        (keysize,
         ciphertext.chunks(keysize)
                   .take(4)
                   .combinations(2)
                   .map(|p| hamming(p[0], p[1]).unwrap())
                   .sum::<f32>())
    }).collect::<Vec<_>>();
    edit_distances.sort_by(|a, b| float_cmp(&a.1, &b.1));
    edit_distances.iter().take(3).map(|&(ks, _)| ks).collect()
}

/// Returns a transposed matrix.
fn transpose(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for _ in 0..matrix[0].len() {
        result.push(Vec::new());
    }
    for row in matrix {
        for (i, byte) in row.iter().enumerate() {
            result[i].push(byte.to_owned());
        }
    }
    result
}

/// Returns DecryptionResult with the most likely key and the associated
/// plaintext.
pub fn decrypt_repeating_xor(ciphertext: &[u8]) -> DecryptionResult {
    let mut result = find_likely_keysize(ciphertext)
        .iter()
        // Split the input into chunks based on the predicted keysize
        .map(|ks| ciphertext.chunks(*ks)
                            .map(|c| c.to_vec())
                            .collect::<Vec<_>>())
        // Transpose the chunks
        .map(transpose)
        // Find xor key for each transposed chunk
        .map(|chunk| chunk.iter()
                          .map(|line| decrypt_xor(line))
                          .collect::<Vec<_>>())
        // Combine the partial DecryptionResults and find the best one
        .map(|rs| {
            let keys: Vec<_> = rs.iter()
                                 .map(|r| r.as_ref().unwrap().key[0])
                                 .collect();
            let scores: Vec<_> = rs.iter()
                                   .map(|r| r.as_ref().unwrap().score)
                                   .collect();
            DecryptionResult {key: keys, score: scores.iter().sum::<f32>() / scores.len() as f32,
                              ciphertext: ciphertext.to_vec(), plaintext: None}
        }).min_by(|a, b| float_cmp(&a.score, &b.score)).unwrap();
    result.plaintext = Some(repeating_key_xor(ciphertext, &result.key));
    result
}

#[test]
fn test_load_ciphertext() {
    let ctext = load_ciphertext("src/files/challenge6.txt");
    assert_eq!(ctext.len(), 2876);
}

#[test]
fn test_find_keysize() {
    let ctext = load_ciphertext("src/files/challenge6.txt");
    let result = find_likely_keysize(&ctext);
    assert!(result.contains(&29));
}

#[test]
fn test_transpose() {
    assert_eq!(transpose(vec![vec![1,2,3], vec![1,2,3], vec![1,2]]),
               vec![vec![1,1,1], vec![2,2,2], vec![3,3]]);
}

#[test]
fn test_base64_to_bytes() {
    let input = [0, 1, 2, 3, 254, 255];
    assert_eq!(base64_to_bytes(bytes_to_base64(&input).as_bytes()), input);
}

#[test]
fn test_decrypt_repeating_xor() {
    let ctext = load_ciphertext("src/files/challenge6.txt");
    let result = decrypt_repeating_xor(&ctext);
    assert_eq!(result.key , Vec::from("Terminator X: Bring the noise"));
    assert!(result.plaintext.unwrap()
                            .starts_with("I'm back and I'm ringin' the bell".as_bytes()));
}
