#![allow(non_snake_case)]
use methods::{
    POSEIDON2_BABYBEAR_ELF, POSEIDON2_BABYBEAR_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use zkhash::{fields::{babybear::FpBabyBear, utils::random_scalar}/* , poseidon2::poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS*/};
// use zkhash::poseidon2::poseidon2::Poseidon2;
// use std::convert::TryFrom;
use std::time::Instant;
// use zkhash::merkle_tree::merkle_tree_fp::MerkleTree;
// use std::convert::TryInto;
// use hex::encode_to_slice;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};


pub fn poseidon2_babybear_bench(mtDepth: usize) {
    
    type Scalar = FpBabyBear;

    let mut input_scalar: Vec<Vec<u8>> = Vec::new();
    let number_of_leaves: u32 = 1 << mtDepth;
    for _ in 0..number_of_leaves {
        let mut uncompressed_bytes = Vec::new();
        let a: Scalar = random_scalar();
        a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();
        input_scalar.push(uncompressed_bytes);
    }

    let env = ExecutorEnv::builder().write(&input_scalar).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, POSEIDON2_BABYBEAR_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // For example:
    let output: Vec<Vec<u8>> = receipt.journal.decode().unwrap();

    let mut output_deseralised: Vec<Scalar> = Vec::new();

    for i in 0..output.len() {
        output_deseralised.push(Scalar::deserialize_uncompressed(&**output.get(i).unwrap()).unwrap());
    }

    eprintln!("size: {:?}", output_deseralised);
    // let hash_final = FpBabyBear::deserialize_uncompressed(&*output).unwrap();

    // verify your receipt
    receipt.verify(POSEIDON2_BABYBEAR_ID).unwrap();

    
    eprintln!("Total time: {:?}", elapsed_time);
    // eprintln!("Hash: {:?}", hash_final);





    // let input2:[Scalar;16] = [Scalar::from(1), Scalar::from(2), Scalar::from(3), Scalar::from(4),Scalar::from(5), Scalar::from(6), Scalar::from(7), Scalar::from(8),Scalar::from(9), Scalar::from(10), Scalar::from(11), Scalar::from(12), Scalar::from(13), Scalar::from(14), Scalar::from(15), Scalar::from(16)];
    // let hash = merkle_tree.accumulate(&input2);

    // let hash_string = hash.0.to_string();
    // // eprintln!("merkle hash: {:?}",hex::encode(hash_string));
    // eprintln!("merkle hash: {:?}", hash_string);

    // let x = hash.0.0;
    // eprintln!("from: {:x}{:x}{:x}{:x}", x[0],x[1], x[2], x[3]);
    // eprintln!("scalar: {:?}", Scalar::from(4));

    // let a = Scalar::from(4);
    // let mut uncompressed_bytes = Vec::new();
    // a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();
    // eprintln!("compress: {:?}", uncompressed_bytes);

    // let a_uncompressed = Scalar::deserialize_uncompressed(&*uncompressed_bytes).unwrap();
    // eprintln!("uncompress: {:?}", a_uncompressed);

    // let t = poseidon2.get_t();
    // let input1: Vec<Scalar> = (0..t).map(|_| random_scalar()).collect();
    // let perm = poseidon2.permutation(&input1);
    // eprintln!("output: {:?}", perm);

    // let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    // let prover = default_prover();

    // let start_time = Instant::now();
    // // Produce a receipt by proving the specified ELF binary.
    // let receipt = prover.prove_elf(env, POSEIDON2_BN128_ELF).unwrap();
    // let elapsed_time = start_time.elapsed();

    // // For example:
    // let _output: sha::Digest = receipt.journal.decode().unwrap();

    // // verify your receipt
    // receipt.verify(POSEIDON2_BN128_ID).unwrap();

    
    // eprintln!("Total time: {:?}", elapsed_time);
    // eprintln!("Hash: {:?}", _output);
}