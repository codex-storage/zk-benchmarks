
use host::sha_bench;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::RngCore;
fn sha256_1kb(c: &mut Criterion) {
    c.bench_function("sha256_bench", |b| {
        //generating 1kb of random data in a vector of u8
        let mut data = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

        // println!("{:?}", input);
        b.iter(|| {
            sha_bench(input.clone());
            // hasher.update(data);
            // black_box(hasher.finalize());
        });
    });

    c.bench_function("sha256_bench", |b| {
        //generating 1kb of random data in a vector of u8
        let mut data = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

        // println!("{:?}", input);
        b.iter(|| {
            sha_bench(input.clone());
            // hasher.update(data);
            // black_box(hasher.finalize());
        });
    });
    
}

criterion_group!(
    name = benches;

    // Setting the sample size to 10 for quick benchmarks
    // Becuase running default number of samples(100) takes a lot of time
    config = Criterion::default().sample_size(10); 
    targets = sha256_1kb
);

// criterion_group!(benches, sha256_bench);
criterion_main!(benches);
