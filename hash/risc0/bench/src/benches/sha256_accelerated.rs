use benchmark_methods::{
    SHA256_ACCELERATED_ELF, SHA256_ACCELERATED_ID
};
use risc0_zkvm::{ExecutorEnv, ExecutorImpl};
use std::time::Instant;
use hex::encode;
pub fn sha_accelerated_bench(input: Vec<u8>) {
   
    let env = ExecutorEnv::builder()
      .write_slice(&input)
      .build()
      .unwrap();
    
    let mut exec = ExecutorImpl::from_elf(env, &SHA256_ACCELERATED_ELF).unwrap();
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
    receipt.verify(SHA256_ACCELERATED_ID).unwrap();
    let elapsed = start.elapsed();

    elapsed
  };

    let _output: [u8;32] = receipt.journal.decode().unwrap();
    let hash = encode(_output);
    eprintln!("Proving Time: {:?}", proving_time);
    eprintln!("Verification Time: {:?}", verification_time);
    eprintln!("Proof Bytes: {:?}", proof_bytes);
    eprintln!("Total Cycles: {:?}", cycles);
    eprintln!("Hash: {:?}", hash);
}