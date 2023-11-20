#!/bin/bash

# Check if ZKBENCH_INPUT_SIZE_KB is set, otherwise set a default value
ZKBENCH_INPUT_SIZE_KB=${ZKBENCH_INPUT_SIZE_KB:-"1"}  # Default to 1 if not set

# Run benchmarks with the specified input size
if [ "$ZKBENCH_INPUT_SIZE_KB" = "1" ]; then
    cargo bench --bench bench_main -- 1
elif [ "$ZKBENCH_INPUT_SIZE_KB" = "2" ]; then
    cargo bench --bench bench_main -- 2
elif [ "$ZKBENCH_INPUT_SIZE_KB" = "10" ]; then
    cargo bench --bench bench_main -- 10
else
    echo "Invalid input size: $ZKBENCH_INPUT_SIZE_KB"
fi
