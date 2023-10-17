#!/bin/bash

if [ -z ${ZKBENCH_NTHREADS}  ]; then 
ZKBENCH_NTHREADS=1    
fi

if [ -z ${ZKBENCH_MEGABYTES} ]; then 
ZKBENCH_MEGABYTES=256
fi

echo "NTHREADS  = $ZKBENCH_NTHREADS"
echo "MEGABYTES = $ZKBENCH_MEGABYTES"

./build/fakedata $ZKBENCH_MEGABYTES | ./build/b3sum --num-threads $ZKBENCH_NTHREADS -
