mod benches;
use benches::{
    sha256::sha_bench,
    keccak::keccak_bench,
    blake2b::blake2b_bench,
    blake3::blake3_bench,
    poseidon2_bn128::poseidon2_bn128_bench,
    poseidon2_babybear::poseidon2_babybear_bench,
    poseidon2_babybear_native::poseidon2_babybear_native_bench,
};
use rand::Rng;

fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}



fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let hash_type = &args[1];
    let size_kb = args[2].parse::<usize>().unwrap();

    match hash_type.as_str() {
        "sha256" => {
            println!("SHA256 Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size_kb);
            let input = generate_bytes(size_kb);
            sha_bench(input.clone());
        }
        "keccak" => {
            println!("KECCAK Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size_kb);
            let input = generate_bytes(size_kb);
            keccak_bench(input.clone());
        }

        "blake2b" => {
            println!("Blake2b Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size_kb);
            let input = generate_bytes(size_kb);
            blake2b_bench(input.clone());
        }

        "blake3" => {
            println!("Blake3 Benchmarking: ");
            eprintln!("data size(bytes): {:?}", size_kb);
            let input = generate_bytes(size_kb);
            blake3_bench(input.clone());
        }

        "poseidon2_bn128" => {
            println!("Poseidon2 Benchmarking on the BN128 field: ");
            eprintln!("Tree Depth: {:?}", size_kb);
            poseidon2_bn128_bench(size_kb);
        }

        "poseidon2_babybear" => {
            println!("Poseidon2 Benchmarking on the BabyBear field: ");
            eprintln!("Tree Depth: {:?}", size_kb);
            eprintln!("number of inputs {:?}",  (1 << size_kb) * 8);
            poseidon2_babybear_bench(size_kb);
        }

        "poseidon2_babybear_native" => {
            println!("Poseidon2 Benchmarking on the BabyBear field: ");
            eprintln!("Tree Depth: {:?}", size_kb);
            eprintln!("number of inputs {:?}",  (1 << size_kb) * 8);
            poseidon2_babybear_native_bench(size_kb);
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }

    println!("All Done!");
    
}
