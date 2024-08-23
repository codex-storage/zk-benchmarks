use rand::Rng;
use std::process;

mod benches;
use benches::{
    sha2::sha2_bench,
    sha3::sha3_bench,
    blake3::blake3_bench,
    blake2::blake2_bench,
    poseidon2_babybear::poseidon2_babybear_bench,
    poseidon2_bn256::poseidon2_bn256_bench,
};

fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

pub fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Wrong number of arguments! The program expects two arguments: <hash_type> and <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let hash_type = &args[1];
    let size = args[2].parse::<usize>().unwrap();

        match hash_type.as_str() {
            "sha256" => {
                println!("sha256 Benchmarking: ");
                eprintln!("data size(bytes): {:?}", size);
                let input = generate_bytes(size);
                sha2_bench(input.clone());
            }
            
            "keccak" => {
                println!("keccak Benchmarking: ");
                eprintln!("data size(bytes): {:?}", size);
                let input = generate_bytes(size);
                sha3_bench(input.clone());
            }

            "blake3" => {
                println!("blake3 Benchmarking: ");
                eprintln!("data size(bytes): {:?}", size);
                let input = generate_bytes(size);
                blake3_bench(input.clone());
            }

            "blake2" => {
                println!("blake2 Benchmarking: ");
                eprintln!("data size(bytes): {:?}", size);
                let input = generate_bytes(size);
                blake2_bench(input.clone());
            }

            "poseidon2_babybear" => {
                println!("poseidon2_babybear Benchmarking: ");
                eprintln!("Tree Depth: {:?}", size);
                eprintln!("number of inputs {:?}",  (1 << size) * 8);
                
                poseidon2_babybear_bench(size);
            }

            "poseidon2_bn256" => {
                println!("poseidon2_bn256 Benchmarking: ");
                eprintln!("Tree Depth: {:?}", size);
                eprintln!("number of inputs {:?}",  (1 << size));
                
                poseidon2_bn256_bench(size);
            }
    
            _ => {
                println!("Wrong Benchmark Name!");
            }
        }
    
        println!("All Done!");
    
}
