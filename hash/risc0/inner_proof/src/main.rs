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
use inner_proof::sha_bench;
use inner_proof_methods::INNER_PROOF_METHOD_ID;
use std::process;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Wrong number of arguments! The program expects two arguments: <number_of_composition> and <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let data_size = args[1].parse::<usize>().unwrap();

    let (receipt, _output) = sha_bench(data_size.try_into().unwrap());

    // Verify receipt, panic if it's wrong
    receipt.verify(INNER_PROOF_METHOD_ID).expect(
        "cannot verify",
    );

    eprintln!("hash: {:?}", _output);
}
