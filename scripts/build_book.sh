#!/usr/bin/env bash
set -euo pipefail

echo "==================Installing necessary binaries=================="

if ! command -v mdbook >/dev/null 2>&1; then
    echo "mdbook not found, installing..."
    cargo install mdbook
else
    echo "mdbook already installed"
fi

if ! command -v wasm-bindgen >/dev/null 2>&1; then
    echo "wasm-bindgen not found, installing..."
    cargo install wasm-bindgen-cli
else
    echo "wasm-bindgen already installed"
fi

echo "==================Building book=================="

mdbook build

echo "==================Building examples=================="

echo "> Building ball-pit"
cargo build --target wasm32-unknown-unknown -p ball-pit -r
echo "> Building counter"
cargo build --target wasm32-unknown-unknown -p counter -r
echo "> Building readme"
cargo build --target wasm32-unknown-unknown -p readme -r

echo "==================Removing old bindings=================="

rm -rf target/book/wasm

echo "==================Generating new bindings=================="

echo "> Generating ball-pit bindings"
wasm-bindgen --target web \
    --out-dir target/book/wasm/ball-pit \
    target/wasm32-unknown-unknown/release/ball-pit.wasm
echo "> Generating counter bindings"
wasm-bindgen --target web \
    --out-dir target/book/wasm/counter \
    target/wasm32-unknown-unknown/release/counter.wasm
echo "> Generating readme bindings"
wasm-bindgen --target web \
    --out-dir target/book/wasm/readme \
    target/wasm32-unknown-unknown/release/readme.wasm