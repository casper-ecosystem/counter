#!/bin/bash
PATH_TO_CASPER_NODE_REPO=/home/ethilios/CasperLabs/casper-node/

NODE_ADDRESS=http://localhost:40101
STATE_ROUTE_HASH=$(casper-client get-block -n $NODE_ADDRESS | jq -r '.result.block.header.state_root_hash')
PUBLIC_KEY=$PATH_TO_CASPER_NODE_REPO/utils/nctl/assets/net-1/nodes/node-2/keys/public_key.pem

echo "Value of the 'count' in a Smart Contract named 'counter'"
casper-client query-state \
    --node-address $NODE_ADDRESS \
    --state-root-hash $STATE_ROUTE_HASH \
    --key $PUBLIC_KEY \
    #--query-path "count"

