#!/bin/bash

ORIG=`pwd`
cd ../../../external/Blake3/
IMPL=`pwd`

cd $IMPL
cargo build --release

cd $IMPL/b3sum
cargo build --release

cd $ORIG

mkdir -p build

cp $IMPL/b3sum/target/release/b3sum build/

gcc -O3 fakedata.c -o build/fakedata

