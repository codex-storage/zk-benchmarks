#!/bin/bash

gcc -O3 sha2.c bench_linear.c -o bench_linear
gcc -O3 sha2.c bench_merkle.c -o bench_merkle
