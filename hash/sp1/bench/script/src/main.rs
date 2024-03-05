pub mod benches{
    pub mod keccak;
    pub mod sha256;
}
use crate::benches::keccak::keccak_benchmark;
use crate::benches::sha256::sha256_benchmark;
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

    match bench_type.as_str() {

        "sha256" => {
            println!("Running sha256: ");
            let _ = sha256_benchmark(size);
        }

        "keccak" => {
            println!("Running keccak benchmark: ");
            let _ = keccak_benchmark(size);
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }
}
