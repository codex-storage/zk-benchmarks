#![no_main]
use risc0_zkvm::guest::env;
use zkhash::poseidon2::poseidon2;
use zkhash::poseidon2::poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS;
use zkhash::merkle_tree::merkle_tree_fp::MerkleTree;
use zkhash::fields::bn256::FpBN256;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<Vec<u8>> = env::read();

    let mut hash_data: Vec<FpBN256> = Vec::new();
    for i in 0..data.len() {
        let a_uncompressed = FpBN256::deserialize_uncompressed(&**data.get(i).unwrap()).unwrap();
        hash_data.push(a_uncompressed);
    }

    
    let permutation = poseidon2::Poseidon2::new(&POSEIDON2_BN256_PARAMS);
    let mut merkle_tree = MerkleTree::new(permutation.clone());

    let hash_final = merkle_tree.accumulate(&hash_data);

    let mut hash_bytes: Vec<u8> = Vec::new();
    hash_final.serialize_uncompressed(&mut hash_bytes).unwrap();
    
    env::commit(&hash_bytes);


}
