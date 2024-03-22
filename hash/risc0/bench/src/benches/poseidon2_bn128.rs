use benchmark_methods::{
    POSEIDON2_BN128_ELF, POSEIDON2_BN128_ID
};
use risc0_zkvm::{ExecutorImpl, ExecutorEnv};
use zkhash::{fields::{bn256::FpBN256, utils::random_scalar}/* , poseidon2::poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS*/};
use std::time::Instant;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};


pub fn poseidon2_bn128_bench(mt_depth: usize) {
    
    type Scalar = FpBN256;

    let mut input_scalar: Vec<Vec<u8>> = Vec::new();
    let number_of_leaves: u32 = 1 << mt_depth;
    for _ in 0..number_of_leaves {
        let mut uncompressed_bytes = Vec::new();
        let a: Scalar = random_scalar();
        a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();
        input_scalar.push(uncompressed_bytes);
    }

    let env = ExecutorEnv::builder().write(&input_scalar).unwrap().build().unwrap();

    let mut exec = ExecutorImpl::from_elf(env, &POSEIDON2_BN128_ELF).unwrap();
    let session = exec.run().unwrap();

    // Produce a receipt by proving the specified ELF binary.
    let (receipt, proving_time) = {

        let start = Instant::now();
        let receipt = session.prove().unwrap();
        let elapsed = start.elapsed();

        (receipt, elapsed)
    };

    //proof size
    let proof_bytes = receipt
        .inner
        .composite()
        .unwrap()
        .segments
        .iter()
        .fold(0, |acc, segment| acc + segment.get_seal_bytes().len())
        as u32;
    
    //number of cycles
    let cycles = session.total_cycles;

    // verify your receipt
    let verification_time = {

        let start = Instant::now(); 
        receipt.verify(POSEIDON2_BN128_ID).unwrap();
        let elapsed = start.elapsed();

        elapsed
    };
    
    let output: Vec<u8> = receipt.journal.decode().unwrap();
    let hash_final = Scalar::deserialize_uncompressed(&*output).unwrap();

    eprintln!("Proving Time: {:?}", proving_time);
    eprintln!("Verification time: {:?}", verification_time);
    eprintln!("Proof Bytes: {:?}", proof_bytes);
    eprintln!("Total Cycles: {:?}", cycles);
    eprintln!("Hash: {:?}", hash_final);

}