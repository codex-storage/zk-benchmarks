pub mod benches{
    pub mod keccak;
    pub mod sha256;
    pub mod blake2;
    pub mod blake3;
    pub mod poseidon2;
}

use crate::benches::{
    keccak::keccak_benchmark,
    sha256::sha256_benchmark,
    blake2::blake2_benchmark,
    blake3::blake3_benchmark,
    poseidon2::poseidon2_benchmark
};
use sp1_core::utils;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Wrong number of arguments! The program expects two arguments: <hash_type> and <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let bench_type = &args[1];
    let size = args[2].parse::<usize>().unwrap();

    // Setup a tracer for logging.
    utils::setup_logger();

    match bench_type.as_str() {

        "sha256" => {
            println!("Running sha256: ");
            let _ = sha256_benchmark(size);
        }

        "keccak" => {
            println!("Running keccak benchmark: ");
            let _ = keccak_benchmark(size);
        }

        "blake2" => {
            println!("Running blake2 benchmark: ");
            let _ = blake2_benchmark(size);
        }

        "blake3" => {
            println!("Running blake3 benchmark: ");
            let _ = blake3_benchmark(size);
        }

        "poseidon2" => {
            println!("Running poseidon2 benchmark: ");
            let _ = poseidon2_benchmark(size);
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }
}
