use methods::{
    SHA256_ELF, SHA256_ID, SHA256_ACCELERATED_ELF, SHA256_ACCELERATED_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{ sha};
// use sha2;
// use rand::RngCore;
use std::time::Instant;

pub fn sha_bench(input: Vec<u8>) {
   

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();
    eprintln!("------risc0_zkvm sha hashing------");
    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, SHA256_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // For example:
    let _output: sha::Digest = receipt.journal.decode().unwrap();

    // verify your receipt
    receipt.verify(SHA256_ID).unwrap();

    eprintln!("Total time: {:?}", elapsed_time);
    eprintln!("Hash: {:?}", _output);

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();
    eprintln!("------RustCrypto sha hashing------");
    // Obtain the default prover.
    let prover = default_prover();

    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, SHA256_ACCELERATED_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // For example:
    let _output: [u8;32] = receipt.journal.decode().unwrap();

    // verify your receipt
    receipt.verify(SHA256_ACCELERATED_ID).unwrap();

    eprintln!("Total time: {:?}", elapsed_time);
    eprintln!("Hash: {:?}", _output);
}