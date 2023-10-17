#!/bin/bash

mkdir -p build

gcc -O3 sha2.c bench_linear.c -o build/bench_linear
gcc -O3 sha2.c bench_merkle.c -o build/bench_merkle
