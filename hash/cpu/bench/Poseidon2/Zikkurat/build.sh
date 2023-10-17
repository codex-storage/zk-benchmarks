#!/bin/bash
echo "build"

ZIK_PATH="../../../src/zikkurat-algebra/"

mkdir -p build

gcc -O3 \
  -I ${ZIK_PATH}/lib/cbits/bigint/             \
  -I ${ZIK_PATH}/lib/cbits/curves/fields/std/  \
  -I ${ZIK_PATH}/lib/cbits/curves/fields/mont/ \
  ${ZIK_PATH}/lib/cbits/bigint/bigint256.c     \
  ${ZIK_PATH}/lib/cbits/curves/fields/std/bn128_r_std.c    \
  ${ZIK_PATH}/lib/cbits/curves/fields/mont/bn128_r_mont.c  \
  poseidon2.c \
  -o build/a.out
