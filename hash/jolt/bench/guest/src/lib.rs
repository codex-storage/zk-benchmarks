// #![cfg_attr(feature = "guest", no_std)]
#![no_main]

use sha2::{Sha256, Digest};
use sha3::Keccak256;
use blake3::hash;
use blake2::Blake2b512;

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
fn blake2(input: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b512::new();
    hasher.update(input);
    let result = hasher.finalize();
    
    result.to_vec()
}

// poseidon2 over babybear
#[jolt::provable(stack_size = 10000, memory_size = 100000000)]
fn poseidon2_babybear(input: Vec<u32>) -> Vec<u32> {
    let mut hash_data: Vec<BabyBearElem> = Vec::new();

    for i in 0..input.len() {
        let a_uncompressed = BabyBearElem::from(input[i] as u32);
        hash_data.push(a_uncompressed);
    }

    let mut binding = Poseidon2HashSuite::new_suite().hashfn.hash_elem_slice(hash_data.as_slice());
    let result = binding.as_mut_words().to_vec();

    result
    
}