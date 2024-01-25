Benchmarking inside risc'0 zkvm
--------------------------------

- `external` folder contains risc0 as a git submodule.
- `bench` folder contains the benchmarking for different hash functions. Go to the `bench/README.md` for more details.
- `inner_proof` folder contains methods for generating the Receipt for sha256 which is being used as an inner proof in `composition`
- `composition` folder contains methods of proof composition which uses `inner_proof`.
