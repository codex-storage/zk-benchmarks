#![allow(non_snake_case)]
use benchmark_methods::{
    POSEIDON2_BABYBEAR_NATIVE_ELF, 
    POSEIDON2_BABYBEAR_NATIVE_ID
};
use risc0_zkvm::{
    ExecutorImpl, 
    ExecutorEnv,
    sha::Digest
};
use std::time::Instant;
use rand::Rng;

pub fn poseidon2_babybear_native_bench(mt_depth: usize) {
    
    let t = (1 << mt_depth) * 8;
    let mut input: Vec<u32> = Vec::new();

    for _ in 0..t {
        
        let mut rng = rand::thread_rng();
        let random_u32: u32 = rng.gen();
        input.push(random_u32);
    }

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    let mut exec = ExecutorImpl::from_elf(env, &POSEIDON2_BABYBEAR_NATIVE_ELF).unwrap();
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
      receipt.verify(POSEIDON2_BABYBEAR_NATIVE_ID).unwrap();
      let elapsed = start.elapsed();
  
      elapsed
    };
  
    let output: Box<Digest> = receipt.journal.decode().unwrap();
    
    eprintln!("Proving Time: {:?}", proving_time);
    eprintln!("Verification time: {:?}", verification_time);
    eprintln!("Proof Bytes: {:?}", proof_bytes);
    eprintln!("Total Cycles: {:?}", cycles);
    eprintln!("Hash: {:?}", output);
}