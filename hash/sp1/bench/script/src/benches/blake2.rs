
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use rand::Rng;
use hex::encode;

const BLAKE2_ELF: &[u8] = include_bytes!("../../../blake2/elf/riscv32im-succinct-zkvm-elf");

pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<u8>()).collect()
}


pub fn blake2_benchmark(size: usize) {

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let data = generate_random_bytes(size);

    stdin.write(&data);

    let t0 = std::time::Instant::now();
    let mut proof = SP1Prover::prove(BLAKE2_ELF, stdin).expect("proving failed");
    let t1 = t0.elapsed();
    // Read output.
    let hash_bytes = proof.stdout.read::<[u8;64]>();
    let hash = encode(hash_bytes);
    println!("hash: {}", hash);

    // Verify proof.
    let t2 = std::time::Instant::now();
    SP1Verifier::verify(BLAKE2_ELF, &proof).expect("verification failed");
    let t3 = t2.elapsed();

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!");
    println!("Proof Generation Time: {:?}", t1);
    println!("Proof verification Time: {:?}", t3);
}
