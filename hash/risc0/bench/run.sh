#!/bin/bash
if [ -z ${ZKBENCH_HASH_TYPE} ]; then 
ZKBENCH_HASH_TYPE="sha256"    
fi

if [ -z ${ZKBENCH_WHICH} ]; then 
ZKBENCH_WHICH="LINEAR"    
fi

if [ -z ${ZKBENCH_NTHREADS} ]; then 
ZKBENCH_NTHREADS=1    
fi

if [ -z ${ZKBENCH_INPUT_SIZE_KB} ]; then 
ZKBENCH_INPUT_SIZE_KB=1024
fi

if [ -z ${ZKBENCH_TREE_DEPTH} ]; then 
ZKBENCH_TREE_DEPTH=4
fi


echo "Running benchmarks with the following configurations:"
echo "HASH            = $ZKBENCH_HASH_TYPE"
# echo "WHICH           = $ZKBENCH_WHICH"
echo "NTHREADS        = $ZKBENCH_NTHREADS"
echo "Input Size (KB) = $ZKBENCH_INPUT_SIZE_KB"

# Run the benchmarks using cargo run
# Check if the environment variable is set to "poseidon2_bn128"
if [[ "$ZKBENCH_HASH_TYPE" == "poseidon2_bn128" || "$ZKBENCH_HASH_TYPE" == "poseidon2_babybear" || "$ZKBENCH_HASH_TYPE" == "poseidon2_babybear_native" ]]; then
    # echo "Running Poseidon2 benchmark over bn128 field"
    CARGO_BUILD_JOBS=$ZKBENCH_NTHREADS cargo run $ZKBENCH_HASH_TYPE $ZKBENCH_TREE_DEPTH
else
    CARGO_BUILD_JOBS=$ZKBENCH_NTHREADS cargo run $ZKBENCH_HASH_TYPE $ZKBENCH_INPUT_SIZE_KB

fi