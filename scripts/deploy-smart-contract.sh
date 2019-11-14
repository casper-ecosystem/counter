#!/bin/bash
DEFINE_WASM=target/wasm32-unknown-unknown/release/counter_define.wasm

RESPONSE=$(casperlabs-client --host deploy.casperlabs.io deploy \
    --private-key keys/private-key \
    --payment-amount 10000000 \
    --session $DEFINE_WASM \
)

HASH=$(echo $RESPONSE | awk '{print $3}')

echo "Deployed with hash $HASH"

sleep 5

casperlabs-client --host deploy.casperlabs.io show-deploy $HASH
