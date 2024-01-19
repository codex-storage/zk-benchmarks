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

use inner_proof_methods::INNER_PROOF_METHOD_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use risc0_zkvm::{ sha};
use rand::Rng;


pub fn generate_bytes(size: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
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