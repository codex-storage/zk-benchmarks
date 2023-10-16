#!/bin/bash

ORIG=`pwd`

mkdir -p build 

gcc -O3 generate_input.c -o build/generate_input || { echo "gcc failed"; exit 101; }

sed "s/ZKBENCH_INPUT_SIZE/${ZKBENCH_INPUT_SIZE}/g" hash_sponge.circom.template >build/hash_sponge.circom

cd build

circom hash_sponge.circom --r1cs --wasm || { echo "circom failed"; exit 102; }
 
cd $ORIG