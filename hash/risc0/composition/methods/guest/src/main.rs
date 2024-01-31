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

use inner_proof_methods::INNER_PROOF_METHOD_ID;
use risc0_zkvm::{guest::env, serde};
use risc0_zkvm::sha;
fn main() {
    let c1 = env::cycle_count();
    let hash: sha::Digest = env::read();
    let hash2: sha::Digest = env::read();
    let c2 = env::cycle_count();

    env::verify(INNER_PROOF_METHOD_ID, &serde::to_vec(&hash).unwrap()).unwrap();
    env::verify(INNER_PROOF_METHOD_ID, &serde::to_vec(&hash2).unwrap()).unwrap();
    let c3 = env::cycle_count();
    eprintln!("cycles for input builder: {:?}", c2 - c1);
    eprintln!("cycles for inner_proof verification inside guest: {:?}", c3 - c2);
    env::commit(&hash);
}

