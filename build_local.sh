#!/bin/bash
set -e
cd "`dirname $0`"

mkdir -p res

cargo build -p rtp --target wasm32-unknown-unknown --release
cargo build -p rtp-factory --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
