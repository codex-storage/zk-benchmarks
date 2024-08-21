use std::time::Instant;
use ark_serialize::CanonicalSerialize;
use rand::Rng;

extern crate alloc;
use alloc::vec::Vec;

pub fn poseidon2_babybear_bench(mt_depth: usize) {

    let t = (1 << mt_depth) * 8;
    let mut input: Vec<u8> = Vec::new();

    for _ in 0..t {
        
        let mut rng = rand::thread_rng();
        let random_u32: u8 = rng.gen();
        input.push(random_u32);
    }

    let (prove_poseidon2_babybear, verify_poseidon2_babybear, guest_build_time) = {

        let start = Instant::now();
        let (prove, verify) = guest::build_poseidon2_babybear();
        let elapsed = start.elapsed();

        (prove, verify, elapsed)
    };

    let (output, proof, proving_time) = {

        let start = Instant::now();
        let (output, proof) = prove_poseidon2_babybear(input.as_slice());
        let elapsed = start.elapsed();

        (output, proof, elapsed)
    };

    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();

    let (is_valid, verification_time) = {

        let start = Instant::now();
        let is_valid = verify_poseidon2_babybear(proof);
        let elapsed = start.elapsed();

        (is_valid, elapsed)
    };

    assert!(is_valid);
    println!("output: {:?}", hex::encode(output));
    println!("guest build time: {:?}", guest_build_time);
    println!("proving time: {:?}", proving_time);
    println!("verification time: {:?}", verification_time);
    println!("proof size: {:?} bytes", proof_bytes.len());
}