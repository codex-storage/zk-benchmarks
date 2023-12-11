#!/bin/bash

# Set a default value if ZKBENCH_INPUT_SIZE_KB is not set
: ${ZKBENCH_INPUT_SIZE_KB:=1024}

# Set a default value if ZKBENCH_INPUT_SIZE_KB is not set
: ${WHICH:="all"}

# Run cargo run with the specified environment variable
cargo run $WHICH $ZKBENCH_INPUT_SIZE_KB
