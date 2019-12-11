#!/bin/bash

PUBLIC_KEY=$(cat keys/public-key-hex)

RESPONSE=$(casperlabs-client --host deploy.casperlabs.io show-blocks)

BLOCK_HASH=$(echo $RESPONSE | awk -F "block_hash: \"" '{print $2}' | awk -F "\" header" '{print $1}')

echo "Value of the 'count' in a Smart Contract named 'counter' deployed under $PUBLIC_KEY account."
casperlabs-client --host deploy.casperlabs.io query-state \
    --block-hash $BLOCK_HASH \
    --type address \
    --key $PUBLIC_KEY \
    --path "counter/count"

