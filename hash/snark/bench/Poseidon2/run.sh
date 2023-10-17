#!/bin/bash

if [ -z ${ZKBENCH_PROVER} ]; then 
ZKBENCH_PROVER="snarkjs"
fi

if [ -z ${ZKBENCH_WHICH} ]; then 
ZKBENCH_WHICH="hash_sponge"
fi

ORIG=`pwd`

cd build

echo "generating proof with ${ZKBENCH_PROVER}"
case $ZKBENCH_PROVER in
  snarkjs)
    NAME="${ZKBENCH_WHICH}"
    snarkjs groth16 prove ${NAME}_prover.zkey ${NAME}_witness.wtns ${NAME}_proof.json ${NAME}_public.json
    ;;
  rapidsnark)
    NAME="${ZKBENCH_WHICH}"
    rapidsnark ${NAME}_prover.zkey ${NAME}_witness.wtns ${NAME}_proof.json ${NAME}_public.json
    ;;
  *)
    echo "unknown prover \`$ZKBENCH_PROVER\`"
    exit 99
    ;;
esac

cd $ORIG
