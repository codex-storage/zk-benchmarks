// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use composition_methods::{COMPOSITION_METHOD_ELF, COMPOSITION_METHOD_ID};
use inner_proof::sha_bench;
use risc0_zkvm::ExecutorEnv;
use risc0_zkvm::sha;
use std::time::Instant;
use std::process;
use risc0_zkvm::ExecutorImpl;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Wrong number of arguments! The program expects one arguments: <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let data_size = args[1].parse::<usize>().unwrap();
    
    // tracer added for logging info
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let (hash_receipt, hash) = sha_bench(data_size.try_into().unwrap());
    let (hash_receipt2, hash2) = sha_bench(data_size.try_into().unwrap());

    let env = ExecutorEnv::builder()
        // add_assumption makes the receipt to be verified available to the prover.
        .add_assumption(hash_receipt)
        .write(&hash)
        .unwrap()
        .add_assumption(hash_receipt2)
        .write(&hash2)
        .unwrap()
        .build()
        .unwrap();

    let mut exec = ExecutorImpl::from_elf(env, &COMPOSITION_METHOD_ELF).unwrap();
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
        receipt.verify(COMPOSITION_METHOD_ID).unwrap();
        let elapsed = start.elapsed();

        elapsed
    };

    let hash: sha::Digest = receipt.journal.decode().unwrap();
    eprintln!("Proving Time: {:?}", proving_time);
    eprintln!("Verification Time: {:?}", verification_time);
    eprintln!("Proof Bytes: {:?}", proof_bytes);
    eprintln!("Total Cycles: {:?}", cycles);
    eprintln!("Hash: {:?}", hash);
}
