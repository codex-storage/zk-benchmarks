
Benchmarking various ZK proof themed implementations 
----------------------------------------------------

We want to benchmark:

- hash functions, both on CPU and inside SNARKs
- algebra implementations (fields, elliptic curves, etc)
- circom / Groth16 implementations
- different proof systems
- approaches to lookup tables in SNARKs
- various zkVMs
- etc

Note: Right now, this is very much WIP...


### Hash functions

Some examples of relevant hash functions

- [ ] Poseidon
- [ ] Poseidon2
- [ ] Reinforced Concrete
- [ ] SHA256
- [ ] Keccak256
- [ ] Blake2
- [ ] Blake3

### Algebra backends

- [ ] Arkworks
- [ ] Constantine
- [ ] Gnark
- [ ] Zikkurat
- [ ] mcl

### circom / Groth16 provers

- [ ] SnarkJS
- [ ] RapidSnark
- [ ] Ark-circom
- [ ] Gnark
- [ ] Bellperson

### Proof system

- Groth16
- PLONK
- Spartan
- Nova
- STARK+FRI
- etc etc

### Lookup tables

- plookup
- logarithmic derivatives
- cached quotients
- Lasso
- etc (there a lot of variations)

### zkVMs

- Cairo (StarkWare)
- Risc0
- MidenVM
- TritonVM
- Lurk (LISP zkVM)
- etc

