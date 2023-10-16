#!/bin/bash

if [ -z ${ZKBENCH_CEREMONY_SIZE} ]; then
ZKBENCH_CEREMONY_SIZE=16
fi

echo "ceremony size = ${ZKBENCH_CEREMONY_SIZE}"

snarkjs powersoftau new bn128 ${ZKBENCH_CEREMONY_SIZE} pot_0000.ptau
echo foobar | snarkjs powersoftau contribute pot_0000.ptau pot_0001.ptau --name="First contribution" 
snarkjs powersoftau prepare phase2 pot_0001.ptau ceremony.ptau

rm pot20_0000.ptau
mv pot20_0001.ptau ceremony.ptau
