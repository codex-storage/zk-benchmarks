#!/bin/bash

if [ -z ${ZKBENCH_NTHREADS} ]; then 
ZKBENCH_NTHREADS="default"    # https://doc.rust-lang.org/cargo/reference/config.html#buildjobs
fi

CARGO_BUILD_JOBS=$ZKBENCH_NTHREADS cargo build --release
