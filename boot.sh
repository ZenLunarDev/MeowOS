#!/bin/bash
set -e

cd /mnt/c/Users/กรมท/Documents/MewoOs

echo "==> Building release binary..."
cargo +nightly build --target x86_64-unknown-uefi --release 2>/dev/null || \
cargo build --target x86_64-unknown-uefi --release

echo "==> Copying to ESP..."
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/release/mewoos.efi esp/EFI/BOOT/BOOTX64.EFI

echo "==> Booting QEMU..."
qemu-system-x86_64 -display sdl \
  -bios /usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M
