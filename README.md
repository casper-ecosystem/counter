# CasperLabs Smart Contract Template

CasperLabs local environment requires installation of the following packages: `protobuf`, `git`, `docker`, `rust`, `cargo`, `cargo-make`, `python` and `pipenv`.

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
Follow the instructions on [this page](https://docs.docker.com/install/) for platform specific installation steps

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
Start interactive Python console with loaded developemnt node and with
predefined accounts.
```bash
$ cargo make console
```

## Step 7 - Run tests
Call python function `run_tests()` to test your smart contracts.
```bash
>>> run_tests()
```