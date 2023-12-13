#!/bin/bash

if [ -z ${ZKBENCH_PROVER} ]; then 
ZKBENCH_PROVER="snarkjs"
fi

# if [ -z ${ZKBENCH_WHICH} ]; then 
# ZKBENCH_WHICH="hash_sponge_rate1"
# fi

ORIG=`pwd`

cd build

NAME="hash"
echo "generating proof with ${ZKBENCH_PROVER}"
case $ZKBENCH_PROVER in
  snarkjs)
    snarkjs groth16 prove ${NAME}_prover.zkey ${NAME}_witness.wtns ${NAME}_proof.json ${NAME}_public.json
    ;;
  rapidsnark)
    rapidsnark ${NAME}_prover.zkey ${NAME}_witness.wtns ${NAME}_proof.json ${NAME}_public.json
    ;;
  *)
    echo "unknown prover \`$ZKBENCH_PROVER\`"
    exit 99
    ;;
esac

cd $ORIG
