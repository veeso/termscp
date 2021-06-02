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

<p align="center">Developed by <a href="https://veeso.github.io/">@veeso</a></p>
<p align="center">Current version: 0.6.0 FIXME: (23/05/2021)</p>

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![Stars](https://img.shields.io/github/stars/veeso/termscp.svg)](https://github.com/veeso/termscp) [![Downloads](https://img.shields.io/crates/d/termscp.svg)](https://crates.io/crates/termscp) [![Crates.io](https://img.shields.io/badge/crates.io-v0.6.0-orange.svg)](https://crates.io/crates/termscp) [![Docs](https://docs.rs/termscp/badge.svg)](https://docs.rs/termscp)  

[![Build](https://github.com/veeso/termscp/workflows/Linux/badge.svg)](https://github.com/veeso/termscp/actions) [![Build](https://github.com/veeso/termscp/workflows/MacOS/badge.svg)](https://github.com/veeso/termscp/actions) [![Build](https://github.com/veeso/termscp/workflows/Windows/badge.svg)](https://github.com/veeso/termscp/actions) [![Coverage Status](https://coveralls.io/repos/github/veeso/termscp/badge.svg)](https://coveralls.io/github/veeso/termscp)

---

## About termscp 🖥

Termscp is a feature rich terminal file transfer and explorer, with support for SCP/SFTP/FTP. So basically is a terminal utility with an TUI to connect to a remote server to retrieve and upload files and to interact with the local file system. It is **Linux**, **MacOS**, **BSD** and **Windows** compatible and supports SFTP, SCP, FTP and FTPS.

![Explorer](assets/images/explorer.gif)

---

## Features 🎁

- 📁  Different communication protocols support
  - SFTP
  - SCP
  - FTP and FTPS
- 🖥  Explore and operate on the remote and on the local machine file system with a handy UI
  - Create, remove, rename, search, view and edit files
- ⭐  Connect to your favourite hosts through built-in bookmarks and recent connections
- 📝  View and edit text files with your favourite text editor
- 💁  SFTP/SCP authentication through SSH keys and username/password
- 🐧  Compatible with Windows, Linux, BSD and MacOS
- ✏  Customizable
  - Custom file explorer format
  - Customizable text editor
  - Customizable file sorting
- 🔐  Save your password in your operating system key vault
- 🦀  Rust-powered
- 🤝  Easy to extend with new file transfers protocols
- 👀  Developed keeping an eye on performance
- 🦄  Frequent awesome updates

---

## Get started 🚀

If you're considering to install termscp I want to thank you 💜 ! I hope you will enjoy termscp!  
If you want to contribute to this project, don't forget to check out our contribute guide. [Read More](CONTRIBUTING.md)

If you are a Linux or a MacOS user this simple shell script will install termscp on your system with a single command:

```sh
curl --proto '=https' --tlsv1.2 -sSf "https://raw.githubusercontent.com/veeso/termscp/main/install.sh" | sh
```

while if you're a Windows user, you can install termscp with [Chocolatey](https://chocolatey.org/).

For more information or other platforms, please visit [veeso.github.io](https://veeso.github.io/termscp/#get-started) to view all installation methods.

---

## Buy me a coffee ☕

If you like termscp and you'd love to see the project to grow, please consider a little donation 🥳

[![Buy-me-a-coffee](https://img.buymeacoffee.com/button-api/?text=Buy%20me%20a%20coffee&emoji=&slug=veeso&button_colour=404040&font_colour=ffffff&font_family=Comic&outline_colour=ffffff&coffee_colour=FFDD00)](https://www.buymeacoffee.com/veeso)

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/termscp>

---

## Known issues 🧻

- `NoSuchFileOrDirectory` on connect (WSL1): I know about this issue and it's a glitch of WSL I guess. Don't worry about it, just move the termscp executable into another PATH location, such as `/usr/bin`, or install it through the appropriate package format (e.g. deb).

---

## Upcoming Features 🧪

Major termscp releases will now be seasonal, so expect 4 major updates during the year.

Planned for *🏄 Summer update 🌴*:

- **Keyring-rs on Linux 🔐**: Check for updates in [this issue](https://github.com/veeso/termscp/issues/2)
- **SMB Support 🎉**: This will require a long time to be implemented, since I'm currently working on a Rust native SMB library, since I don't want to add new C-bindings. ~~Fear the 🦚~~
- **Open files with any application ☄️**: possibility to open files of any kind and with any application directly inside termscp. This will be achieved through this awesome crate [open-rs](https://github.com/Byron/open-rs).

To be planned:

- **Themes provider 🎨**: I'm still thinking about how I will implement this, but basically the idea is to have a configuration file where it will be possible to define the color schema for the entire application. I haven't planned this release yet
- **Configuration profile for bookmarks 📚**: I would like to, but I still have to analyze it.
- **AWS S3 support 🪣**: There is already a library for AWS S3, but this is really on bottom of my implementation list at the moment, due to interest and I don't really have a system where to test it.

Along to new features, termscp developments is now focused on UI and performance improvements, so if you have any suggestion, feel free to open an issue.

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve termscp, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View termscp's changelog [HERE](CHANGELOG.md)

---

## Powered by 💪

termscp is powered by these aweseome projects:

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [keyring-rs](https://github.com/hwchen/keyring-rs)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [rust-ftp](https://github.com/mattnenterprise/rust-ftp)
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
- [textwrap](https://github.com/mgeisler/textwrap)
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
