# AresOS QEMU x86_64 config — epoch 6 versioned script (ARCHITECTURE_TARGETS.md)
# Changelog: v1 — virtio-blk-pci + smp=2 for phases 141+

param(
    [string]$KernelImage = "target/x86_64-unknown-none/debug/bootimage-kernel.bin",
    [int]$Smp = 2
)

$ErrorActionPreference = "Stop"
$Repo = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent

Push-Location $Repo
try {
    & cargo build -p kernel
    & qemu-system-x86_64 `
        -drive "format=raw,file=$KernelImage" `
        -device virtio-blk-pci `
        -smp $Smp `
        -serial stdio `
        -display none `
        -no-reboot
} finally {
    Pop-Location
}
