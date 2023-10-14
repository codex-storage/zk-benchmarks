#!/bin/bash

ORIG=`pwd`
cd ../../src/Blake3/
IMPL=`pwd`

cd $IMPL
cargo build --release

cd $IMPL/b3sum
cargo build --release

cd $ORIG
cp $IMPL/b3sum/target/release/b3sum .

gcc -O3 fakedata.c -o fakedata

