mod benches;
use benches::{
    sha256::sha_bench,
    sha256_accelerated::sha_accelerated_bench,
    keccak::keccak_bench,
    blake2b::blake2b_bench,
    blake3::blake3_bench,
    poseidon2_bn128::poseidon2_bn128_bench,
    poseidon2_babybear::poseidon2_babybear_bench,
    poseidon2_babybear_native::poseidon2_babybear_native_bench,
};
use rand::Rng;
use std::process;

fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}



fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Wrong number of arguments! The program expects two arguments: <hash_type> and <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let hash_type = &args[1];
    let size = args[2].parse::<usize>().unwrap();
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    match hash_type.as_str() {
        "sha256" => {
            println!("SHA256 Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size);
            let input = generate_bytes(size);
            sha_bench(input.clone());
        }
        "sha256_accelerated" => {
            println!("Accelerated SHA256(Patched rustCrypto) Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size);
            let input = generate_bytes(size);
            sha_accelerated_bench(input.clone());
        }
        "keccak" => {
            println!("KECCAK Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size);
            let input = generate_bytes(size);
            keccak_bench(input.clone());
        }

        "blake2b" => {
            println!("Blake2b Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size);
            let input = generate_bytes(size);
            blake2b_bench(input.clone());
        }

        "blake3" => {
            println!("Blake3 Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size);
            let input = generate_bytes(size);
            blake3_bench(input.clone());
        }

        "poseidon2_bn128" => {
            println!("Poseidon2 Benchmarking on the BN128 field: ");
            eprintln!("Tree Depth: {:?}", size);
            poseidon2_bn128_bench(size);
        }

        "poseidon2_babybear" => {
            println!("Poseidon2 Benchmarking on the BabyBear field: ");
            eprintln!("Tree Depth: {:?}", size);
            eprintln!("number of inputs {:?}",  (1 << size) * 8);
            poseidon2_babybear_bench(size);
        }

        "poseidon2_babybear_native" => {
            println!("Poseidon2 Benchmarking on the risc0's native BabyBear field: ");
            eprintln!("Tree Depth: {:?}", size);
            eprintln!("number of inputs {:?}",  (1 << size) * 8);
            poseidon2_babybear_native_bench(size);
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }

    println!("All Done!");
    
}
