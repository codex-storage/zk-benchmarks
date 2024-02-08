use std::process;
mod bench;
use bench::poseidon::poseidon_bench;

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

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }

    println!("All Done!");
    
}