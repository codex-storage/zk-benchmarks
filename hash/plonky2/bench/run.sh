#!/bin/bash
if [ -z ${ZKBENCH_HASH_TYPE} ]; then 
ZKBENCH_HASH_TYPE="sha256"    
fi

if [ -z ${ZKBENCH_INPUT_SIZE_BYTES} ]; then 
ZKBENCH_INPUT_SIZE_BYTES=256
fi

echo "Running benchmarks with the following configurations:"
echo "HASH            = $ZKBENCH_HASH_TYPE"
echo "Input Size (Bytes) = $ZKBENCH_INPUT_SIZE_BYTES"

# Run the benchmarks
./target/release/plonky2_hash_benchmarks $ZKBENCH_HASH_TYPE $ZKBENCH_INPUT_SIZE_BYTES