use benchmark_methods::{
    SHA256_ACCELERATED_ELF, SHA256_ACCELERATED_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::time::Instant;
use hex::encode;
pub fn sha_accelerated_bench(input: Vec<u8>) {
   
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();
    eprintln!("\n------RustCrypto sha hashing(accelerated)------\n");
    // Obtain the default prover.
    let prover = default_prover();

    let start_time = Instant::now();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, SHA256_ACCELERATED_ELF).unwrap();
    let elapsed_time = start_time.elapsed();

    // verify your receipt
    receipt.verify(SHA256_ACCELERATED_ID).unwrap();

    let elapsed_time2 = start_time.elapsed();

    let _output: [u8;32] = receipt.journal.decode().unwrap();
    let hash = encode(_output);
    eprintln!("Total time: {:?}", elapsed_time2);
    eprintln!("Verification time: {:?}", elapsed_time2 - elapsed_time);

    eprintln!("Hash: {:?}", hash);
}