use criterion::Criterion;
// use clap::Parser;
// use std::env;
mod benchmarks;

fn main() {
    let mut criterion: criterion::Criterion<_>  = (Criterion::default().sample_size(10)).configure_from_args();
    match std::env::args().skip(1).next() {
        Some(arg) => {
            match arg.as_str() {
                "1" => benchmarks::sha256_benchmarks::sha256_benchmarks_1kb(&mut criterion),
                "2" => benchmarks::sha256_benchmarks::sha256_benchmarks_2kb(&mut criterion),
                "10" => benchmarks::sha256_benchmarks::sha256_benchmarks_10kb(&mut criterion),
                _ => eprintln!("Invalid benchmark argument: {}", arg),
            }
        }
       None => {eprintln!("No benchmark")}
    }
    
    criterion::Criterion::default().configure_from_args().final_summary();
 }


// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[clap(long, short)]
//     run_benchmark_function_one: bool,
//     #[clap(long, short)]
//     run_benchmark_function_two: bool,
// }

// fn main(){
//     let args: Args = Args::parse();
//     let mut criterion: criterion::Criterion<_>  = (Criterion::default().sample_size(10)).configure_from_args();

//     if args.run_benchmark_function_one {
//         benchmarks::sha256_benchmarks::sha256_benchmarks1(&mut criterion);
//     }
//     if args.run_benchmark_function_two {
//         benchmarks::sha256_benchmarks22::sha256_benchmarks123(&mut criterion);
//     }
//     criterion::Criterion::default().configure_from_args().final_summary();
// }
