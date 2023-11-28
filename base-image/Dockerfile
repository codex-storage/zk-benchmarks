FROM ubuntu:22.04 AS base
SHELL ["/bin/bash", "--rcfile", "/root/.bashrc", "-lc"]

# === install some basic tools and libs ===

RUN apt-get -y update && \
    apt-get -y install curl wget vim less build-essential git             \
                       libgmp-dev zlib1g zlib1g-dev pkg-config libffi-dev \
                       libdigest-sha3-perl

RUN echo -e "\nalias dir=\"ls -la\"" >>$HOME/.bashrc

RUN mkdir /zk && mkdir /zk/versions

# === install python === 

RUN apt -y install python3 && \
    echo -e "\nalias python=python3" >>$HOME/.bashrc && \
    python3 --version >/zk/versions/python.txt 

# === install clang ===

RUN apt-get -y install clang && \
    clang --version >/zk/versions/clang.txt

# === install LLVM ===

RUN apt -y install llvm && \
    llvm-ar --version >/zk/versions/llvm.txt

# -- NOTE: this would install `llvm-*-16` instead of `llvm-*` which we need...
# RUN apt -y install lsb-release wget software-properties-common gnupg && \
#     cd /tmp && \
#     wget https://apt.llvm.org/llvm.sh && \
#     chmod u+x llvm.sh                 && \
#     ./llvm.sh 16

# === install node.js ===

ENV MY_NVM_VERSION="v20.2.0"

RUN cd /tmp && \
    curl --proto '=https' --tlsv1.2 https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh -sSf >install_nvm && \
    chmod u+x install_nvm        && \
    ./install_nvm                && \
    source $HOME/.nvm/nvm.sh     && \
    nvm install $MY_NVM_VERSION  && \
    nvm use     $MY_NVM_VERSION  && \
    rm ./install_nvm             && \
    echo -e "\nsource /root/.nvm/nvm.sh" >>$HOME/.bashrc && \
    nvm  --version >/zk/versions/nvm.txt   && \
    npm  --version >/zk/versions/npm.txt   && \
    node --version >/zk/versions/node.txt 

# === install Rust ===

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf >install_rustup && \
    chmod u+x install_rustup     && \
    echo -e "\n" >>$HOME/.bashrc && \
    sh ./install_rustup -y       && \
    rm ./install_rustup          && \
    export PATH="$HOME/.cargo/bin:$PATH"     && \
    rustc --version >/zk/versions/rustc.txt

# === install nim ===

RUN cd /tmp && \
    echo -e "\n" >>$HOME/.bashrc && \
    curl --proto '=https' --tlsv1.2 https://nim-lang.org/choosenim/init.sh -sSf >install_choosenim && \
    chmod u+x install_choosenim          && \
    sh ./install_choosenim -y            && \
    echo -e "\nexport PATH=/root/.nimble/bin:\$PATH" >>$HOME/.bashrc && \
    export PATH=/root/.nimble/bin:$PATH        && \
    choosenim stable                           && \
    nimble --version >/zk/versions/nimble.txt  && \
    nim    --version >/zk/versions/nim.txt     && \
    rm -rf /tmp/*

# === install Haskell ===

RUN cd /tmp && \
    apt-get install -y libffi-dev libffi8ubuntu1 libgmp-dev libgmp10 libncurses-dev libncurses5 libtinfo5 && \
    curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org >install_ghcup && \
    chmod u+x install_ghcup && \
    BOOTSTRAP_HASKELL_NONINTERACTIVE=1 sh ./install_ghcup && \
    source /root/.ghcup/env && \
    echo -e "\nsource /root/.ghcup/env" >>$HOME/.bashrc && \
    ghcup --version >/zk/versions/ghcup.txt  && \
    rm /tmp/install_ghcup       

RUN source /root/.ghcup/env && \
    ghcup upgrade               && \
    ghcup install cabal 2.4.1.0 && \
    ghcup set     cabal 2.4.1.0 && \
    ghcup install ghc   8.6.5   && \
    ghcup set     ghc   8.6.5   && \ 
    ghc   --version >/zk/versions/ghc.txt   && \
    cabal --version >/zk/versions/cabal.txt && \
    cabal update         
    
# === install circom & snarkjs ===

RUN cd /tmp && \
    git clone https://github.com/iden3/circom.git && \
    cd circom                   && \
    source $HOME/.bashrc        && \
    cargo build --release       && \
    cargo install --path circom && \
    circom --version >/zk/versions/circom.txt  && \
    rm -rf /tmp/circom
    
RUN source /root/.nvm/nvm.sh      && \
    npm install -g snarkjs        && \
    (snarkjs --version >/zk/versions/snarkjs.txt || true)
    
# === install go ===

ENV GO_TAR_FNAME="go1.20.4.linux-amd64.tar.gz"

RUN cd /tmp && \
    wget https://go.dev/dl/$GO_TAR_FNAME && \
    tar -C /usr/local -xzf $GO_TAR_FNAME && \
    export PATH=$PATH:/usr/local/go/bin  && \
    echo -e "\nexport PATH=\$PATH:/usr/local/go/bin" >>$HOME/.bashrc && \
    go version >/zk/versions/go.txt && \
    rm -rf /tmp/$GO_TAR_FNAME 

# === download and build constantine ===

RUN (mkdir /zk/src || true) && \
    cd /zk/src              && \
    git clone https://github.com/mratsim/constantine

RUN export PATH=/root/.nimble/bin:$PATH && \
    cd /zk/src/constantine   && \
    CC=clang nimble bindings && \
    nimble test_bindings

RUN export PATH=/root/.nimble/bin:$PATH && \
    cd /zk/src/constantine   && \
    nimble install
        
# === download gnark ===

RUN (mkdir /zk/src || true) && \
    cd /zk/src              && \
    git clone https://github.com/ConsenSys/gnark

# === download circomlib ===

RUN (mkdir /zk/src      || true) && \
    (mkdir /zk/circuits || true) && \
    cd /zk/src                   && \
    git clone https://github.com/iden3/circomlib && \
    cp -r /zk/src/circomlib/circuits /zk/circuits/circomlib
    
# === download and build MCL ===

RUN (mkdir /zk/src || true) && \
    cd /zk/src              && \
    git clone https://github.com/herumi/mcl
    
RUN cd /zk/src/mcl    && \
    make -j4

# === download and/or copy some powers of tau files ===

# note: 
#  - 2^16 = 75  megabytes
#  - 2^20 = 1.2 gigabytes
#  - 2^24 = 19  gigabytes

# RUN cd /zk/ptau  && \
#     wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_8.ptau  && \
#     wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_12.ptau && \
#     wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_16.ptau 

# RUN mkdir /zk/ptau 
# COPY ./ptau /zk/ptau


