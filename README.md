# MeowOS

**100% Rust UEFI Kernel** with minimal GUI and interactive shell.

## What this is

A bootable OS kernel written entirely in Rust. It runs as a UEFI application — no Windows, no Linux underneath. When you boot it, you see colored rectangles drawn directly on the framebuffer and a text shell you can type into.

**Creator:** ZenLunarDev

## Why Rust

- No null pointers
- No use-after-free
- Memory safety without garbage collection
- Compiler-enforced safety at the kernel level

## Requirements

- Windows machine with WSL2 (for Linux toolchain)
- Rust nightly (`rustup toolchain install nightly`)
- QEMU for testing

## Building

```bash
# Set up the nightly toolchain
rustup target add x86_64-unknown-uefi --toolchain nightly

# Build the kernel
cd kernel
cargo +nightly build --target x86_64-unknown-uefi
```

Output: `target/x86_64-unknown-uefi/debug/mewoos.efi`

## Running in QEMU

```bash
# From WSL2 / Linux
qemu-system-x86_64 \
  -bios /usr/share/OVMF/OVMF_CODE.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M \
  -no-reboot
```

Or just double-click the `.efi` file on real UEFI hardware.

## Features

- Framebuffer graphics (RGB rectangles)
- Interactive text shell
- Safe Rust kernel code
- UEFI boot services integration

## Commands

Once booted, type:
- `help` - show commands
- `rect` - draw random rectangles
- `clear` - clear text console
- `cls` - clear framebuffer
- `exit` - halt

## Stack

- Rust nightly
- uefi crate
- LLD linker
- x86_64-unknown-uefi target

---

*Boot it. See "Hello from Rust OS!" on bare metal. No OS required.*
