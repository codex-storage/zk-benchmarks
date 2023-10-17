#!/bin/bash

if [ -z ${ZKBENCH_MEGABYTES} ]; then 
ZKBENCH_MEGABYTES=128
fi

echo "MEGABYTES = $ZKBENCH_MEGABYTES"

./build/fakedata $ZKBENCH_MEGABYTES | shasum -a256 -b -