#!/bin/bash

if [ -z ${ZKBENCH_WHICH} ]; then 
ZKBENCH_WHICH="hash_sponge"
fi

ORIG=`pwd`
ROOT="${ORIG}/../../../../"

cd build

echo "circuit-specific ceremony..."
NAME="${ZKBENCH_WHICH}"
snarkjs groth16 setup ${NAME}.r1cs ${ROOT}/ceremony/ceremony.ptau ${NAME}_0000.zkey
echo "some_entropy" | snarkjs zkey contribute ${NAME}_0000.zkey ${NAME}_0001.zkey --name="1st Contributor Name"
rm ${NAME}_0000.zkey
mv ${NAME}_0001.zkey ${NAME}_prover.zkey
snarkjs zkey export verificationkey ${NAME}_prover.zkey ${NAME}_verification_key.json

cd $ORIG
