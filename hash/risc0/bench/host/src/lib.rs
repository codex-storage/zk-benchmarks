use methods::{
    METHOD_ELF, METHOD_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::{ sha};

pub fn sha_bench(input: Vec<u8>) {
    // Build an executor environment with the input.
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, METHOD_ELF).unwrap();

    // For example:
    let _output: sha::Digest = receipt.journal.decode().unwrap();

    // verify your receipt
    receipt.verify(METHOD_ID).unwrap();
}


