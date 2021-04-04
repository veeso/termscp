# Deploy checklist

Document audience: project maintainers

- [Deploy checklist](#deploy-checklist)
  - [Description](#description)
  - [Checklist](#checklist)

## Description

This document describes the checklist that must be fulfilled before releasing a new version of termscp.

## Checklist

- [ ] The latest build didn't report any error in the CI
- [ ] All commands when using SFTP work
- [ ] All commands when using SCP work
- [ ] All commands when using FTP work
- [ ] It is possible to load bookmarks
- [ ] Recent connections get saved
- [ ] Update versions and release date in readme, changelog and cargo.toml
- [ ] Build on MacOS
- [ ] Update sha256 and version on homebrew repository
- [ ] Build on Windows
- [ ] Update sha256 and version in chocolatey repository
- [ ] Create chocolatey package
- [ ] Build Linux version using docker from `dist/build/build.sh`
- [ ] Update sha256 and version in AUR files
- [ ] Create release and attach the following artifacts
  - [ ] Deb package
  - [ ] RPM package
  - [ ] MacOs tar.gz
  - [ ] Windows nupkg
  - [ ] Windows zip
  - [ ] AUR tar.gz
