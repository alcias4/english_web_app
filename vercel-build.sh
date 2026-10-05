#!/usr/bin/env bash

set -e

# Vercel ya trae Rust en esta imagen
export PATH="/rust/bin:$PATH"

echo "Rust:"
rustc --version
cargo --version

echo "Installing cargo-binstall..."
curl -L --proto '=https' --tlsv1.2 -sSf \
https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
| bash

export PATH="$HOME/.cargo/bin:/rust/bin:$PATH"

echo "Installing Dioxus CLI..."
cargo binstall dioxus-cli -y --force

echo "Dioxus:"
dx --version

echo "Building..."
dx bundle --web --release --out-dir dist
