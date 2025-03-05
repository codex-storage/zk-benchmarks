
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use zkhash::fields::{bn256::FpBN256, utils::random_scalar};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

const POSEIDON2_ELF: &[u8] = include_bytes!("../../../poseidon2/elf/riscv32im-succinct-zkvm-elf");

pub fn poseidon2_benchmark(mt_depth: usize) {

    // Generate proof.
    let mut stdin = SP1Stdin::new();

    type Scalar = FpBN256;

    // generating data and serialize
    let mut input_scalar: Vec<Vec<u8>> = Vec::new();
    let number_of_leaves: u32 = 1 << mt_depth;
    for _ in 0..number_of_leaves {
        let mut uncompressed_bytes = Vec::new();
        let a: Scalar = random_scalar();
        a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();
        input_scalar.push(uncompressed_bytes);
    }

    stdin.write(&input_scalar);

    let t0 = std::time::Instant::now();
    let mut proof = SP1Prover::prove(POSEIDON2_ELF, stdin).expect("proving failed");
    let t1 = t0.elapsed();
    
    // Read output.
    let hash_bytes = proof.stdout.read::<Vec<u8>>();
    let hash_final = Scalar::deserialize_uncompressed(&*hash_bytes).unwrap();

    println!("hash: {}", hash_final);

    // Verify proof.
    let t2 = std::time::Instant::now();
    SP1Verifier::verify(POSEIDON2_ELF, &proof).expect("verification failed");
    let t3 = t2.elapsed();

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!");
    println!("Proof Generation Time: {:?}", t1);
    println!("Proof verification Time: {:?}", t3);
}
