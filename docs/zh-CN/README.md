# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" alt="termscp logo" width="256" height="256" />
</p>

<p align="center">~ 功能丰富的终端文件传输工具 ~</p>
<p align="center">
  <a href="https://termscp.rs" target="_blank">网站</a>
  ·
  <a href="https://termscp.rs/install" target="_blank">安装</a>
  ·
  <a href="https://docs.termscp.rs" target="_blank">用户手册</a>
</p>

<p align="center">
  <a
    href="https://github.com/veeso/termscp/blob/main/README.md"
    ><img
      height="20"
      src="/assets/images/flags/gb.png"
      alt="English"
  /></a>
</p>

<p align="center">由 <a href="https://veeso.me/" target="_blank">@veeso</a> 开发</p>
<p align="center">当前版本： 1.0.0 2026-04-18</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/termscp/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/termscp?style=flat"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/termscp"
    ><img
      src="https://img.shields.io/crates/d/termscp.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/termscp"
    ><img
      src="https://img.shields.io/crates/v/termscp.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/Linux/badge.svg"
      alt="Linux CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/MacOS/badge.svg"
      alt="MacOS CI"
  /></a>
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/Windows/badge.svg"
      alt="Windows CI"
  /></a>
</p>

---

## 关于 termscp 🖥

termscp 是一个功能丰富的终端文件浏览和传输工具，支持 SCP/SFTP/FTP/Kube/S3/WebDAV。 简而言之，它是一个带有 TUI 的终端工具，可以连接到远程服务器进行文件的检索和上传，并能够与本地文件系统进行交互。 它兼容 **Linux**、**MacOS**、**FreeBSD**、**NetBSD** 和 **Windows** 操作系统。

![Explorer](assets/images/explorer.gif)

---

## 特性 🎁

- 📁  支持多种通信协议
  - **SFTP**
  - **SCP**
  - **FTP** 和 **FTPS**
  - **Kube**
  - **S3**
  - **SMB**
  - **WebDAV**
- 🖥  使用便捷的 UI 在远程和本地文件系统上浏览和操作
  - 创建、删除、重命名、搜索、查看和编辑文件
- ⭐  通过“内置书签”和“最近连接”快速连接到您喜爱的主机
- 📝  使用您喜欢的应用程序查看和编辑文件
- 💁  使用 SSH 密钥和用户名/密码进行 SFTP/SCP 身份验证
- 🐧  兼容 Windows、Linux、FreeBSD、NetBSD 和 MacOS 操作系统
- 🐚  内置终端，可在系统上执行命令。
- 🎨  丰富的个性化设置！
  - 主题
  - 自定义文件浏览器格式
  - 可自定义的文本编辑器
  - 可自定义的文件排序
  - 以及许多其他参数...
- 📫  传输大文件时通过桌面通知获得提醒
- 🔭  与远程主机文件更改保持同步
- 🔐  将密码保存在操作系统密钥保管库中
- 🦀  由 Rust 提供强力支持
- 👀  开发时更注重性能
- 🦄  频繁的精彩更新

---

## 开始 🚀

如果您正在考虑安装 termscp，我想对您表示感谢 💜 ！ 希望您会喜欢 termscp！  
如果您想为此项目做出贡献，请不要忘记查看我们的[贡献指南](CONTRIBUTING.md)。

如果您是 Linux、FreeBSD 或 MacOS 用户，使用以下简单的 shell 脚本即可通过单行指令在您的系统上安装 termscp：

```sh
curl --proto '=https' --tlsv1.2 -sSLf https://termscp.rs/install.sh | sh
```

> ❗ MacOS 安装需要 [Homebrew](https://brew.sh/)，否则将会安装 Rust 编译器

如果您是 Windows 用户，则可以在 PowerShell 中通过单行指令安装 termscp：

```ps
irm https://termscp.rs/install.ps1 | iex
```

或者，使用 [Chocolatey](https://chocolatey.org/)：

```ps
choco install termscp
```

NetBSD 用户可以从官方仓库安装 termscp。

```sh
pkgin install termscp
```

Arch Linux 用户可以从官方仓库安装 termscp。

```sh
pacman -S termscp
```

如需更多信息或其他平台支持，请访问 [termscp.rs](https://termscp.rs/install) 查看所有安装方法。

⚠️ 如果您想了解如何更新 termscp，只需从 CLI 运行 termscp： `(sudo) termscp update` ⚠️

### 依赖 ❗

- **Linux** 用户：
  - libdbus-1
  - pkg-config
  - libsmbclient
- **FreeBSD** 或 **NetBSD** 用户：
  - dbus
  - pkgconf
  - libsmbclient

### 可选依赖 ✔️

这些依赖并非运行 termscp 的强制要求，但有助于享受其全部功能

- **Linux/FreeBSD** 用户：
  - 用 `V` **打开**文件（至少其中之一）
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- **Linux** 用户：
  - 密钥环管理器：在[用户手册](https://docs.termscp.rs/zh-CN/configuration/password-security.html#linux-密钥环)中阅读更多内容
- **WSL** 用户
  - 用 `V` **打开**文件（至少其中之一）
    - [wslu](https://github.com/wslutilities/wslu)

---

## 支持开发者 ☕

如果您喜欢 termscp 并且感激我所做的工作，请考虑给予一点捐赠 🥳

您可以通过以下平台之一进行捐赠：

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## 用户手册 📚

用户手册可以在 [termscp 文档网站](https://docs.termscp.rs)上找到。

---

## 即将推出的功能 🧪

请查看 [Milestones](https://github.com/veeso/termscp/milestones)

---

## 贡献和问题 🤝🏻

欢迎贡献、bug 报告、新功能和问题！ 😉
如果您有任何问题或困惑，或者想建议新功能，或者只是想改进 termscp，请随时打开 issue 或 PR。

一个**值得赞赏**的贡献是将用户手册和 README 翻译成**其他语言**

请遵循[我们的贡献指南](CONTRIBUTING.md)

---

## 更新日志 ⏳

查看 termscp 的更新日志[点此](CHANGELOG.md)

---

## 支持 💪

termscp 由这些很棒的项目提供支持：

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [keyring-rs](https://github.com/hwchen/keyring-rs)
- [kube](https://github.com/kube-rs/kube)
- [open-rs](https://github.com/Byron/open-rs)
- [pavao](https://github.com/veeso/pavao)
- [remotefs](https://github.com/veeso/remotefs-rs)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [self_update](https://github.com/jaemk/self_update)
- [ratatui](https://github.com/ratatui-org/ratatui)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## 演示 🎬

> termscp 首页

![Auth](assets/images/auth.gif)

> 书签

![Bookmarks](assets/images/bookmarks.gif)

> 设置

![Setup](assets/images/config.gif)

> 文本编辑器

![TextEditor](assets/images/text-editor.gif)

---

## 许可协议 📃

termscp 使用 MIT 许可证授权。

您可以阅读完整的[许可证](LICENSE)
