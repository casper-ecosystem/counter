# CasperLabs Smart Contract Template

Let's make the CasperLabs Smart Contract developer's life easier!

## Repository structure
Please take a moment to see how this repository is organized.

- `contracts` - Place where smart contracts live. There should be one Cargo project per smart contract. In this template we provide two smart contract: `counter-call` and `counter-define`.
- `tests` - Place where all test files live. Each Python file, that starts with `test_` should define `unittest.TestCase` classes with tests. All tests will be executed when calling `run_tests()` in interactive shell.
- `Cargo.toml` - Definition of workspace.
- `cl_test_context.py` - Python module that defines helper functions.
- `cl_test_runner.py` - Python script that is loaded at the start of interactive shell. It starts docker network and exports `run_tests()` method.
- `Makefile.toml` - List of tasks available via `cargo make <task>`. It defines `cargo make prepare` and `cargo make console`.
- `.cargo/config` - Cargo configure file. It sets `wasm32-unknown-unknown` as the prime target.
- `rust-toolchain` - Cargo configure file. It sets toolchain to `nightly-2019-08-25`

## Installation

Local environment requires installation of the following packages: `protobuf`, `git`, `docker`, `rust`, `cargo`, `cargo-make`, `python3` and `pipenv`.

## Step 1 - Install system prerequisites

### Debian, Ubuntu, Mint
```bash
$ sudo apt install protobuf-compiler git docker docker-compose
```

### macOS
```bash
$ brew install protobuf git pipenv md5sha1sum
```

#### Docker for macOS
Follow the instructions on [this page](https://docs.docker.com/docker-for-mac/install/) for platform specific installation steps

## Step 2 - Clone this repository
```bash
$ git clone https://github.com/zie1ony/casperlabs-smart-contract-template
$ cd casperlabs-smart-contract-template
```

## Step 3 - Install Rust environment
```bash
$ curl https://sh.rustup.rs -sSf | sh
$ cargo install cargo-make
```

## Step 4 - Install Python environment

### Debian, Ubuntu, Mint
```bash
$ sudo apt-get install python3-pip
$ sudo pip3 install pipenv
```

### macOS
```bash
$ brew install python3 pipenv
```

## Step 5 - Prepare local environment
Run script to prepare docker images and setup python environment for this project.
```bash
$ cargo make prepare
```

## Step 6 - Start interactive environment
Start interactive Python console with loaded developement node and with
predefined accounts. It takes around 15 seconds to start docker nodes.
```bash
$ cargo make console
```

## Step 7 - Run tests
Call python function `run_tests()` to test your smart contracts. Whenever you change files in `tests` or `contracts` directories it will be detected and rebuild for you.
```bash
>>> run_tests()
```