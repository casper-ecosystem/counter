#!/bin/bash
if [ -z "$CASPER_NODE_REPO" ]; then
    echo "\$CASPER_NODE_REPO is not set."
    echo "Example:"
    echo "    export CASPER_NODE_REPO=/tmp/casper-node."
    exit 1
fi
# CASPER_NODE_REPO=/home/ziel/workspace/casperlabs/casper-node

NODE_ADDRESS=http://localhost:40101
FAUCET_PUBLIC_KEY=$CASPER_NODE_REPO/utils/nctl/assets/net-1/faucet/public_key.pem
CASPER_CLIENT=$CASPER_NODE_REPO/target/release/casper-client

STATE_ROUTE_HASH=$($CASPER_CLIENT get-block -n $NODE_ADDRESS | jq -r '.result.block.header.state_root_hash')

RESPONSE=$($CASPER_CLIENT query-state \
    --node-address $NODE_ADDRESS \
    --state-root-hash $STATE_ROUTE_HASH \
    --key $FAUCET_PUBLIC_KEY \
    --query-path "counter/count")

echo "Value of the 'count' integer under a Smart Contract named 'counter':"
echo $(echo $RESPONSE | jq -r '.result.stored_value.CLValue.parsed')

