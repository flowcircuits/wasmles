#!/bin/bash
set -e

# Setup script for @flowcircuits/wasmles
# Installs the Rust + WASM toolchain and JS dependencies for a fresh checkout.

echo "Setting up wasmles..."

# Rust toolchain
if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo not found. Install Rust from https://rustup.rs"
    exit 1
fi
echo "Using $(cargo --version)"

# wasm-pack needs the wasm32 target
echo "Ensuring wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true

# JS deps (includes the wasm-pack binary, a devDependency)
echo "Installing dependencies..."
yarn install

echo ""
echo "Setup complete. Build with 'yarn build', test with 'yarn test'."
