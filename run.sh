#!/bin/sh

# Build kernel
cd kernel
cargo build --release
cd ..

# Build loader
cd boot
cargo build --release
cd ..

# Run qemu
./tools/qemu/run_qemu.sh \
    ./boot/target/x86_64-unknown-uefi/release/kernel_loader.efi \
    ./kernel/kernel.elf
