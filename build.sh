#!/bin/sh

mkdir -p target
arm-none-eabi-as crt0.s -o target/crt0.o || exit 1
cargo xbuild --release --target thumbv4t-none-eabi.json || exit 1
arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/gba-demo /mnt/c/Users/vbihl/Desktop/issou.gba || exit 1
