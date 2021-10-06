# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" width="256" height="256" />
</p>

<p align="center">~ 功能丰富的终端文件传输 ~</p>
<p align="center">
  <a href="https://veeso.github.io/termscp/" target="_blank">网站</a>
  ·
  <a href="https://veeso.github.io/termscp/#get-started" target="_blank">安装</a>
  ·
  <a href="https://veeso.github.io/termscp/#user-manual" target="_blank">用户手册</a>
</p>

<p align="center">
  <a href="https://github.com/veeso/termscp"
    ><img
      height="20"
      src="/assets/images/flags/us.png"
      alt="English"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/de/README.md"
    ><img
      height="20"
      src="/assets/images/flags/de.png"
      alt="Deutsch"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/es/README.md"
    ><img
      height="20"
      src="/assets/images/flags/es.png"
      alt="Español"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/fr/README.md"
    ><img
      height="20"
      src="/assets/images/flags/fr.png"
      alt="Français"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/it/README.md"
    ><img
      height="20"
      src="/assets/images/flags/it.png"
      alt="Italiano"
  /></a>
  &nbsp;
  <a
    href="https://github.com/veeso/termscp/blob/main/docs/zh-CN/README.md"
    ><img
      height="20"
      src="/assets/images/flags/cn.png"
      alt="简体中文"
  /></a>
</p>

<p align="center">由 <a href="https://veeso.github.io/" target="_blank">@veeso</a> 开发</p>
<p align="center">当前版本： 0.7.0 (12/10/2021)</p>

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![Stars](https://img.shields.io/github/stars/veeso/termscp.svg)](https://github.com/veeso/termscp) [![Downloads](https://img.shields.io/crates/d/termscp.svg)](https://crates.io/crates/termscp) [![Crates.io](https://img.shields.io/badge/crates.io-v0.7.0-orange.svg)](https://crates.io/crates/termscp)  

[![Linux](https://github.com/veeso/termscp/workflows/Linux/badge.svg)](https://github.com/veeso/termscp/actions) [![MacOs](https://github.com/veeso/termscp/workflows/MacOS/badge.svg)](https://github.com/veeso/termscp/actions) [![Windows](https://github.com/veeso/termscp/workflows/Windows/badge.svg)](https://github.com/veeso/termscp/actions) [![FreeBSD](https://github.com/veeso/termscp/workflows/FreeBSD/badge.svg)](https://github.com/veeso/termscp/actions) [![Coverage Status](https://coveralls.io/repos/github/veeso/termscp/badge.svg)](https://coveralls.io/github/veeso/termscp) [![Docs](https://docs.rs/termscp/badge.svg)](https://docs.rs/termscp)

---

## 关于 termscp 🖥

termscp 是一个功能丰富的终端文件传输和浏览器，支持 SCP/SFTP/FTP/S3。 所以基本上是一个带有 TUI 的终端实用程序，用于连接到远程服务器以检索和上传文件并与本地文件系统进行交互。
它与 **Linux**、**MacOS**、**FreeBSD** 和 **Windows** 兼容。

![Explorer](/assets/images/explorer.gif)

---

## Features 🎁

- 📁  不同的通讯协议
  - **SFTP**
  - **SCP**
  - **FTP** and **FTPS**
  - **Aws S3**
- 🖥  使用方便的 UI 在远程和本地机器文件系统上探索和操作
  - 创建、删除、重命名、搜索、查看和编辑文件
- ⭐  通过内置书签和最近的连接连接到您最喜欢的主机
- 📝  使用您喜欢的应用程序查看和编辑文件
- 💁  使用 SSH 密钥和用户名/密码进行 SFTP/SCP 身份验证
- 🐧  与 Windows、Linux、FreeBSD 和 MacOS 兼容
- 🎨  让它成为你的！
  - 主题
  - 自定义文件浏览器格式
  - 可定制的文本编辑器
  - 可定制的文件排序
  - 和许多其他参数...
- 📫  传输大文件时通过桌面通知获得通知
- 🔐  将密码保存在操作系统密钥保管库中
- 🦀  Rust 动力
- 👀  开发时注意性能
- 🦄  频繁的精彩更新

---

## 开始 🚀

如果您正在考虑安装termscp，我要感谢您💜！ 我希望你会喜欢termscp！  
如果您想为此项目做出贡献，请不要忘记查看我们的贡献指南。 [阅读更多](../../CONTRIBUTING.md)

如果您是 Linux、FreeBSD 或 MacOS 用户，这个简单的 shell 脚本将使用单个命令在您的系统上安装 termscp：

```sh
curl --proto '=https' --tlsv1.2 -sSLf "https://git.io/JBhDb" | sh
```

如果您是 Windows 用户，则可以使用 [Chocolatey](https://chocolatey.org/) 安装 termscp：

```sh
choco install termscp
```

如需更多信息或其他平台，请访问 [veeso.github.io](https://veeso.github.io/termscp/#get-started) 查看所有安装方法。

⚠️ 如果您正在寻找如何更新 termscp 只需从 CLI 运行 termscp ： `(sudo) termscp --update` ⚠️

### 要求 ❗

- **Linux** 用户:
  - libssh
  - libdbus-1
  - pkg-config
- **FreeBSD** 用户:
  - libssh
  - dbus
  - pkgconf

### 可选要求 ✔️

这些要求不是运行 termscp 的强制要求，而是要享受它的所有功能

- **Linux/FreeBSD** 用户:
  - 用 `V` **打开** 文件（至少其中之一）
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- **Linux** 用户:
  - keyring manager: [在用户手册中阅读更多内容](man.md#linux-keyring)
- **WSL** 用户
  - 用 `V` **打开** 文件（至少其中之一）
    - [wslu](https://github.com/wslutilities/wslu)

---

## 支持我 ☕

如果您喜欢 termscp 并且希望看到该项目不断发展和改进，请考虑在**给我买杯咖啡**上捐款以支持我🥳

[![给我买一杯咖啡](https://img.buymeacoffee.com/button-api/?text=%E7%BB%99%E6%88%91%E4%B9%B0%E4%B8%80%E6%9D%AF%E5%92%96%E5%95%A1&emoji=&slug=veeso&button_colour=404040&font_colour=ffffff&font_family=Comic&outline_colour=ffffff&coffee_colour=FFDD00)](https://www.buymeacoffee.com/veeso)

---

## 用户手册和文档 📚

用户手册可以在[termscp的网站](https://veeso.github.io/termscp/#user-manual)上找到 或者在[Github](man.md)上。

开发者文档可以在 <https://docs.rs/termscp> 的 Rust Docs 上找到。

---

## 已知的问题 🧻

- `NoSuchFileOrDirectory` 连接时 (WSL1): 我知道这个问题，我猜这是 WSL 的一个小故障。 别担心，只需将 termscp 可执行文件移动到另一个 PATH 位置，例如`/usr/bin`，或者通过适当的包格式（例如 deb）安装它。

---

## 贡献和问题 🤝🏻

欢迎贡献、错误报告、新功能和问题！ 😉
如果您有任何问题或疑虑，或者您想建议新功能，或者您只想改进termscp，请随时打开问题或 PR。

请遵循 [我们的贡献指南](../../CONTRIBUTING.md)

---

## 变更日志 ⏳

查看termscp的更新日志 [这里](../../CHANGELOG.md)

---

## Powered by 💪

termscp 由这些很棒的项目提供支持：

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [keyring-rs](https://github.com/hwchen/keyring-rs)
- [open-rs](https://github.com/Byron/open-rs)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [rust-s3](https://github.com/durch/rust-s3)
- [self_update](https://github.com/jaemk/self_update)
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
- [suppaftp](https://github.com/veeso/suppaftp)
- [textwrap](https://github.com/mgeisler/textwrap)
- [tui-rs](https://github.com/fdehau/tui-rs)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## 画廊 🎬

> 家

![Auth](/assets/images/auth.gif)

> 书签

![Bookmarks](/assets/images/bookmarks.gif)

> 设置

![Setup](/assets/images/config.gif)

> 文本编辑器

![TextEditor](/assets/images/text-editor.gif)

---

## 执照 📃

“termscp”在 MIT 许可下获得许可。

您可以阅读整个许可证 [这里](../../LICENSE)
