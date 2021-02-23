# Counter Smart Contract

This is an example of a simple smart contract, the Counter.

The steps below are a quick start if you have already set up the [nctl](https://github.com/CasperLabs/casper-node/tree/master/utils/nctl) testing tool. 

## Install

### Set up the Rust toolchain
You need the Rust toolchain to develop smart contracts.
```bash
$ make prepare
```

### Compile smart contracts
Compile WASM files that will be used later.
```bash
$ make build-contracts
```

## Prepare the local `nctl` network
Set up [nctl](https://github.com/CasperLabs/casper-node/tree/master/utils/nctl)

## Run
Set `CASPER_NODE_REPO` Bash variable to point at the `casper-node` directory.
Example:
```bash
$ export CASPER_NODE_REPO=/code/casper-node
```

### Deploy the Counter Smart Contract
The counter smart contract has to be deployed first.
```bash
$ make deploy
```

### Increment the counter
Make a deploy that increments the counter by 1.
```bash
$ make increment
```

### Get the counter value
Read the blockchain to get the counter value.
```bash
$ make check-counter
```
