// #![cfg_attr(feature = "guest", no_std)]
#![no_main]

use sha2::{Sha256, Digest};
use sha3::Keccak256;
use blake3::hash;
use blake2::Blake2s256;

use risc0_zkp::core::hash::poseidon2::Poseidon2HashSuite;
use risc0_zkp::field::baby_bear::BabyBearElem;

extern crate alloc;
use alloc::vec::Vec;

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

// poseidon2 over babybear
#[jolt::provable(stack_size = 10000, memory_size = 100000000)]
//TODO: input should be made u32
fn poseidon2_babybear(input: &[u8]) -> String {
    let mut hash_data: Vec<BabyBearElem> = Vec::new();

    for i in 0..input.len() {
        let a_uncompressed = BabyBearElem::from(input[i] as u32);
        hash_data.push(a_uncompressed);
    }
    let result = Poseidon2HashSuite::new_suite().hashfn.hash_elem_slice(hash_data.as_slice());

    result.to_string()
    
}