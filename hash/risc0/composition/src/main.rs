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
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::sha;
use std::time::Instant;
use std::process;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Wrong number of arguments! The program expects one arguments: <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let data_size = args[1].parse::<usize>().unwrap();
    
    let t0 = Instant::now();

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

    let t1 = t0.elapsed();

    let receipt = default_prover().prove(env, COMPOSITION_METHOD_ELF).unwrap();

    let t2 = t0.elapsed();

    receipt.verify(COMPOSITION_METHOD_ID).unwrap();

    let t3 = t0.elapsed();

    let hash: sha::Digest = receipt.journal.decode().unwrap();
    eprintln!("hash: {:?}", hash);
    eprintln!("ExecutorEnv Builder time: {:?}", t1);
    eprintln!("Proof generation + receiving receipt time: {:?}", t2 - t1);
    eprintln!("Verification time: {:?}", t3 - t2);
}
