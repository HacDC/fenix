# Setup Firmware Environment

## Install RustUp (if you haven't already)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install Build tools

```shell
cargo install espup
espup install -f espidf.sh -t all
source espidf.sh
```

## Install Firmware

```shell
cd fenix-rust
cargo build
cargo flash
```
