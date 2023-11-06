#!/bin/bash

if [ -z ${ZKBENCH_MEGABYTES} ]; then 
ZKBENCH_MEGABYTES=128
fi

echo "MEGABYTES = $ZKBENCH_MEGABYTES"

OSTYPE=`uname -s`

case $OSTYPE in
  Darwin)
    ./build/fakedata $ZKBENCH_MEGABYTES | shasum -a256 -b -
    ;;
  Linux)  
    ./build/fakedata $ZKBENCH_MEGABYTES | sha256sum -b -
    ;;
  FreeBSD)  
    ./build/fakedata $ZKBENCH_MEGABYTES | sha256sum -b -
    ;;

  *)
    echo "unknown operating system \`$OSTYPE\`"
    exit 99
    ;;
esac
