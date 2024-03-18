
#![no_main]
sp1_zkvm::entrypoint!(main);

// poseidon2 https://github.com/HorizenLabs/poseidon2
use zkhash::poseidon2::poseidon2::Poseidon2;
use zkhash::poseidon2::poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS;
use zkhash::merkle_tree::merkle_tree_fp::MerkleTree;
use zkhash::fields::bn256::FpBN256;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

pub fn main() {

    let input = sp1_zkvm::io::read::<Vec<Vec<u8>>>();

    //build input
    let mut hash_data: Vec<FpBN256> = Vec::new();
    for i in 0..input.len() {
        let a_uncompressed = FpBN256::deserialize_uncompressed(&**input.get(i).unwrap()).unwrap();
        hash_data.push(a_uncompressed);
    }

    // create a poseidon2 merkle object
    let perm = Poseidon2::new(&POSEIDON2_BN256_PARAMS);
    let mut mt = MerkleTree::new(perm.clone());
    
    let hash_final = mt.accumulate(&hash_data);


    let mut hash_bytes: Vec<u8> = Vec::new();
    hash_final.serialize_uncompressed(&mut hash_bytes).unwrap();

    sp1_zkvm::io::write(&hash_bytes);

}
