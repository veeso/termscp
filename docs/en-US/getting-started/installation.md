# Installation

termscp is available for many platforms. Pick the method that matches your
system below.

## Linux, FreeBSD, and macOS

This shell script installs termscp on your system with a single command:

```sh
curl --proto '=https' --tlsv1.2 -sSLf https://termscp.rs/install.sh | sh
```

On macOS the installation requires [Homebrew](https://brew.sh/); otherwise the
Rust compiler is installed to build termscp from source.

## Windows

Install termscp from PowerShell with a single command:

```ps
irm https://termscp.rs/install.ps1 | iex
```

Alternatively, install it with [Chocolatey](https://chocolatey.org/):

```ps
choco install termscp
```

## NetBSD

Install termscp from the official repositories:

```sh
pkgin install termscp
```

## Arch Linux

Install termscp from the official repositories:

```sh
pacman -S termscp
```

## Requirements

The following system dependencies are required to run termscp.

- Linux users:
  - libdbus-1
  - pkg-config
  - libsmbclient
- FreeBSD and NetBSD users:
  - dbus
  - pkgconf
  - libsmbclient

### Optional requirements

These dependencies are not required to run termscp, but they are needed to
enjoy all of its features.

- Linux and FreeBSD users, to open files via `V` (at least one of these):
  - xdg-open
  - gio
  - gnome-open
  - kde-open
- Linux users: a keyring manager. Read more in the
  [Password security](../configuration/password-security.md) page.
- WSL users, to open files via `V`:
  - [wslu](https://github.com/wslutilities/wslu)

## Updating termscp

To update termscp to the latest version, run it from the command line with:

```sh
(sudo) termscp update
```

For all platforms and methods, see <https://termscp.rs/install>.
