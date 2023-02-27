# termscp

<p align="center">
  <img src="/assets/images/termscp.svg" width="256" height="256" />
</p>

<p align="center">~ A feature rich terminal file transfer ~</p>
<p align="center">
  <a href="https://veeso.github.io/termscp/" target="_blank">Website</a>
  ·
  <a href="https://veeso.github.io/termscp/#get-started" target="_blank">Installation</a>
  ·
  <a href="https://veeso.github.io/termscp/#user-manual" target="_blank">User manual</a>
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
    href="/docs/de/README.md"
    ><img
      height="20"
      src="/assets/images/flags/de.png"
      alt="Deutsch"
  /></a>
  &nbsp;
  <a
    href="/docs/es/README.md"
    ><img
      height="20"
      src="/assets/images/flags/es.png"
      alt="Español"
  /></a>
  &nbsp;
  <a
    href="/docs/fr/README.md"
    ><img
      height="20"
      src="/assets/images/flags/fr.png"
      alt="Français"
  /></a>
  &nbsp;
  <a
    href="/docs/it/README.md"
    ><img
      height="20"
      src="/assets/images/flags/it.png"
      alt="Italiano"
  /></a>
  &nbsp;
  <a
    href="/docs/zh-CN/README.md"
    ><img
      height="20"
      src="/assets/images/flags/cn.png"
      alt="简体中文"
  /></a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.11.1 (20/02/2023)</p>

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
  <a href="https://coveralls.io/github/veeso/termscp"
    ><img
      src="https://coveralls.io/repos/github/veeso/termscp/badge.svg"
      alt="Coveralls"
  /></a>
</p>

---

## About termscp 🖥

Termscp is a feature rich terminal file transfer and explorer, with support for SCP/SFTP/FTP/S3. So basically is a terminal utility with an TUI to connect to a remote server to retrieve and upload files and to interact with the local file system. It is **Linux**, **MacOS**, **FreeBSD**, **NetBSD** and **Windows** compatible.

![Explorer](assets/images/explorer.gif)

---

## Features 🎁

- 📁  Different communication protocols
  - **SFTP**
  - **SCP**
  - **FTP** and **FTPS**
  - **S3**
- 🖥  Explore and operate on the remote and on the local machine file system with a handy UI
  - Create, remove, rename, search, view and edit files
- ⭐  Connect to your favourite hosts through built-in bookmarks and recent connections
- 📝  View and edit files with your favourite applications
- 💁  SFTP/SCP authentication with SSH keys and username/password
- 🐧  Compatible with Windows, Linux, FreeBSD, NetBSD and MacOS
- 🎨  Make it yours!
  - Themes
  - Custom file explorer format
  - Customizable text editor
  - Customizable file sorting
  - and many other parameters...
- 📫  Get notified via Desktop Notifications when a large file has been transferred
- 🔭  Keep file changes synchronized with the remote host
- 🔐  Save your password in your operating system key vault
- 🦀  Rust-powered
- 👀  Developed keeping an eye on performance
- 🦄  Frequent awesome updates

---

## Get started 🚀

If you're considering to install termscp I want to thank you 💜 ! I hope you will enjoy termscp!  
If you want to contribute to this project, don't forget to check out our [contribute guide](CONTRIBUTING.md).

If you are a Linux, a FreeBSD or a MacOS user this simple shell script will install termscp on your system with a single command:

```sh
curl --proto '=https' --tlsv1.2 -sSLf "https://git.io/JBhDb" | sh
```

> ❗ MacOs installation requires [Homebrew](https://brew.sh/), otherwise the Rust compiler will be installed

while if you're a Windows user, you can install termscp with [Chocolatey](https://chocolatey.org/):

```ps
choco install termscp
```

NetBSD users can install termscp from the official repositories.

```sh
pkgin install termscp
```

For more information or other platforms, please visit [veeso.github.io](https://veeso.github.io/termscp/#get-started) to view all installation methods.

⚠️ If you're looking on how to update termscp just run termscp from CLI with: `(sudo) termscp --update` ⚠️

### Requirements ❗

- **Linux** users:
  - libdbus-1
  - pkg-config
- **FreeBSD** or, **NetBSD** users:
  - dbus
  - pkgconf

### Optional Requirements ✔️

These requirements are not forced required to run termscp, but to enjoy all of its features

- **Linux/FreeBSD** users:
  - To **open** files via `V` (at least one of these)
    - *xdg-open*
    - *gio*
    - *gnome-open*
    - *kde-open*
- **Linux** users:
  - A keyring manager: read more in the [User manual](docs/man.md#linux-keyring)
- **WSL** users
  - To **open** files via `V` (at least one of these)
    - [wslu](https://github.com/wslutilities/wslu)

---

## Support the developer ☕

If you like termscp and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)

---

## User manual 📚

The user manual can be found on the [termscp's website](https://veeso.github.io/termscp/#user-manual) or on [Github](docs/man.md).

---

## Upcoming Features 🧪

For **2023** there will be two major updates during the year.

Along to new features, termscp developments is now focused on UX and performance improvements, so if you have any suggestion, feel free to open an issue.

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve termscp, feel free to open an issue or a PR.

An **appreciated** contribution would be a translation of the user manual and readme in **other languages**

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View termscp's changelog [HERE](CHANGELOG.md)

---

## Powered by 💪

termscp is powered by these awesome projects:

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [keyring-rs](https://github.com/hwchen/keyring-rs)
- [open-rs](https://github.com/Byron/open-rs)
- [remotefs](https://github.com/veeso/remotefs-rs)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [self_update](https://github.com/jaemk/self_update)
- [tui-rs](https://github.com/fdehau/tui-rs)
- [tui-realm](https://github.com/veeso/tui-realm)
- [whoami](https://github.com/libcala/whoami)
- [wildmatch](https://github.com/becheran/wildmatch)

---

## Gallery 🎬

> Termscp Home

![Auth](assets/images/auth.gif)

> Bookmarks

![Bookmarks](assets/images/bookmarks.gif)

> Setup

![Setup](assets/images/config.gif)

> Text editor

![TextEditor](assets/images/text-editor.gif)

---

## License 📃

termscp is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
