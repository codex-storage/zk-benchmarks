use inner_proof_methods::INNER_PROOF_METHOD_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use risc0_zkvm::{ sha};
use rand::Rng;


pub fn generate_bytes(size: u32) -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..(size/2)).map(|_| rng.gen()).collect();

    let random_string: String = random_bytes
        .iter()
        .map(|byte| format!("{:02X}", byte)) // Convert each byte to a two-digit hexadecimal string
        .collect();

    // eprintln!("bytes: {:?}", random_string.as_bytes().len());
    random_string

}

pub fn sha_bench(size: u32) -> (Receipt, sha::Digest) {

    let input = generate_bytes(size);

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
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