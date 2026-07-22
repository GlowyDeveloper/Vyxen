#!/usr/bin/env bash
set -euo pipefail

targets=(
    x86_64-pc-windows-msvc
    x86_64-unknown-linux-gnu
    aarch64-apple-darwin
    x86_64-apple-darwin
    aarch64-apple-ios
    aarch64-apple-ios-sim
    aarch64-linux-android
    wasm32-unknown-unknown
)

for target in "${targets[@]}"; do
    echo "==> Installing $target"
    rustup target add "$target"
    echo "==> Checking $target"
    cargo check --all-targets --all-features --target "$target"
done