
use host::sha_bench;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::RngCore;
fn sha256_benchmarks(c: &mut Criterion) {
    c.bench_function("Benchmarking sha256 on 1KB of random data:", |b| {
        //generating 1kb of random data in a vector of u8
        let mut data = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

        // println!("{:?}", input);
        b.iter(|| {
            sha_bench(input.clone());
        });
    });

    c.bench_function("Benchmarking sha256 on 2KB of random data:", |b| {
        //generating 2kb of random data in a vector of u8
        let mut data = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

        // println!("{:?}", input);
        b.iter(|| {
            sha_bench(input.clone());

        });
    });

    c.bench_function("Benchmarking sha256 on 10KB of random data:", |b| {
        //generating 10kb of random data in a vector of u8
        let mut data = [0u8; 1280];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

        b.iter(|| {
            sha_bench(input.clone());
        });
    });
}

// fn sha256_benchmarks_2(c: &mut Criterion) {
//     c.bench_function("Benchmarking sha256 on 1KB of random data:", |b| {
//         //generating 1kb of random data in a vector of u8
//         let mut data = [0u8; 64];
//         rand::thread_rng().fill_bytes(&mut data);
//         let input: Vec<u8> = data.to_vec();

//         // println!("{:?}", input);
//         b.iter(|| {
//             sha_bench(input.clone());
//         });
//     });

//     // c.bench_function("Benchmarking sha256 on 2KB of random data:", |b| {
//     //     //generating 2kb of random data in a vector of u8
//     //     let mut data = [0u8; 128];
//     //     rand::thread_rng().fill_bytes(&mut data);
//     //     let input: Vec<u8> = data.to_vec();

//     //     // println!("{:?}", input);
//     //     b.iter(|| {
//     //         sha_bench(input.clone());

//     //     });
//     // });

//     // c.bench_function("Benchmarking sha256 on 10KB of random data:", |b| {
//     //     //generating 10kb of random data in a vector of u8
//     //     let mut data = [0u8; 1280];
//     //     rand::thread_rng().fill_bytes(&mut data);
//     //     let input: Vec<u8> = data.to_vec();

//     //     b.iter(|| {
//     //         sha_bench(input.clone());
//     //     });
//     // });

// }

criterion_group!(
    name = benches;

    // Setting the sample size to 10 for quick benchmarks
    // Becuase running default number of samples(100) takes a lot of time
    config = Criterion::default().sample_size(10); 
    targets = sha256_benchmarks //, sha256_benchmarks_2
);

// criterion_group!(benches, sha256_bench);
criterion_main!(benches);
