#!/bin/bash
CALL_WASM=target/wasm32-unknown-unknown/release/counter_call.wasm
PATH_TO_CASPER_NODE_REPO=/home/ethilios/CasperLabs/casper-node/

NODE_ADDRESS=http://localhost:40101

RESPONSE=$(casper-client put-deploy \
    --node-address $NODE_ADDRESS \
    --chain-name casper-net-1 \
    --secret-key $PATH_TO_CASPER_NODE_REPO/utils/nctl/assets/net-1/nodes/node-2/keys/secret_key.pem  \
    --payment-amount 10000000000000000000 \
    --session-path $CALL_WASM \
)

HASH=$(echo $RESPONSE | jq -r '.result.deploy_hash')

echo "Deployed with hash -> $HASH"

sleep 10

casper-client get-deploy $HASH --node-address $NODE_ADDRESS
