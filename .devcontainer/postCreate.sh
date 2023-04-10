#!/bin/bash -e

function deps {
    sudo apt update 
    # Clang for mold, which is just a dev productivity tool
    # rest for libseccomp
    sudo apt install -y clang

    # Annoyingly we dont need the whole libseccomp-dev package, we just need a symlink
    sudo ln -s ./libseccomp.so.2 /usr/lib/x86_64-linux-gnu/libseccomp.so
}

function mold {
    curl -L https://github.com/rui314/mold/releases/download/v1.11.0/mold-1.11.0-x86_64-linux.tar.gz \
     | sudo tar -C /opt -xz

    echo 'PATH="$PATH:/opt/mold-1.11.0-x86_64-linux/bin"' \
     >> ~/.profile
}

function main {
    deps
    mold
}

main "$@"
