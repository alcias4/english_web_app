#!/usr/bin/env bash

set -e

echo "=== Using Vercel Rust installation ==="

source /rust/env
export PATH="/rust/bin:$PATH"

echo "=== Rust versions ==="
rustc --version
cargo --version
rustup --version

echo "=== Installing WebAssembly target ==="
rustup target add wasm32-unknown-unknown

echo "=== Installing Dioxus CLI ==="
curl -fsSL https://dioxus.dev/install.sh | bash

export PATH="$HOME/.local/bin:$HOME/.cargo/bin:/rust/bin:$PATH"

echo "=== Dioxus version ==="
dx --version

echo "=== Building Dioxus Web ==="
dx bundle --web --release --out-dir dist
