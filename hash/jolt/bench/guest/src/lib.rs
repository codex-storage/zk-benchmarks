#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use sha2::{Sha256, Digest};
use sha3::Keccak256;
use blake3::hash;
use blake2::Blake2s256;

// sha256
#[jolt::provable]
fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

// keccak
#[jolt::provable]
fn sha3(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

// blake3
#[jolt::provable]
fn blake3(input: &[u8]) -> [u8; 32] {
    let result = hash(input);
    Into::<[u8; 32]>::into(result)
}

// blake2
#[jolt::provable]
fn blake2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Blake2s256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}
