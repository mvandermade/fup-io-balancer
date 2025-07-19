#!/usr/bin/env -S bash -eEu -o pipefail

echo 'going to compile proto files'

mkdir -p ./proto-rust

docker run --rm -v"$(pwd)":/code -w /code rvolosatovs/protoc -I=. \
    --rust_out=experimental-codegen=enabled,kernel=cpp:"./proto-rust" \
    balancerapi/*.proto

echo 'finished compiling '
