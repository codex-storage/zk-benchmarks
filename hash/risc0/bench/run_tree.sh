#!/bin/bash
if [ -z ${ZKBENCH_HASH_TYPE} ]; then 
ZKBENCH_HASH_TYPE="poseidon2_babybear_native"    
fi

if [ -z ${ZKBENCH_TREE_DEPTH} ]; then 
ZKBENCH_TREE_DEPTH=2
fi

echo "Running benchmarks with the following configurations:"
echo "HASH            = $ZKBENCH_HASH_TYPE"
echo "Tree Depth = $ZKBENCH_TREE_DEPTH"

# Run the benchmarks
./target/release/benchmark $ZKBENCH_HASH_TYPE $ZKBENCH_TREE_DEPTH