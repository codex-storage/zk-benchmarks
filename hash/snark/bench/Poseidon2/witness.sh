#!/bin/bash

if [ -z ${ZKBENCH_INPUT_SIZE} ]; then 
ZKBENCH_INPUT_SIZE=256
fi

if [ -z ${ZKBENCH_WHICH} ]; then 
ZKBENCH_WHICH="hash_sponge"
fi

ORIG=`pwd`

cd build

echo "generating input..."
./generate_input $ZKBENCH_INPUT_SIZE >input.json

echo "generating witness... (WASM)"
NAME="${ZKBENCH_WHICH}"
cd ${NAME}_js
node generate_witness.js ${NAME}.wasm ../input.json ../${NAME}_witness.wtns || { echo "witness gen failed"; exit 101; }
cd ..

cd $ORIG
