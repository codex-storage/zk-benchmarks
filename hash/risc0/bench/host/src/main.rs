use methods::{
    METHOD_ELF, METHOD_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{ sha};
// use rand::RngCore;
use rand::Rng;
use std::time::Instant;

fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

pub fn sha_bench(input: Vec<u8>) {
   
    let start_time = Instant::now();

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, METHOD_ELF).unwrap();

    // TODO: Implement code for retrieving receipt journal here.

    // For example:
    let _output: sha::Digest = receipt.journal.decode().unwrap();
   
    // verify your receipt
    receipt.verify(METHOD_ID).unwrap();

    let elapsed_time = start_time.elapsed();
    eprintln!("Total time: {:?}", elapsed_time);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    eprintln!("{:?}", &args[1]);
    // eprintln!("{:?}", &args[2]);
    let size_kb = args[1].parse::<usize>().unwrap();
    eprintln!("{:?}", size_kb);
    let input = generate_bytes(size_kb);
    sha_bench(input);
}
