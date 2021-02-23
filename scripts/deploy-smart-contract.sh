#!/bin/bash
if [ -z "$CASPER_NODE_REPO" ]; then
    echo "\$CASPER_NODE_REPO is not set."
    echo "Example:"
    echo "    export CASPER_NODE_REPO=/tmp/casper-node."
    exit 1
fi

NODE_ADDRESS=http://localhost:40101
FAUCET_SECRET_KEY=$CASPER_NODE_REPO/utils/nctl/assets/net-1/faucet/secret_key.pem
CASPER_CLIENT=$CASPER_NODE_REPO/target/release/casper-client
DEFINE_WASM=../target/wasm32-unknown-unknown/release/counter-define.wasm

RESPONSE=$($CASPER_CLIENT put-deploy \
    --node-address $NODE_ADDRESS \
    --chain-name casper-net-1 \
    --secret-key $FAUCET_SECRET_KEY \
    --payment-amount 100000000000 \
    --session-path $DEFINE_WASM \
)

HASH=$(echo $RESPONSE | jq -r '.result.deploy_hash')
echo "Deployed with hash: $HASH"
echo "Waiting for deploy to process..."
sleep 10
echo "Deploy result:"
$CASPER_CLIENT get-deploy $HASH --node-address $NODE_ADDRESS
