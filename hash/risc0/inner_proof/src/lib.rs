use inner_proof_methods::INNER_PROOF_METHOD_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use risc0_zkvm::{ sha};
use rand::Rng;


fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

pub fn sha_bench(size: usize) -> (Receipt, sha::Digest) {

    let input = generate_bytes(size);

    let env = ExecutorEnv::builder()
        .write_slice(&input)
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, INNER_PROOF_METHOD_ELF).unwrap();

    let _output: sha::Digest = receipt.journal.decode().expect(
        "cannot deserialise",
    );

    (receipt, _output)
}