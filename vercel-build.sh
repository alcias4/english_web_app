#!/usr/bin/env bash

set -e

echo "=== Using Vercel Rust installation ==="

# Vercel ya tiene Rust instalado aquí
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

# El instalador de Dioxus puede colocar dx en un directorio del usuario
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:/rust/bin:$PATH"

echo "=== Dioxus version ==="
dx --version

echo "=== Building Dioxus Web ==="
dx bundle --web --release --out-dir dist

echo "=== Build finished ==="
