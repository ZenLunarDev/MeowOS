# MeowOS

A bootable OS kernel written in Rust. Runs as a UEFI application — no Windows, no Linux underneath.

**Author:** [ZenLunarDev](https://github.com/ZenLunarDev)

---

## What it is

MeowOS is an x86_64 UEFI kernel built with Rust. It boots directly from firmware, draws to the framebuffer, and gives you a small interactive shell.

The goal is to have a small, safe, self-contained kernel that on real hardware shows: black screen → `"Hello from Rust OS!"` — without Windows or Linux running underneath.

## Screenshot

Saved via shell command `shot` → `shot.bmp`

## Requirements

- Rust nightly
- WSL2 + Ubuntu or Linux
- QEMU
- UEFI firmware or OVMF

## Quick start

```bash
# 1. Clone
git clone https://github.com/ZenLunarDev/MeowOS.git
cd MeowOS

# 2. Build
cargo +nightly build --target x86_64-unknown-uefi --release

# 3. Boot in QEMU
qemu-system-x86_64 -display sdl \
  -bios /usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M
```

## How to use

### Build

```bash
cd kernel
cargo +nightly build --target x86_64-unknown-uefi
```

Output:
- debug: `kernel/target/x86_64-unknown-uefi/debug/mewoos.efi`
- release: `kernel/target/x86_64-unknown-uefi/release/mewoos.efi`

### Boot from WSL / Linux

```bash
# option A: direct ESP folder, no disk image
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/release/mewoos.efi esp/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 -display sdl \
  -bios /usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive format=raw,file=fat:rw:esp \
  -m 512M
```

```bash
# option B: manual disk image
dd if=/dev/zero of=mewoos.img bs=1M count=64
mkfs.fat -F 32 mewoos.img

mkdir -p /mnt/tmp_mewoos
sudo mount -o loop mewoos.img /mnt/tmp_mewoos
mkdir -p /mnt/tmp_mewoos/EFI/BOOT
cp esp/EFI/BOOT/BOOTX64.EFI /mnt/tmp_mewoos/EFI/BOOT/
sudo umount /mnt/tmp_mewoos

qemu-system-x86_64 -display sdl \
  -bios /usr/share/OVMF/OVMF_CODE_4M.fd \
  -drive format=raw,file=mewoos.img \
  -m 512M
```

### Boot from Windows

Use `boot.ps1` from the project root:

```powershell
# build only
.\boot.ps1 -Build

# create image only
.\boot.ps1 -Image

# boot only
.\boot.ps1 -Run

# build + image + boot
.\boot.ps1 -All
```

If your `cd` lands in a different folder, run from the folder that contains `boot.ps1`:

```powershell
cd C:\Users\กรมท\Documents\MewoOS
```

## Shell commands

After boot, the firmware loads `BOOTX64.EFI` and the shell starts automatically.

Type these commands:

- `help` — show help
- `rect` — draw random rectangles
- `clear` — clear the text console
- `cls` — clear the framebuffer
- `gui` — draw widget demo
- `mouse` — show mouse init status
- `shot` — save screenshot to `shot.bmp`
- `exit` — halt

## Troubleshooting

**QEMU can't find OVMF**

```bash
sudo apt-get install ovmf
find /usr/share -name "OVMF_CODE_4M.fd"
```

If the file is elsewhere, replace the `-bios` path accordingly.

**Mount fails**

```bash
sudo mkdir -p /mnt/tmp_mewoos
sudo mount -o loop mewoos.img /mnt/tmp_mewoos
```

Use `sudo umount /mnt/tmp_mewoos` to unmount.

**Keyboard input not working**

QEMU window must be focused. Click inside the VM window before typing shell commands.

## Stack

- Rust nightly
- `uefi` crate
- LLD linker
- `x86_64-unknown-uefi`

## License

Proprietary. All rights reserved. See [LICENSE](LICENSE) for details.
