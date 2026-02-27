# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

termscp is a terminal file transfer client with a TUI (Terminal User Interface), supporting SFTP, SCP, FTP/FTPS, Kube, S3, SMB, and WebDAV protocols. It features a dual-pane file explorer (local + remote), bookmarks, system keyring integration, file watching/sync, an embedded terminal, and customizable themes.

- **Language**: Rust (edition 2024, MSRV 1.89.0)
- **UI Framework**: tuirealm v3 (built on crossterm)
- **File Transfer**: remotefs ecosystem

## Build & Development Commands

```bash
# Build
cargo build
cargo build --release
cargo build --no-default-features              # minimal build without SMB/keyring

# Test (CI-equivalent)
cargo test --no-default-features --features github-actions --no-fail-fast

# Run a single test
cargo test <test_name> -- --nocapture

# Run tests for a module
cargo test --lib filetransfer::
cargo test --lib config::params::tests

# Lint
cargo clippy -- -Dwarnings

# Format
cargo fmt --all -- --check      # check only
cargo fmt --all                 # fix
```

### System Dependencies (for building)

- **Linux**: `libdbus-1-dev`, `libsmbclient-dev`
- **macOS**: `pkg-config`, `samba` (brew, with force link)

## Feature Flags

- **`smb`** (default): SMB/Samba protocol support
- **`keyring`** (default): System keyring integration for password storage
- **`smb-vendored`**: Vendored SMB library (for static builds)
- **`github-actions`**: CI flag — disables real keyring in tests, uses file-based storage
- **`isolated-tests`**: For parallel test isolation

## Architecture

### Application Lifecycle

```
main.rs → parse CLI args → ActivityManager::new() → ActivityManager::run()
                                                        ↓
                                              Activity loop (draw → poll → update)
                                              ├── AuthActivity (login/bookmarks)
                                              ├── FileTransferActivity (dual-pane explorer)
                                              └── SetupActivity (configuration)
```

`ActivityManager` owns a `Context` that is passed between activities. Each activity takes ownership of the Context on `on_create()` and returns it on `on_destroy()`.

### Key Modules

| Module | Path | Purpose |
|--------|------|---------|
| **activity_manager** | `src/activity_manager.rs` | Orchestrates activity lifecycle and transitions |
| **ui/activities** | `src/ui/activities/{auth,filetransfer,setup}/` | Three main screens, each implementing the `Activity` trait |
| **ui/context** | `src/ui/context.rs` | Shared `Context` struct (terminal, config, bookmarks, theme) |
| **filetransfer** | `src/filetransfer/` | Protocol enum, `RemoteFsBuilder`, connection parameters |
| **host** | `src/host/` | `HostBridge` trait — abstracts local (`Localhost`) and remote (`RemoteBridged`) file operations |
| **explorer** | `src/explorer/` | `FileExplorer` — directory navigation, sorting, filtering, transfer queue |
| **system** | `src/system/` | `BookmarksClient`, `ConfigClient`, `ThemeProvider`, `SshKeyStorage`, `KeyStorage` trait |
| **config** | `src/config/` | TOML-based serialization for themes, bookmarks, user params |

### Core Traits

- **`Activity`** (`src/ui/activities/mod.rs`): `on_create`, `on_draw`, `will_umount`, `on_destroy` — UI screen lifecycle
- **`HostBridge`** (`src/host/bridge.rs`): Unified file operations interface (connect, list_dir, open_file, mkdir, remove, rename, copy, etc.)
- **`KeyStorage`** (`src/system/keys/mod.rs`): `get_key`/`set_key` — password storage abstraction (keyring or encrypted file fallback)

### Conditional Compilation

The `build.rs` defines cfg aliases via `cfg_aliases`:
- `posix`, `macos`, `linux`, `win` — platform shortcuts
- `smb`, `smb_unix`, `smb_windows` — feature + platform combinations

Platform-specific dependencies: SSH and FTP crates use different TLS backends on Unix vs Windows. SMB support is completely gated behind the `smb` feature flag.

### File Transfer Protocols

`FileTransferProtocol` enum maps to protocol-specific parameter types (`ProtocolParams` enum) and `RemoteFsBuilder` constructs the appropriate `RemoteFs` client. Each protocol has its own params struct (e.g., `GenericProtocolParams` for SSH-based, `AwsS3Params`, `KubeProtocolParams`, `SmbParams`, `WebDAVProtocolParams`).

## Code Conventions

- **rustfmt**: `group_imports = "StdExternalCrate"`, `imports_granularity = "Module"`
- **Error handling**: Custom error types with `thiserror`, module-level Result aliases (e.g., `HostResult<T>`)
- **Builder pattern**: Used for `RemoteFsBuilder`, `HostBridgeBuilder`
- **Client pattern**: System services wrapped as clients (`BookmarksClient`, `ConfigClient`)
- **Tests**: Unit tests in `#[cfg(test)]` blocks within source files. Tests requiring serial execution use `#[serial]` from `serial_test`
- **Encryption**: Bookmark passwords encrypted with `magic-crypt`; keys stored in system keyring or encrypted file
