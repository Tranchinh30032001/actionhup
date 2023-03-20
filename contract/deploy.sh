#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi
echo ">> Deploying contractokok"


near deploy vinactionhup.testnet --wasmFile ./target/wasm32-unknown-unknown/release/hello_near.wasm
