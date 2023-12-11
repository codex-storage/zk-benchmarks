mod benches;
use benches::sha256::sha_bench;
use benches::keccak::keccak_bench;
use rand::Rng;


fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}



fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut flag = 0;

    let hash_type = &args[1];
    let size_kb = args[2].parse::<usize>().unwrap();

    eprintln!("data size(bytes): {:?}", size_kb);
    let input = generate_bytes(size_kb);

    if hash_type == "all" || hash_type == "sha256" {
        println!("SHA256 Benchmarking: ");
        sha_bench(input.clone());
        println!("");
        flag = 1;
    }
    
    if hash_type == "all" || hash_type == "keccak" {
        println!("KECCAK Benchmarking: ");
        keccak_bench(input.clone());
        println!("");
        flag = 1;
    }

    if flag == 0 {
        println!("Wrong Benchmarking Name");
    }
    println!("All Done!");
    
}
