#!/bin/bash
if [ -z ${ZKBENCH_HASH_TYPE_TREE} ]; then 
ZKBENCH_HASH_TYPE_TREE="poseidon"    
fi

if [ -z ${ZKBENCH_TREE_DEPTH} ]; then 
ZKBENCH_TREE_DEPTH=4
fi

echo "Running benchmarks with the following configurations:"
echo "HASH            = $ZKBENCH_HASH_TYPE_TREE"
echo "Tree Depth = $ZKBENCH_TREE_DEPTH"

# Run the benchmarks
./target/release/plonky2_hash_benchmarks $ZKBENCH_HASH_TYPE_TREE $ZKBENCH_TREE_DEPTH