#!/bin/bash

if [ -z ${ZKBENCH_WHICH} ]; then 
ZKBENCH_WHICH="LINEAR"    
fi

if [ -z ${ZKBENCH_NTHREADS} ]; then 
ZKBENCH_NTHREADS=1    
fi

if [ -z ${ZKBENCH_MEGABYTES} ]; then 
ZKBENCH_MEGABYTES=128
fi

echo "WHICH     = $ZKBENCH_WHICH"
echo "NTHREADS  = $ZKBENCH_NTHREADS"
echo "MEGABYTES = $ZKBENCH_MEGABYTES"

case $ZKBENCH_WHICH in
  LINEAR)
    build/bench_linear $ZKBENCH_MEGABYTES
    ;;
  MERKLE)
    build/bench_merkle $ZKBENCH_MEGABYTES $ZKBENCH_NTHREADS
    ;;
  *)
    echo "unknown selector: \`$ZKBENCH_WHICH\`"
    exit 99
    ;;
esac
