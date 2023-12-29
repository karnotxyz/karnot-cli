#!/bin/bash

set -x #echo on

BASE_PATH={{BASE_PATH}}

# setup node
#cargo run --release -- setup --chain=dev --from-remote --base-path=$BASE_PATH

$HOME/.karnot/madara/target/debug/madara \
  --base-path=$BASE_PATH \
  --rpc-cors=all \
  --chain=dev \
  --force-authoring \
  --alice \
  --rpc-external \
  --rpc-methods=unsafe \
  --tx-ban-seconds 0