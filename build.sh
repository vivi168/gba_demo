#!/bin/sh

arm-none-eabi-as crt0.s -o target/crt0.o || exit 1
cargo xbuild --release --target thumbv4-none-agb.json || exit 1
arm-none-eabi-objcopy -O binary target/thumbv4-none-agb/release/gba-demo /mnt/c/Users/vbihl/Desktop/issou.gba || exit 1
