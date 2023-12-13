mod benches;
use benches::{
    sha256::sha_bench,
    keccak::keccak_bench,
    blake2b::blake2b_bench,
    blake3::blake3_bench,
};
use rand::Rng;


fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}



fn main() {
    let args: Vec<String> = std::env::args().collect();

    // let mut flag = 0;

    let hash_type = &args[1];
    let size_kb = args[2].parse::<usize>().unwrap();

    eprintln!("data size(bytes): {:?}", size_kb);
    let input = generate_bytes(size_kb);

    match hash_type.as_str() {
        "sha256" => {
            println!("SHA256 Benchmarking: ");
            sha_bench(input.clone());
        }
        "keccak" => {
            println!("KECCAK Benchmarking: ");
            keccak_bench(input.clone());
        }

        "blake2b" => {
            println!("Blake2b Benchmarking: ");
            blake2b_bench(input.clone());
        }

        "blake3" => {
            println!("Blake3 Benchmarking: ");
            blake3_bench(input.clone());
        }
        _ => {
            println!("Wrong Benchmark Name!");
        }
    }
    
    println!("All Done!");
    
}
