
Benchmarking framework
----------------------

The role of this program is to build, setup, and run benchmarks with various
parameter settings, collecting the timing results together.

A benchmark is defined by the following shell scripts:

- `build.sh` - build the code
- `setup.sh` - run some additional setup, for example Groth16 circuit-specific setup
- `witness.sh` - run witness generation for SNARKs (separate from `setup` because we may want to measure it)
- `run.sh` - run the benchmark itself

These are run in this order, and results are cached unless explicitely requested.
All except `run.sh` are optional.

Recommended organization is to put all build artifacts into a `build` subdirectory.

Benchmarks can be parameterized using environment variables. By convention, we
start the names of these environment variables with the `ZKBENCH_` prefix.

An additional file `bench.cfg` specifies the configuration and parameter ranges.
Example file:

    name:   "Poseidon2 Groth16 benchmarks"
    author: Xenon Y. Zorro
    timeout: 300
    rerun_from: build 
    params:
      [ PROVER:     [ snarkjs, rapidsnark ]
      , INPUT_SIZE: [ 256, 512, 1024, 2048 ]
      , WHICH:      [ hash_sponge, hash_sponge_rate2, hash_merkle ]
      ]
    tags: Groth16, Poseidon2, $PROVER
    comments:
      Here you can even write 
      multiline comments
      for convenience

Note: in case of an arithmetic circuit, every step of the build process must be 
rerun if the circuit changes, and the circuit depends on the input size...
The `rerun_from` parameter allows to set this. Normally you want it te be `run`
(only rerun the `run.sh` script), but in case of Groth16 you want that to be `build`.

`timeout` (in seconds) sets the maximum target time we should spend on this specific
benchmark. If the initial run is fast enough, we will rerun it up to 10 times 
and everage them to get a less noisy result.

`params` corresponds to the possible values of the corresponding environment 
variables (in this example, `ZKBENCH_PROVER`, etc)

`tags` are used to select relevant subsets of the benchmarks (as we expect to
have a lots of them, with lots of parameter settings).
