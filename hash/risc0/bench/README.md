Benchmarking different hashes inside risc'0 zkvm
-----------------------------------------------

The benchmark is defined by the following shell scripts:

- `build.sh` - build the code.

- `run.sh` and `run2.sh` - run the benchmark itself (`run.sh` for sha256, keccak, blake2b, blake3 and `run2.sh` for poseidon2 over bn128 and babybear)

Benchmarks can be parameterized using environment variables. By convention, we start the names of these environment variables with the `ZKBENCH_` prefix.

Additional files `bench.cfg` and `bench_tree.cfg` specifies the configurations and parameters.
