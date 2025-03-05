#![allow(non_snake_case)]
use benchmark_methods::{
    POSEIDON2_BABYBEAR_ELF, 
    POSEIDON2_BABYBEAR_ID
};
use risc0_zkvm::{
    ExecutorImpl, 
    ExecutorEnv
};
use zkhash::fields::{
        babybear::FpBabyBear, 
        utils::random_scalar
    };
use std::time::Instant;
use ark_serialize::{
    CanonicalSerialize, 
    CanonicalDeserialize
};


pub fn poseidon2_babybear_bench(mt_depth: usize) {
    
    type Scalar = FpBabyBear;

    let t = (1 << mt_depth) * 8;
    let mut input_scalar: Vec<Vec<u8>> = Vec::new();

    for _ in 0..t {
        let mut uncompressed_bytes = Vec::new();
        let a: Scalar = random_scalar();
        a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();
        input_scalar.push(uncompressed_bytes);
    }

    let env = ExecutorEnv::builder().write(&input_scalar).unwrap().build().unwrap();

    let mut exec = ExecutorImpl::from_elf(env, &POSEIDON2_BABYBEAR_ELF).unwrap();
    let session = exec.run().unwrap();

    // Produce a receipt by proving the specified ELF binary.
    let (receipt, proving_time) = {

        let start = Instant::now();
        let receipt = session.prove().unwrap();
        let elapsed = start.elapsed();

        (receipt, elapsed)
    };

    //proof size
    let proof_bytes = receipt
        .inner
        .composite()
        .unwrap()
        .segments
        .iter()
        .fold(0, |acc, segment| acc + segment.get_seal_bytes().len())
        as u32;
    
    //number of cycles
    let cycles = session.total_cycles;

    // verify your receipt
    let verification_time = {

    let start = Instant::now(); 
    receipt.verify(POSEIDON2_BABYBEAR_ID).unwrap();
    let elapsed = start.elapsed();

    elapsed
  };

    let output: Vec<Vec<u8>> = receipt.journal.decode().unwrap();

    let mut output_deseralised: Vec<Scalar> = Vec::new();

    for i in 0..output.len() {
        output_deseralised.push(Scalar::deserialize_uncompressed(&**output.get(i).unwrap()).unwrap());
    }

    eprintln!("Proving Time: {:?}", proving_time);
    eprintln!("Verification Time: {:?}", verification_time);
    eprintln!("Proof Bytes: {:?}", proof_bytes);
    eprintln!("Total Cycles: {:?}", cycles);
    eprintln!("hash: {:?}", output_deseralised);

}