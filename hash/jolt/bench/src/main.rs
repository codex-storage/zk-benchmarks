use rand::Rng;
use std::process;

mod benches;
use benches::{
    sha2::sha2_bench,
    sha3::sha3_bench,
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
                println!("SHA256 Benchmarking: ");
                eprintln!("data size(bytes): {:?}", size);
                let input = generate_bytes(size);
                sha2_bench(input.clone());
            }
            
            "keccak" => {
                println!("KECCAK Benchmarking: ");
                eprintln!("data size(bytes): {:?}", size);
                let input = generate_bytes(size);
                sha3_bench(input.clone());
            }
    
            _ => {
                println!("Wrong Benchmark Name!");
            }
        }
    
        println!("All Done!");
    
}
