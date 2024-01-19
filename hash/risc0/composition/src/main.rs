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
fn main() {
    
    let (hash_receipt, hash) = sha_bench(32);

    let env = ExecutorEnv::builder()
        // add_assumption makes the receipt to be verified available to the prover.
        .add_assumption(hash_receipt)
        .write(&hash)
        .unwrap()
        .build()
        .unwrap();

    let receipt = default_prover().prove(env, COMPOSITION_METHOD_ELF).unwrap();

    receipt.verify(COMPOSITION_METHOD_ID).unwrap();

    let hash: sha::Digest = receipt.journal.decode().unwrap();
    eprintln!("hash: {:?}", hash);

}
