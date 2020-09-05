#!/bin/sh
set -e

echop() {
    echo "\n$@\n--------------------------------\n"
}

echop "Installing rustup..."
sudo apt -qqy install curl git
./scripts/rustup-init.sh

echop "Installing stable rust with rustup..."
rustup install stable

echop "Installing clippy..."
rustup component add clippy

echop "Installing rustfmt..."
rustup component add rustfmt

echop "Installing rust-analyzer..."
mkdir -p temp_install
pushd temp_install
git clone https://github.com/rust-analyzer/rust-analyzer.git && cd rust-analyzer
cargo xtask install --server

