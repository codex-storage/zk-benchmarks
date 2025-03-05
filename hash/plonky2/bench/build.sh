#!/bin/bash

# Set nightly as the default toolchain
rustup override set nightly

# Build 
RUSTFLAGS=-Ctarget-cpu=native cargo build --release --bin plonky2_hash_benchmarks


