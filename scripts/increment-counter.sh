#!/bin/bash
CALL_WASM=target/wasm32-unknown-unknown/release/counter_call.wasm

RESPONSE=$(casperlabs-client --host deploy.casperlabs.io deploy \
    --private-key keys/key.private.key \
    --payment-amount 10 \
    --session $CALL_WASM \
)

HASH=$(echo $RESPONSE | awk '{print $3}')

echo "Deployed with hash $HASH"

sleep 5

casperlabs-client --host deploy.casperlabs.io show-deploy $HASH
