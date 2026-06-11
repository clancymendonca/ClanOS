# Launch AresOS with visible VGA desktop + serial shell.
$ErrorActionPreference = "Stop"
$env:Path = "C:\Program Files\qemu;" + $env:Path
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "Building bootimage..."
cargo bootimage -p kernel | Out-Host
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$Image = Join-Path $Root "target\x86_64-unknown-none\debug\bootimage-kernel.bin"
Write-Host "Starting QEMU (serial shell in this window, desktop in QEMU window)..."
qemu-system-x86_64 `
    -drive format=raw,file=$Image `
    -serial stdio `
    -display default `
    -vga std `
    -no-reboot
