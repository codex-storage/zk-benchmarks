Benchmarking inside sp1 ZKVM
--------------------------------

- The `sp1/benches` contains the following hash `program`(the source code that will be proven inside the zkVM): sha256, keccak, blake2, blake3, and poseidon2 ober BN256.
- `script` folder contains the benchmarking code that contains proof generation and verification code for each hash program.
- The `build.sh` script builds the whole code.
- `run.sh` and `run_tree.sh` runs the benchmark. (`run.sh` for sha256, keccak, blake2, blake3 and `run_tree.sh` for poseidon2 over BN256)
- Benchmarks can be parameterized using environment variables. By convention, we start the names of these environment variables with the `ZKBENCH_` prefix.
- By default the `run.sh` will run the sha256 benchmark over 1KB of data. other hashes can be run by settig the environment variables accordingly.
- Additional files `bench.cfg` and `bench_tree.cfg` specifies the configurations and parameters.
