# MeowOS

<p align="center">
  <strong>100% Rust UEFI Kernel</strong><br>
  Minimal GUI + Interactive Shell + Screenshots<br>
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
  <a href="https://github.com/ZenLunarDev/MeowOS/releases">
    <img src="https://img.shields.io/badge/release-v0.2-orange" alt="Release">
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

Open laptop → black screen → **"Hello from Rust OS!"** appears → no Windows/Linux loading underneath. That's the **"I built my own world"** moment.

## Features

- **GUI Widgets**: Button, Checkbox, ProgressBar
- **Interactive Shell**: Type commands directly
- **Screenshot**: Save framebuffer to BMP
- **Memory Allocator**: Dynamic allocation in UEFI
- **Safe Rust**: 100% memory-safe kernel code

## Requirements

- Rust nightly (`rustup toolchain install nightly`)
- WSL2 + Ubuntu (for building and QEMU boot)
- QEMU (for testing)

## Build

```bash
# Add UEFI target
rustup target add x86_64-unknown-uefi --toolchain nightly

# Build kernel
cd kernel
cargo +nightly build --target x86_64-unknown-uefi --release
```

Output: `target/x86_64-unknown-uefi/release/mewoos.efi`

## Boot in QEMU

### Option 1: PowerShell script (Windows)
```powershell
.\boot.ps1 -All
```

### Option 2: WSL/Linux script
```bash
chmod +x boot.sh
./boot.sh
```

### Option 3: Manual WSL
```bash
cd /mnt/c/Users/กรมท/Documents/MewoOs

# Build
cargo +nightly build --target x86_64-unknown-uefi --release

# Copy to ESP
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/release/mewoos.efi esp/EFI/BOOT/BOOTX64.EFI

# Boot
qemu-system-x86_64 -display sdl \
  -bios /usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M
```

### Option 4: Direct ESP (no image)
```bash
qemu-system-x86_64 -display sdl \
  -machine q35,smm=on \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_VARS_4M.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M
```

## Shell Commands

Once booted, type:
- `help` - show commands
- `rect` - draw random rectangles
- `clear` - clear text console
- `cls` - clear framebuffer
- `gui` - show widget demo
- `mouse` - mouse status
- `shot` - save screenshot as `shot.bmp`
- `exit` - halt

## Stack

- Rust nightly
- [uefi](https://crates.io/crates/uefi) crate v0.36
- LLD linker
- `x86_64-unknown-uefi` target
- Release optimizations: LTO + size optimization (`opt-level = "s"`)

## Binary Size

| Build | Size |
|-------|------|
| Debug | ~126 KB |
| Release (LTO + opt-level=s) | **~43 KB** |

## Project Structure

```
MeowOS/
├── kernel/
│   ├── src/
│   │   ├── main.rs         # Entry point, widget demo
│   │   ├── framebuffer.rs  # GOP protocol, pixel/text drawing
│   │   ├── gui.rs          # Button, Checkbox, ProgressBar widgets
│   │   ├── mouse.rs        # Mouse driver stub
│   │   ├── allocator.rs    # UEFI pool-based memory allocator
│   │   ├── screenshot.rs   # BMP screenshot saver
│   │   └── shell.rs        # Interactive shell loop
│   ├── Cargo.toml
│   └── .cargo/
│       └── config.toml     # UEFI target + linker config
├── esp/
│   └── EFI/BOOT/BOOTX64.EFI
├── boot.ps1               # Windows build & boot script
├── boot.sh                # WSL/Linux build & boot script
├── Cargo.toml             # Workspace root
└── README.md
```

## License

MIT OR Apache-2.0

---

<p align="center">
  <em>Built with Rust. Boots on real metal. No OS required.</em>
</p>
