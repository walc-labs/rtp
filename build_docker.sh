#!/usr/bin/env bash
set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

NAME="build_rtp"

if docker ps -a --format '{{.Names}}' | grep -Eq "^${NAME}\$"; then
    echo "Container exists"
else
docker create \
    --mount type=bind,source=$DIR,target=/host \
    --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
    --name=$NAME \
    -w /host \
    -e RUSTFLAGS='-C link-arg=-s' \
    -it \
    nearprotocol/contract-builder \
    /bin/bash
fi

docker start $NAME
docker exec $NAME /bin/bash -c "rustup default 1.73; \
    rustup target add wasm32-unknown-unknown; \
    cargo build -p rtp --target wasm32-unknown-unknown --release; \
    cargo build -p rtp-factory --target wasm32-unknown-unknown --release"

mkdir -p res
cp $DIR/target/wasm32-unknown-unknown/release/*.wasm $DIR/res/

# wasm-opt -Oz res/rtp.wasm -o res/rtp.wasm --strip-debug --vacuum
# wasm-opt -Oz res/rtp_factory.wasm -o res/rtp.wasm --strip-debug --vacuum
