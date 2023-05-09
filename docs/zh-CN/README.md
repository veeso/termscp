# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" width="256" height="256" />
</p>

<p align="center">~ 功能丰富的终端文件传输工具 ~</p>
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
      src="/assets/images/flags/gb.png"
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
<p align="center">当前版本： 0.12.0 (19/04/2023)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/termscp/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/termscp.svg"
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
  <a href="https://github.com/veeso/termscp/actions"
    ><img
      src="https://github.com/veeso/termscp/workflows/FreeBSD/badge.svg"
      alt="FreeBSD CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/termscp"
    ><img
      src="https://coveralls.io/repos/github/veeso/termscp/badge.svg"
      alt="Coveralls"
  /></a>
</p>

---

## 关于 termscp 🖥

termscp 是一个功能丰富的终端文件浏览和传输工具，支持 SCP/SFTP/FTP/S3。 作为一个带有 TUI 的命令行工具，它可以连接到远程服务器进行文件检索和上传，并能够与本地文件系统进行交互。

兼容 **Linux**、**MacOS**、**FreeBSD** 和 **Windows** 操作系统。

![Explorer](/assets/images/explorer.gif)

---

## 特性 🎁

- 📁  支持多种通信协议
  - **SFTP**
  - **SCP**
  - **FTP** and **FTPS**
  - **S3**
- 🖥  使用便捷的 UI 在远程和本地文件系统上浏览和操作
  - 创建、删除、重命名、搜索、查看和编辑文件
- ⭐  通过“内置书签”和“最近连接”快速连接到您的主机
- 📝  使用您喜欢的应用程序查看和编辑文件
- 💁  使用 SSH 密钥和用户名/密码进行 SFTP/SCP 身份验证
- 🐧  兼容 Windows、Linux、FreeBSD 和 MacOS 操作系统
- 🎨  丰富的个性化设置！
  - 主题
  - 自定义文件浏览器格式
  - 可选择的文本编辑器
  - 可选择的文件排序
  - 探索更多功能...
- 📫  传输大文件时通过桌面通知获得提醒
- 🔭  与远程主机文件更改保持同步
- 🔐  将密码保存在操作系统密钥保管库中
- 🦀  由 Rust 提供强力支持
- 👀  开发时更注重性能
- 🦄  快速且精彩迭代

---

## 开始 🚀

非常荣幸您能考虑安装termscp💜！ 希望你会喜欢termscp！  

如果您想为此项目做出贡献，请不要忘记查看我们的贡献指南。 [阅读更多](../../CONTRIBUTING.md)

如果您是 Linux、FreeBSD 或 MacOS 用户，使用以下简单的 shell 脚本通过单行指令在您的系统上安装 termscp：

```sh
curl -sSLf http://get-termscp.veeso.dev | sh
```

如果您是 Windows 用户，则可以使用 [Chocolatey](https://chocolatey.org/) 安装 termscp：

```sh
choco install termscp
```

如需更多信息或其他的平台支持，请访问 [veeso.github.io](https://veeso.github.io/termscp/#get-started) 查看所有安装方法。

⚠️ 如果您正在寻找如何更新 termscp 只需从 CLI 运行 termscp ： `(sudo) termscp --update` ⚠️

### 依赖 ❗

- **Linux** 用户:
  - libssh
  - libdbus-1
  - pkg-config
- **FreeBSD** 用户:
  - libssh
  - dbus
  - pkgconf

### 可选项 ✔️

通过执行以下操作以享受软件的完整功能，但不做强制要求

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

如果您喜欢 termscp 并且希望看到该项目不断发展和改进，请考虑在 **Buy me a coffee** 上赞赏以支持我🥳

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)

或者，如果您愿意，您也可以在 PayPal 上赞赏我：

[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## 用户手册和文档 📚

用户手册可以在[termscp的网站](https://veeso.github.io/termscp/#user-manual)或者在[Github](man.md)上找到。

---

## 贡献和问题 🤝🏻

欢迎贡献、bug报告、新功能和问题！ 😉

如果您有任何问题或困惑，或者您想建议新功能，或者您只是想改进termscp，请随时打开 issue 或 PR。

请遵循 [我们的贡献指南](../../CONTRIBUTING.md)

---

## 更新日志 ⏳

查看termscp的 [更新日志](../../CHANGELOG.md)

---

## 支持 💪

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
- [tui-rs](https://github.com/fdehau/tui-rs)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## 演示 🎬

> 首页

![Auth](/assets/images/auth.gif)

> 书签

![Bookmarks](/assets/images/bookmarks.gif)

> 设置

![Setup](/assets/images/config.gif)

> 文本编辑器

![TextEditor](/assets/images/text-editor.gif)

---

## 许可协议 📃

“termscp”使用 MIT 许可。

您可以阅读整个 [许可证](../../LICENSE)
