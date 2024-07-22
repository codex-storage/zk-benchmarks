
use std::process;
mod bench{
    pub mod poseidon;
    pub mod sha256{
        pub mod constants;
        pub mod sigma;
        pub mod sha;
        pub mod xor3;
        pub mod maj;
        pub mod ch;

    }

    pub mod  keccak256{
        pub mod keccak;
        pub mod keccak_polygon;
    }
}

mod arithmetic {
    pub mod binary_arithmetic;
    pub mod u32_arithmetic;
    pub mod u64_arithmetic;
}

use bench::poseidon::poseidon_bench;
use bench::keccak256::keccak::keccak_bench;
use bench::sha256::sha::sha256_bench;
use bench::keccak256::keccak_polygon::keccak_polygon_bench;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Wrong number of arguments! The program expects two arguments: <hash_type> and <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let hash_type = &args[1];
    let size = args[2].parse::<usize>().unwrap();

    match hash_type.as_str() {

        "poseidon" => {
            println!("Running Poseidon: ");
            eprintln!("Tree Depth: {:?}", size);
            let _ = poseidon_bench(size);
        }

        "keccak" => {
            println!("Running keccak: ");
            eprintln!("input size: {:?}", size);
            let _ = keccak_bench(size);
        }

        "keccak-polygon" => {
            println!("Running keccak of plolygon zk_evm: ");
            eprintln!("number of permutation: {:?}", size);
            let _ = keccak_polygon_bench(size);
        }


        "sha256" => {
            println!("Running sha256: ");
            let _ = sha256_bench(size);
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }

    println!("All Done!");
    
}