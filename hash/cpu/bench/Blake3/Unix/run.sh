#!/bin/bash
./fakedata $ZKBENCH_MEGABYTES | ./b3sum --num-threads $ZKBENCH_NTHREADS -