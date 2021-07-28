# Rust - GBA

## requirements

```
sudo apt install binutils-arm-none-eabi
rustup default nightly
rustup component add rust-src
cargo install cargo-xbuild
```

## build

```
arm-none-eabi-as crt0.s -o target/crt0.o
cargo xbuild --release --target thumbv4-none-agb.json
arm-none-eabi-objcopy -O binary target/thumbv4-none-agb/release/gba-demo target/rom.gba
```
