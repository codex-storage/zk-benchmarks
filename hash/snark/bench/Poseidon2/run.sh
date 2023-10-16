#!/bin/bash

ORIG=`pwd`

cd build

NAME="hash_sponge"
echo "generating proof with snarkjs"
snarkjs groth16 prove ${NAME}_prover.zkey ${NAME}_witness.wtns ${NAME}_proof.json ${NAME}_public.json

cd $ORIG
