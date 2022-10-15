$ErrorActionPreference = 'Stop';

if ($args.Count -eq 0) {
  Write-Output "Usage: windows.ps1 <version>"
  exit 1
}

$version = $args[0]

# Go to root directory
Set-Location ..\..\
# Build
cargo build --release
# Make zip
$zipName = "termscp-v$version-x86_64-pc-windows-msvc.zip"
Set-Location .\target\release\
Compress-Archive -Force termscp.exe $zipName
# Get checksum
Get-FileHash $zipName
Move-Item $zipName .\..\..\dist\pkgs\windows\$zipName
