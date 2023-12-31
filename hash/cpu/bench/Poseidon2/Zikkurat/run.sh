#!/bin/bash

if [ -z ${ZKBENCH_NTHREADS}  ]; then 
ZKBENCH_NTHREADS=1    
fi

if [ -z ${ZKBENCH_TREE_DEPTH} ]; then 
ZKBENCH_TREE_DEPTH=16
fi

echo "NTHREADS   = $ZKBENCH_NTHREADS"
echo "TREE_DEPTH = $ZKBENCH_TREE_DEPTH"

./build/a.out $ZKBENCH_TREE_DEPTH $ZKBENCH_NTHREADS