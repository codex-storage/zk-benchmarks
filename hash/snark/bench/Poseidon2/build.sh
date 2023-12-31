#!/bin/bash

if [ -z ${ZKBENCH_INPUT_SIZE} ]; then 
ZKBENCH_INPUT_SIZE=256
fi

if [ -z ${ZKBENCH_WHICH} ]; then 
ZKBENCH_WHICH="hash_sponge_rate1"
fi

ORIG=`pwd`

mkdir -p build 

gcc -O3 generate_input.c -o build/generate_input || { echo "gcc failed"; exit 101; }

#NAME=${ZKBENCH_WHICH}
NAME="hash"
cat ${NAME}.circom.template \
  | sed "s/ZKBENCH_INPUT_SIZE/${ZKBENCH_INPUT_SIZE}/g" \
  | sed "s/ZKBENCH_WHICH/${ZKBENCH_WHICH}/g"           \
  >build/${NAME}.circom

cd build

circom ${NAME}.circom --r1cs --wasm || { echo "circom failed"; exit 102; }
 
cd $ORIG