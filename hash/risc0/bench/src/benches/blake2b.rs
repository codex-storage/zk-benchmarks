use methods::{
    BLAKE2B_ELF, BLAKE2B_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{ sha};
// use rand::RngCore;
use std::time::Instant;

pub fn blake2b_bench(input: Vec<u8>) {
   
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, BLAKE2B_ELF).unwrap();

    // For example:
    let _output: sha::Digest = receipt.journal.decode().unwrap();

    let elapsed_time1 = start_time.elapsed();
    // verify your receipt
    receipt.verify(BLAKE2B_ID).unwrap();

    let elapsed_time2 = start_time.elapsed();

    eprintln!("Total time: {:?}", elapsed_time2);
    eprintln!("verification time: {:?}", elapsed_time2 -  elapsed_time1);

}