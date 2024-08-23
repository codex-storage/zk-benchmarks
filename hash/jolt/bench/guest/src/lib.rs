// #![cfg_attr(feature = "guest", no_std)]
#![no_main]

use sha2::{Sha256, Digest};
use sha3::Keccak256;
use blake3::hash;
use blake2::Blake2b512;
use ark_serialize::{CanonicalSerialize,CanonicalDeserialize};

use risc0_zkp::core::hash::poseidon2::Poseidon2HashSuite;
use risc0_zkp::field::baby_bear::BabyBearElem;

extern crate alloc;
use alloc::vec::Vec;

pub mod poseidon2_bn256;
use poseidon2_bn256::{Scalar, MerkleTree, Poseidon2, Poseidon2Params};

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

//TODO: too slow!! need to do some optimizations
// poseidon2 over BN256
#[jolt::provable(stack_size = 1000000, memory_size = 10000000)]
pub fn poseidon2_bn256(input: Vec<Vec<u8>>) -> Vec<u8> {
    let mut hash_data: Vec<Scalar> = Vec::new();
    for i in 0..input.len() {
        let a_uncompressed = Scalar::deserialize_uncompressed(&*input[i]).unwrap();
        hash_data.push(a_uncompressed);
    }

    let bn256_param: Poseidon2Params<Scalar> = Poseidon2Params::<Scalar>::POSEIDON2_BN256_PARAMS();
    let permutation = Poseidon2::new(bn256_param);
    let mut merkle_tree = MerkleTree::new(permutation);

    let hash_final = merkle_tree.accumulate(&hash_data);

    let mut hash_bytes: Vec<u8> = Vec::new();
    hash_final.serialize_uncompressed(&mut hash_bytes).unwrap();
    
    hash_bytes
}

