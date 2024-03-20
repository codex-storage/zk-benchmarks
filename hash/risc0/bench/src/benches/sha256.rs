use benchmark_methods::{
    SHA256_ELF, SHA256_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{ sha};
use std::time::Instant;

pub fn sha_bench(input: Vec<u8>) {
   
    let env = ExecutorEnv::builder()
      .write_slice(&input)
      .build()
      .unwrap();
    
    // Obtain the default prover.
    let prover = default_prover();
    eprintln!("\n------risc0_zkvm sha hashing------\n");

    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, SHA256_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // verify your receipt
    receipt.verify(SHA256_ID).unwrap();

    let elapsed_time2 = start_time.elapsed();

    let _output: sha::Digest = receipt.journal.decode().unwrap();

    eprintln!("Total time: {:?}", elapsed_time2);
    eprintln!("Verification time: {:?}", elapsed_time2 - elapsed_time);

    eprintln!("Hash: {:?}", _output);

}