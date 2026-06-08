$ErrorActionPreference = 'Stop';
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Detect arch: PROCESSOR_ARCHITECTURE is "ARM64" on ARM, "AMD64" on x64
$is_arm64 = $env:PROCESSOR_ARCHITECTURE -eq 'ARM64' -or $env:PROCESSOR_ARCHITEW6432 -eq 'ARM64'

if ($is_arm64) {
    $url      = 'https://github.com/veeso/termscp/releases/download/v1.1.1/termscp-v1.1.1-msvc.zip'
    $checksum = 'f6ad6c62f1578562f9af4bcee93bd4cc429cb52219c3636359b008db8789587e'
} else {
    $url      = 'https://github.com/veeso/termscp/releases/download/v1.1.1/termscp-v1.1.1-x86_64-pc-windows-msvc.zip'
    $checksum = 'd7796081b6f67b82acfa94557aa6852d12a33daab3cca6490660b20e42752005'
}

$packageArgs = @{
    packageName    = $env:ChocolateyPackageName
    fileType       = 'EXE'
    url            = $url
    unzipLocation  = $toolsDir
    softwareName   = 'termscp*'
    checksum       = $checksum
    checksumType   = 'sha256'
    validExitCodes = @(0, 3010, 1641)
}
Install-ChocolateyZipPackage @packageArgs
