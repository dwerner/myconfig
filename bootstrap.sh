#!/bin/bash
# base system packages - ie rust, curl, git. Other stuff should be added to packages.yaml
set -e

echop() {
    echo "\n$@\n--------------------------------\n"
}

echop "Installing rustup..."
sudo apt -qqy install curl build-essential
./scripts/rustup-init.sh

. $HOME/.cargo/env

echop "Installing stable rust with rustup..."
rustup install stable

echop "Installing clippy..."
rustup component add clippy

echop "Installing rustfmt..."
rustup component add rustfmt

echop "Installing rust-analyzer..."
mkdir -p temp_install
pushd temp_install
git clone https://github.com/rust-analyzer/rust-analyzer.git
pushd rust-analyzer
cargo xtask install --server
popd

echo "Running..."
cargo run
