This crate produces two binaries:

* fenix: The firmware for the spaceblimp. Designed around the V4 LoRa32.

* fraublucher: The firmware for the ground station. Designed around the V3 LoRa32.

# Development Process

If you want to build and flash the spaceblimp firmware, use this command:

```shell
cargo run --bin fenix
```

The command for building and flashing the ground station is similar:

```shell
cargo run --bin fraublucher
```

If you want to build both without flashing either of them, then use this
command:

```shell
cargo run --bin fraublucher
```
