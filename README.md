# Counter Example

This is an example of simple smart contract and how to deploy it to our devnet. The example is a counter smart contract.

Repository uses our template: https://github.com/zie1ony/casperlabs-smart-contract-template .

# Usage

## Step 1 - Compile smart contracts.
```bash
$ cargo build --release
```

## Step 2 - Deploy the counter smart contract.
Make sure to run scripts in the root directory!
```bash
$ ./scripts/deploy-smart-contract.sh
```

## Step 3 - Check the counter value.
```bash
$ ./scripts/check-counter.sh
```
Value of the counter should be `0`.

## Step 4 - Increment the counter.
```bash
$ ./scripts/increment-counter.sh
```

Now check the value of the counter again. It should be `1`.

## GraphQL
You can check the value of the counter using devnet's GraphQL console:
https://devnet-graphql.casperlabs.io

Go to and then:

### Check latest block hash in 
```
query {
  dagSlice(depth: 1) {
      blockHash
  }
}
```

### Get public key of your account
```bash
$ cat keys/key.public.hex.key
```

### Check the counter value.
Put block hash under `blockHashBase16Prefix` and your public key under `keyBase`.
```
query {
  globalState(
    blockHashBase16Prefix: "96720f16a215b5e55f1a7475256370f48efa932248b7bcd633d29413a5c1f033"
    StateQueries: [
      {
        keyType: Address
        keyBase16: "64d0c86f888e925731cae4398c6ea86d26a14e2574e70b36bd4eeaec3a292cde"
        pathSegments: ["counter", "count"]
      }
    ]
  ) {
    value {
      __typename
      ... on IntValue {
        int: value
      }
    }
  }
}
```