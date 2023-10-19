
ZK benchmarking Docker image
============================

This is a Docker image intended for experimenting with and benchmarking various ZKP implementations.

It should have many of the standard tools, for example:

- languages: node.js, Rust, Haskell (GHC), Nim, Go, Python3
- circom, snarkjs, rapidsnark, circomlib
- libraries: arkworks, gnark, constantine... 
- perpetual powers of tau ceremony files
- etc


Building
--------

Build it with:

    $ docker build . -t zk-bench

Note: this will take a *long time*. 
After it (hopefully) finished, you can
Test the image with an interactive shell:

    $ docker run -it zk-bench

or with terminal colors enabled:

    $ docker run -e "TERM=xterm-256color" -it zk-bench
    

Using
-----

Hopefully the tools will work, and you will find stuff under `/zk`.


