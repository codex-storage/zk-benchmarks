use host::sha_bench;
use criterion::Criterion;
use rand::RngCore;

pub fn sha256_benchmarks_1kb(c: &mut Criterion) {
    c.bench_function(" sha256 on 1KB of random data:", |b| {
        //generating 1kb of random data in a vector of u8
        let mut data = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

        // println!("{:?}", input);
        b.iter(|| {
            sha_bench(input.clone());
        });
    });
}

pub fn sha256_benchmarks_2kb(c: &mut Criterion) {
    
    c.bench_function(" sha256 on 2KB of random data:", |b| {
        //generating 2kb of random data in a vector of u8
        let mut data = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

        // println!("{:?}", input);
        b.iter(|| {
            sha_bench(input.clone());

        });
    });
}

pub fn sha256_benchmarks_10kb(c: &mut Criterion) {
    
    //generating 10kb of random data in a vector of u8
    let mut data = [0u8; 1280];
        rand::thread_rng().fill_bytes(&mut data);
        let input: Vec<u8> = data.to_vec();

    c.bench_function(" sha256 on 10KB of random data:", |b| {

        b.iter(|| {
            sha_bench(input.clone());
        });
    });
}