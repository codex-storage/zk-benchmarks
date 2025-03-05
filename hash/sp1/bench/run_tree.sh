#!/bin/bash
if [ -z ${ZKBENCH_HASH_TYPE_TREE} ]; then 
ZKBENCH_HASH_TYPE_TREE="poseidon2"    
fi

if [ -z ${ZKBENCH_TREE_DEPTH} ]; then 
ZKBENCH_TREE_DEPTH=4
fi

cd script
echo "Running benchmarks with the following configurations:"
echo "HASH            = $ZKBENCH_HASH_TYPE_TREE"
echo "Tree Depth = $ZKBENCH_TREE_DEPTH"

# Run the benchmarks
RUST_LOG=info ./target/release/bench-script $ZKBENCH_HASH_TYPE_TREE $ZKBENCH_TREE_DEPTH