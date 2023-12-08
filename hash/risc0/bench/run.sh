#!/bin/bash

# Set a default value if ZKBENCH_INPUT_SIZE_KB is not set
: ${ZKBENCH_INPUT_SIZE_KB:=1024}

# Run cargo run with the specified environment variable
cargo run $ZKBENCH_INPUT_SIZE_KB
