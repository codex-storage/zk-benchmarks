#!/bin/bash

# Set nightly as the default toolchain
rustup override set nightly

# Build 
cargo build

