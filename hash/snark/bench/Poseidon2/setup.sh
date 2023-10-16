#!/bin/bash

ORIG=`pwd`
ROOT="${ORIG}/../../../../"

cd build

echo "generating input..."
./generate_input $ZKBENCH_INPUT_SIZE >input.json

echo "generating witness..."
NAME="hash_sponge"
cd ${NAME}_js
node generate_witness.js ${NAME}.wasm ../input.json ../${NAME}_witness.wtns || { echo "witness gen failed"; exit 101; }
cd ..

echo "circuit-specific ceremony..."
snarkjs groth16 setup ${NAME}.r1cs ${ROOT}/ceremony/ceremony.ptau ${NAME}_0000.zkey
echo "some_entropy" | snarkjs zkey contribute ${NAME}_0000.zkey ${NAME}_0001.zkey --name="1st Contributor Name"
rm ${NAME}_0000.zkey
mv ${NAME}_0001.zkey ${NAME}_prover.zkey
snarkjs zkey export verificationkey ${NAME}_prover.zkey ${NAME}_verification_key.json

cd $ORIG
