#!/bin/bash
cd blake2
cargo prove build
cd ../keccak
cargo prove build
cd ../sha256
cargo prove build
cd ../blake3
cargo prove build
cd ../poseidon2
cargo prove build
cd ../script
cargo build --release
