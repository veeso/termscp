# Chocolatey How To

Just:

1. Calculate the SHA256 checksum for the latest release of ZIP files both for aarch64 and x86_64 versions.
2. Update checksums in `tools/chocolateyinstall.ps1`
3. run `choco pack`
4. run `choco push termscp.$VERSION.nupkg --source https://push.chocolatey.org/`
