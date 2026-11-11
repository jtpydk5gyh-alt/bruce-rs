# firmware

Board binaries that build for real ESP32 targets. **Separate Cargo workspace** from the
root on purpose: these crates use `esp-hal` and only compile for the chip, so keeping them
out of the root workspace lets `cargo test --workspace` still run the host logic without a
board.

## Toolchain

Xtensa (ESP32-S3) needs the Espressif Rust toolchain:

```sh
cargo install espup --locked      # or download the prebuilt binary
espup install --targets esp32s3
. ~/export-esp.sh                  # once per shell
```

## Build

```sh
cd firmware
cargo build --release             # cross-compiles for xtensa-esp32s3-none-elf
```

The target, linker args and `build-std` live in `firmware/.cargo/config.toml`.

## Flash (needs the board)

```sh
cargo install espflash --locked
cd firmware
cargo run --release -p tembed-cc1101   # flashes + opens the serial monitor
```

## Crates

| Crate | Board |
|-------|-------|
| `tembed-cc1101` | LilyGo T-Embed CC1101 (ESP32-S3) |
