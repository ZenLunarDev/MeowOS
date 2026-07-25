# MeowOS

<p align="center">
  <strong>100% Rust UEFI Kernel</strong><br>
  Minimal GUI + Interactive Shell<br>
  Boot it. <em>No Windows/Linux underneath.</em>
</p>

<p align="center">
  <a href="https://www.rust-lang.org">
    <img src="https://img.shields.io/badge/Rust-nightly-red?logo=rust" alt="Rust">
  </a>
  <a href="https://uefi.org">
    <img src="https://img.shields.io/badge/UEFI-x86__64-blue" alt="UEFI">
  </a>
  <a href="https://github.com/rust-osdev/uefi-rs">
    <img src="https://img.shields.io/badge/uefi--rs-0.36-green" alt="uefi-rs">
  </a>
</p>

---

## What is this?

A bootable OS kernel written **entirely in Rust**. It runs as a UEFI application — no Windows, no Linux underneath. When you boot it, you see colored rectangles drawn directly on the framebuffer and a text shell you can type into.

**Creator:** [ZenLunarDev](https://github.com/ZenLunarDev)

## Why Rust

- No null pointers
- No use-after-free
- Memory safety without garbage collection
- Compiler-enforced safety at the kernel level

## Boot Experience

Open laptop → black screen → **"Hello, MeowOS!"** appears → no Windows/Linux loading underneath. That's the **"I built my own world"** moment.

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- WSL2 + Ubuntu (for disk image creation)
- QEMU (for testing)

## Build

```bash
# Add UEFI target
rustup target add x86_64-unknown-uefi --toolchain nightly

# Build kernel
cd kernel
cargo +nightly build --target x86_64-unknown-uefi
```

Output: `target/x86_64-unknown-uefi/debug/mewoos.efi`

## Boot in QEMU

### Option 1: PowerShell script (Windows)
```powershell
# From project root
.\boot.ps1 -All
```

### Option 2: Manual (WSL2/Linux)
```bash
# Create bootable image
cd /mnt/c/Users/กรมท/Documents/MewoOs

# Mount ESP and copy EFI
sudo mkdir -p /mnt/tmp_mewoos
sudo mount -o loop mewoos.img /mnt/tmp_mewoos 2>/dev/null || true
mkdir -p /mnt/tmp_mewoos/EFI/BOOT
cp esp/EFI/BOOT/BOOTX64.EFI /mnt/tmp_mewoos/EFI/BOOT/
sudo umount /mnt/tmp_mewoos 2>/dev/null || true

# Boot
qemu-system-x86_64 -display sdl \
  -bios /usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M
```

### Option 3: Direct ESP folder (no image)
```bash
qemu-system-x86_64 -display sdl \
  -machine q35,smm=on \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_VARS_4M.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M
```

## Features

- Framebuffer graphics (RGB rectangles)
- Interactive text shell
- Safe Rust kernel code
- UEFI boot services integration

## Shell Commands

Once booted, type:
- `help` - show commands
- `rect` - draw random rectangles
- `clear` - clear text console
- `cls` - clear framebuffer
- `exit` - halt

## Stack

- Rust nightly
- [uefi](https://crates.io/crates/uefi) crate
- LLD linker
- `x86_64-unknown-uefi` target

## Project Structure

```
MeowOS/
├── kernel/
│   ├── src/
│   │   ├── main.rs       # Entry point, framebuffer init
│   │   ├── framebuffer.rs # GOP protocol, pixel drawing
│   │   └── shell.rs      # Interactive shell loop
│   ├── Cargo.toml
│   └── .cargo/
│       └── config.toml   # UEFI target + linker config
├── esp/
│   └── EFI/BOOT/BOOTX64.EFI
├── boot.ps1             # Build & boot script for Windows
├── Cargo.toml           # Workspace root
└── README.md
```

## License

MIT OR Apache-2.0

---

<p align="center">
  <em>Built with Rust. Boots on real metal. No OS required.</em>
</p>
