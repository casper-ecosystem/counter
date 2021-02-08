#!/bin/bash
DEFINE_WASM=target/wasm32-unknown-unknown/release/counter_define.wasm
PATH_TO_CASPER_NODE_REPO=/home/ethilios/CasperLabs/casper-node/

RESPONSE=$(casper-client put-deploy \
    --node-address http://localhost:40101 \
    --chain-name casper-net-1 \
    --secret-key $PATH_TO_CASPER_NODE_REPO/utils/nctl/assets/net-1/nodes/node-2/keys/secret_key.pem \
    --payment-amount 10000000000000000000000000 \
    --session-path $DEFINE_WASM \
)

HASH=$(echo $RESPONSE | jq -r '.result.deploy_hash')

echo "Deployed with hash -> $HASH"

echo "Waiting for deploy to process..."
sleep 10

echo "Deploy result ->"
casper-client get-deploy $HASH --node-address http://localhost:40101
