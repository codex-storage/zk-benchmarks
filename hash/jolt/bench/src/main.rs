use rand::Rng;

fn generate_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

pub fn main() {
    let (prove_sha2, verify_sha2) = guest::build_sha2();
    let (prove_sha3, verify_sha3) = guest::build_sha3();

    // random data
    let binding = generate_bytes(1024);
    let input = binding.as_slice();

    let (output, proof) = prove_sha2(input);
    let is_valid = verify_sha2(proof);

    println!("sha2 output: {:?}", output);
    println!("sha2 valid: {}", is_valid);

    let (output, proof) = prove_sha3(input);
    let is_valid = verify_sha3(proof);

    println!("sha3 output: {:?}", output);
    println!("sha3 valid: {}", is_valid);
}
