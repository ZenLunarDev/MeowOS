param(
    [switch]$Build,
    [switch]$Image,
    [switch]$Run,
    [switch]$All,
    [switch]$InstallWSL
)

$ErrorActionPreference = "Stop"

$ProjectDir = Join-Path $env:USERPROFILE "Documents\MewoOs"
$KernelDir = Join-Path $ProjectDir "kernel"
$EspDir = Join-Path $ProjectDir "esp"
$EfiFile = Join-Path $ProjectDir "target\x86_64-unknown-uefi\debug\mewoos.efi"
$ImageFile = Join-Path $ProjectDir "mewoos.img"

$Wsl = "wsl"
$WslProjectDir = & $Wsl wslpath -u $ProjectDir 2>$null
if ($LASTEXITCODE -ne 0) {
    $WslProjectDir = "/mnt/c/Users/กรมท/Documents/MewoOs"
}
$OvmfCode = "/usr/share/OVMF/OVMF_CODE.fd"

function Test-WSL {
    $result = & $Wsl --status 2>&1
    return $LASTEXITCODE -eq 0
}

function New-Kernel {
    Write-Host "==> Building kernel..." -ForegroundColor Cyan
    $toolchainBin = Join-Path $env:USERPROFILE ".rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin"
    $env:PATH = "$toolchainBin;$env:PATH"
    Set-Location -LiteralPath $KernelDir
    cargo +nightly build --target x86_64-unknown-uefi
    if (-not (Test-Path $EfiFile)) {
        throw "Build failed - EFI binary not found at $EfiFile"
    }
    Write-Host "    Built: $EfiFile" -ForegroundColor Green
}

function New-BootImage {
    Write-Host "==> Creating bootable disk image..." -ForegroundColor Cyan
    
    if (-not (Test-Path $EfiFile)) {
        throw "EFI binary not found. Run with -Build first."
    }
    
    if (-not (Test-Path $EspDir)) {
        New-Item -ItemType Directory -Path $EspDir -Force | Out-Null
    }
    
    $bootDir = Join-Path $EspDir "EFI\BOOT"
    if (-not (Test-Path $bootDir)) {
        New-Item -ItemType Directory -Path $bootDir -Force | Out-Null
    }
    
    Write-Host "    Copying EFI binary to ESP..." 
    Copy-Item -Force $EfiFile (Join-Path $bootDir "BOOTX64.EFI")
    
    Write-Host "    Creating 64MB FAT32 disk image using WSL..." 
    
    if (-not (Test-WSL)) {
        throw "WSL2 not available. Run this script with -InstallWSL to install it."
    }
    
    if (-not (Test-Path $OvmfCode)) {
        Write-Host "    Installing OVMF firmware in WSL2..." -ForegroundColor Yellow
        & $Wsl bash -c "sudo apt-get update -qq && sudo apt-get install -y -qq ovmf" 2>$null
    }
    
    if (Test-Path $ImageFile) {
        Remove-Item $ImageFile -Force
    }
    
    $wslImg = Join-Path $WslProjectDir "mewoos.img"
    
    & $Wsl bash -c "dd if=/dev/zero of='$wslImg' bs=1M count=64" 2>$null
    & $Wsl bash -c "mkfs.fat -F 32 '$wslImg'" 2>$null
    
    $espWsl = Join-Path $WslProjectDir "esp/EFI/BOOT/BOOTX64.EFI"
    & $Wsl bash -c "mkdir -p /mnt/tmp_mewoos && sudo mount -o loop '$wslImg' /mnt/tmp_mewoos && mkdir -p /mnt/tmp_mewoos/EFI/BOOT && cp '$espWsl' /mnt/tmp_mewoos/EFI/BOOT/ && sudo umount /mnt/tmp_mewoos && rm -rf /mnt/tmp_mewoos" 2>$null
    
    if (-not (Test-Path $ImageFile)) {
        throw "Failed to create disk image"
    }
    
    $size = [math]::Round((Get-Item $ImageFile).Length / 1MB, 1)
    Write-Host "    Image created: $ImageFile ($size MB)" -ForegroundColor Green
}

function Start-QEMU {
    param([string]$ImagePath)
    
    Write-Host "==> Starting QEMU..." -ForegroundColor Cyan
    
    if (-not (Test-Path $ImagePath)) {
        throw "Image not found at $ImagePath"
    }
    
    if (-not (Test-WSL)) {
        Write-Host "    ERROR: WSL2 not available." -ForegroundColor Red
        return
    }
    
    if (-not (& $Wsl test -f $OvmfCode 2>$null)) {
        Write-Host "    ERROR: OVMF firmware not found in WSL2." -ForegroundColor Red
        Write-Host "    Install with: wsl bash -c 'sudo apt-get install ovmf'" 
        return
    }
    
    Write-Host "    Booting: $ImagePath" 
    Write-Host "    Config: 512MB RAM, UEFI" 
    Write-Host ""
    Write-Host "    Controls:" -ForegroundColor Gray
    Write-Host "      Ctrl+Alt+G  - release mouse" -ForegroundColor Gray
    Write-Host "      Ctrl+Alt+Q  - quit QEMU" -ForegroundColor Gray
    Write-Host ""
    
    $wslImg = Join-Path $WslProjectDir "mewoos.img"
    & $Wsl bash -c "qemu-system-x86_64 -bios /usr/share/OVMF/OVMF_CODE.fd -drive format=raw,file='$wslImg' -m 512M -no-reboot" 2>&1
}

function Install-WSL {
    Write-Host "==> Installing WSL2 + Ubuntu..." -ForegroundColor Cyan
    Write-Host "    This may require a restart." -ForegroundColor Yellow
    
    & $Wsl --install -d Ubuntu --no-launch 2>&1
    Write-Host ""
    Write-Host "    WSL2 installation complete. Restart your computer if prompted." -ForegroundColor Green
    Write-Host "    After restart, run: wsl" 
    Write-Host "    Then inside WSL: sudo apt-get install ovmf qemu-system-x86" 
}

if ($All -or $Image -or $Run) {
    if (-not (Test-WSL)) {
        Write-Host "WSL2 not found. Installing automatically..." -ForegroundColor Yellow
        Install-WSL
        exit 0
    }
}

if ($All) {
    New-Kernel
    New-BootImage
    Start-QEMU -ImagePath $ImageFile
} elseif ($Build) {
    New-Kernel
} elseif ($Image) {
    New-BootImage
} elseif ($Run) {
    if (-not (Test-Path $ImageFile)) {
        Write-Host "Image not found. Auto-creating..." -ForegroundColor Yellow
        New-BootImage
    }
    Start-QEMU -ImagePath $ImageFile
} elseif ($InstallWSL) {
    Install-WSL
} else {
    Write-Host ""
    Write-Host "MeowOS - Build & Boot Tool" -ForegroundColor Cyan
    Write-Host "========================" 
    Write-Host ""
    Write-Host "Usage: .\boot.ps1 [-Build|-Image|-Run|-All|-InstallWSL]"
    Write-Host ""
    Write-Host "  -Build      Build the kernel EFI binary"
    Write-Host "  -Image      Create bootable disk image (mewoos.img)"
    Write-Host "  -Run        Boot the image in QEMU"
    Write-Host "  -All        Build + Image + Run (complete flow)"
    Write-Host "  -InstallWSL Install WSL2 + Ubuntu (for disk image creation)"
    Write-Host ""
    Write-Host "Quick start:" 
    Write-Host "  .\boot.ps1 -All" -ForegroundColor Yellow
    Write-Host ""
}
