# 安装

termscp 可在多种平台上使用。请在下方选择与你的系统相匹配的安装方式。

## Linux、FreeBSD 与 macOS

下面的 shell 脚本只需一条命令即可在你的系统上安装 termscp：

```sh
curl --proto '=https' --tlsv1.2 -sSLf https://termscp.rs/install.sh | sh
```

在 macOS 上，安装需要 [Homebrew](https://brew.sh/)；否则将安装 Rust 编译器以从源码构建 termscp。

## Windows

只需一条命令即可在 PowerShell 中安装 termscp：

```ps
irm https://termscp.rs/install.ps1 | iex
```

或者，使用 [Chocolatey](https://chocolatey.org/) 安装：

```ps
choco install termscp
```

## NetBSD

从官方仓库安装 termscp：

```sh
pkgin install termscp
```

## Arch Linux

从官方仓库安装 termscp：

```sh
pacman -S termscp
```

## 系统要求

运行 termscp 需要以下系统依赖。

- Linux 用户：
  - libdbus-1
  - pkg-config
  - libsmbclient
- FreeBSD 和 NetBSD 用户：
  - dbus
  - pkgconf
  - libsmbclient

### 可选依赖

运行 termscp 并不需要这些依赖，但要使用其全部功能则需要它们。

- Linux 和 FreeBSD 用户，若要通过 `V` 打开文件（以下至少需要一项）：
  - xdg-open
  - gio
  - gnome-open
  - kde-open
- Linux 用户：一个密钥环管理器。请在[密码安全](../configuration/password-security.md)页面了解更多。
- WSL 用户，若要通过 `V` 打开文件：
  - [wslu](https://github.com/wslutilities/wslu)

## 更新 termscp

要将 termscp 更新到最新版本，请在命令行中运行：

```sh
(sudo) termscp update
```

有关所有平台和安装方式，请参阅 <https://termscp.rs/install>。
