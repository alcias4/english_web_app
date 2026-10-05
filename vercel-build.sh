#!/usr/bin/env bash

set -e

# Instalar Rust
curl --proto '=https' \
    --tlsv1.2 \
    -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal

source "$HOME/.cargo/env"

# Target WebAssembly
rustup target add wasm32-unknown-unknown

# Instalar Dioxus CLI
curl -sSL https://dioxus.dev/install.sh | bash

export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

# Compilar Dioxus Web
dx bundle --web --release --out-dir dist
