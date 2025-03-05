use std::time::Instant;
use ark_serialize::CanonicalSerialize;
use ark_ff::PrimeField;

extern crate alloc;
use alloc::vec::Vec;

fn random_scalar<F: PrimeField>() -> F {
    let mut rng = ark_std::rand::thread_rng();
    F::rand(&mut rng)
}

pub fn poseidon2_bn256_bench(mt_depth: usize) {

    pub type Scalar = ark_bn254::fr::Fr;

    let mut input_scalar: Vec<Vec<u8>> = Vec::new();
    let number_of_leaves: u32 = 1 << mt_depth;
    for _ in 0..number_of_leaves {
        let mut uncompressed_bytes = Vec::new();
        let a: Scalar = random_scalar();
        a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();
        input_scalar.push(uncompressed_bytes);
    }

    let (prove_poseidon2_bn256, verify_poseidon2_bn256, guest_build_time) = {

        let start = Instant::now();
        let (prove, verify) = guest::build_poseidon2_bn256();
        let elapsed = start.elapsed();

        (prove, verify, elapsed)
    };

    let (output, proof, proving_time) = {

        let start = Instant::now();
        let (output, proof) = prove_poseidon2_bn256(input_scalar);
        let elapsed = start.elapsed();

        (output, proof, elapsed)
    };

    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();

    let (is_valid, verification_time) = {

        let start = Instant::now();
        let is_valid = verify_poseidon2_bn256(proof);
        let elapsed = start.elapsed();

        (is_valid, elapsed)
    };
    
    assert!(is_valid);
    println!("output: {:?}", hex::encode(&output));
    println!("guest build time: {:?}", guest_build_time);
    println!("proving time: {:?}", proving_time);
    println!("verification time: {:?}", verification_time);
    println!("proof size: {:?} bytes", proof_bytes.len());
}