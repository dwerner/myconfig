#!/usr/bin/env fish
export EDITOR='nvim'
export PATH="/usr/local/bin:$PATH"
export RUST_BACKTRACE=full
export RUSTUP_HOME="$HOME/.rustup"
export PATH="$HOME/.cargo/bin:$PATH"
export RUST_SRC_PATH=(rustc --print sysroot)"/lib/rustlib/src/rust/src"

