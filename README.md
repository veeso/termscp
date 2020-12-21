# TermSCP

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/TermSCP.svg)](https://github.com/ChristianVisintin/TermSCP) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/TermSCP.svg)](https://github.com/ChristianVisintin/TermSCP/issues) [![Downloads](https://img.shields.io/crates/d/termscp.svg)](https://crates.io/crates/termscp) [![Crates.io](https://img.shields.io/badge/crates.io-v0.2.0-orange.svg)](https://crates.io/crates/termscp) [![Docs](https://docs.rs/termscp/badge.svg)](https://docs.rs/termscp)  

[![Build](https://github.com/ChristianVisintin/TermSCP/workflows/Linux/badge.svg)](https://github.com/ChristianVisintin/TermSCP/actions) [![Build](https://github.com/ChristianVisintin/TermSCP/workflows/MacOS/badge.svg)](https://github.com/ChristianVisintin/TermSCP/actions) [![Build](https://github.com/ChristianVisintin/TermSCP/workflows/Windows/badge.svg)](https://github.com/ChristianVisintin/TermSCP/actions) [![codecov](https://codecov.io/gh/ChristianVisintin/termscp/branch/main/graph/badge.svg?token=au67l7nQah)](https://codecov.io/gh/ChristianVisintin/termscp)

~ Basically, WinSCP on a terminal ~  
Developed by Christian Visintin  
Current version: 0.2.0 (21/12/2020)

---

- [TermSCP](#termscp)
  - [About TermSCP 🖥](#about-termscp-)
    - [Why TermSCP 🤔](#why-termscp-)
  - [Features 🎁](#features-)
  - [Installation 🛠](#installation-)
    - [Cargo 🦀](#cargo-)
    - [Deb package 📦](#deb-package-)
    - [RPM package 📦](#rpm-package-)
    - [AUR Package](#aur-package)
    - [Chocolatey 🍫](#chocolatey-)
    - [Brew 🍻](#brew-)
  - [Usage ❓](#usage-)
    - [Address argument 🌎](#address-argument-)
      - [How Password can be provided 🔐](#how-password-can-be-provided-)
  - [Bookmarks ⭐](#bookmarks-)
    - [Are my passwords Safe 😈](#are-my-passwords-safe-)
  - [Text Editor ✏](#text-editor-)
    - [How do I configure the text editor 🦥](#how-do-i-configure-the-text-editor-)
  - [Keybindings ⌨](#keybindings-)
  - [Documentation 📚](#documentation-)
  - [Known issues 🧻](#known-issues-)
  - [Upcoming Features 🧪](#upcoming-features-)
  - [Contributions 🤝🏻](#contributions-)
  - [Changelog ⏳](#changelog-)
  - [Powered by 🚀](#powered-by-)
  - [Gallery 🎬](#gallery-)
  - [License 📃](#license-)

---

## About TermSCP 🖥

TermSCP is basically a porting of WinSCP to terminal. So basically is a terminal utility with an TUI to connect to a remote server to retrieve and upload files and to interact with the local file system. It works both on **Linux**, **MacOS**, **BSD** and **Windows** and supports SFTP, SCP, FTP and FTPS.

![Explorer](assets/images/explorer.gif)

---

### Why TermSCP 🤔

It happens quite often to me, when using SCP at work to forget the path of a file on a remote machine, which forces me then to connect through SSH, gather the file path and finally download it through SCP. I could use WinSCP, but I use Linux and I pratically use the terminal for everything, so I wanted something like WinSCP on my terminal. Yeah, I know there midnight commander too, but actually I don't like it very much tbh (and hasn't a decent support for scp).

## Features 🎁

- Different communication protocols
  - SFTP
  - SCP
  - FTP and FTPS
- Practical user interface to explore and operate on the remote and on the local machine file system
- Compatible with Windows, Linux, BSD and MacOS
- Written in Rust
- Easy to extend with new file transfers protocols

---

## Installation 🛠

If you're considering to install TermSCP I want to thank you 💛 ! I hope you will enjoy TermSCP!  
If you want to contribute to this project, don't forget to check out our contribute guide. [Read More](CONTRIBUTING.md)

### Cargo 🦀

```sh
# Install termscp through cargo
cargo install termscp
```

### Deb package 📦

Get `deb` package from [HERE](https://github.com/ChristianVisintin/TermSCP/releases/download/latest/termscp_0.2.0_amd64.deb)
or run `wget https://github.com/ChristianVisintin/TermSCP/releases/download/latest/termscp_0.2.0_amd64.deb`

then install through dpkg:

```sh
dpkg -i termscp_*.deb
# Or even better with gdebi
gdebi termscp_*.deb
```

### RPM package 📦

Get `rpm` package from [HERE](https://github.com/ChristianVisintin/TermSCP/releases/download/latest/termscp-0.2.0-1.x86_64.rpm)
or run `wget https://github.com/ChristianVisintin/TermSCP/releases/download/latest/termscp-0.2.0-1.x86_64.rpm`

then install through rpm:

```sh
rpm -U termscp_*.rpm
```

### AUR Package

On Arch Linux based distribution, you can install termscp using for example [yay](https://github.com/Jguer/yay), which I recommend to install AUR packages.

```sh
yay -S termscp
```

### Chocolatey 🍫

You can install TermSCP on Windows using [chocolatey](https://chocolatey.org/)

Start PowerShell as administrator and run

```ps
choco install termscp
```

Alternatively you can download the ZIP file from [HERE](https://github.com/ChristianVisintin/TermSCP/releases/download/latest/termscp.0.2.0.nupkg)

and then with PowerShell started with administrator previleges, run:

```ps
choco install termscp -s .
```

### Brew 🍻

You can install TermSCP on MacOS using [brew](https://brew.sh/)

From your terminal run

```sh
brew tap ChristianVisintin/termscp
brew install termscp
```

---

## Usage ❓

TermSCP can be started with the following options:

- `-P, --password <password>` if address is provided, password will be this argument
- `-v, --version` Print version info
- `-h, --help` Print help page

TermSCP can be started in two different mode, if no extra arguments is provided, TermSCP will show the authentication form, where the user will be able to provide the parameters required to connect to the remote peer.

Alternatively, the user can provide an address as argument to skip the authentication form and starting directly the connection to the remote server.

### Address argument 🌎

The address argument has the following syntax:

```txt
[protocol]://[username@]<address>[:port]
```

Let's see some example of this particular syntax, since it's very comfortable and you'll probably going to use this instead of the other one...

- Connect using default protocol (sftp) to 192.168.1.31, port is default for this protocol (22); username is current user's name

    ```sh
    termscp 192.168.1.31
    ```

- Connect using default protocol (sftp) to 192.168.1.31, port is default for this protocol (22); username is `root`

    ```sh
    termscp root@192.168.1.31
    ```

- Connect using scp to 192.168.1.31, port is 4022; username is `omar`

    ```sh
    termscp scp://omar@192.168.1.31:4022
    ```

#### How Password can be provided 🔐

You have probably noticed, that, when providing the address as argument, there's no way to provide the password.
Password can be basically provided through 3 ways when address argument is provided:

- `-P, --password` option: just use this CLI option providing the password. I strongly unrecommend this method, since it's very unsecure (since you might keep the password in the shell history)
- Via `sshpass`: you can provide password via `sshpass`, e.g. `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`
- You will be prompted for it: if you don't use any of the previous methods, you will be prompted for the password, as happens with the more classics tools such as `scp`, `ssh`, etc.

---

## Bookmarks ⭐

In TermSCP it is possible to save favourites hosts, which can be then loaded quickly from the main layout of termscp.
TermSCP will also save the last 16 hosts you connected to.
This feature allows you to load all the parameters required to connect to a certain remote, simply selecting the bookmark in the tab under the authentication form.

Bookmarks will be saved, if possible at:

- `$HOME/.config/termscp/` on Linux
- `FOLDERID_RoamingAppData\termscp\` on Windows
- `$HOME/Library/Application Support/termscp` on MacOs

For bookmarks only (this won't apply to recent hosts) it is also possible to save the password used to authenticate. The password is not saved by default and must be specified through the prompt when saving a new Bookmark.

> I was very undecided about storing passwords in termscp. The reason? Saving a password on your computer might give access to a hacker to any server you've registered. But I must admit by myself that for many machines typing the password everytime is really boring, also many times I have to work with machines in LAN, which wouldn't provide any advantage to an attacker, So I came out with a good compromise for passwords.

I warmly suggest you to follow these guidelines in order to decide whether you should or you shouldn't save passwords:

- **DON'T** save passwords for machines which are exposed on the internet, save passwords only for machines in LAN
- Make sure your machine is protected by attackers. If possible encrypt your disk and don't leave your PC unlocked while you're away.
- Preferably, save passwords only when a compromising of the target machine wouldn't be a problem.

To create a bookmark, just fulfill the authentication form and then input `CTRL+S`; you'll then be asked to give a name to your bookmark, and tadah, the bookmark has been created.
If you go to [gallery](#gallery-), there is a GIF showing how bookmarks work 💪.

### Are my passwords Safe 😈

Well, kinda.
As said before, bookmarks are saved in your configuration directory along with passwords. Passwords are obviously not plain text, they are encrypted with **AES-128**. Does this make them safe? Well, no, the key used to encrypt your passwords is generated at the first launch of termscp and stored on your drive. So it's still possible to retrieve the key to decrypt passwords. Luckily, the location of the key guarantees your key can't be read by users different from yours, but yeah, I still wouldn't save the password for a server exposed on the internet 😉.

---

## Text Editor ✏

TermSCP has, as you might have noticed, many features, one of these is the possibility to view and edit text file. It doesn't matter if the file is located on the local host or on the remote host, termscp provides the possibility to open a file in your favourite text editor.
In case the file is located on remote host, the file will be first downloaded into your temporary file directory and then, **only** if changes were made to the file, re-uploaded to the remote host. TermSCP checks if you made changes to the file calculating the digest of the file using `sha256`.

Just a reminder: **you can edit only textual file**; binary files are not supported.

### How do I configure the text editor 🦥

Text editor is automatically found using this [awesome crate](https://github.com/milkey-mouse/edit), if you want to change the text editor it has chosen for you, just set the `EDITOR` variable in your environment.

> This mechanism will probably change in 0.3.0, since I'm going to introduce the possibility to configure directly in termscp's settings.

---

## Keybindings ⌨

| Key           | Command                                               | Reminder  |
|---------------|-------------------------------------------------------|-----------|
| `<ESC>`       | Disconnect from remote; return to authentication page |           |
| `<TAB>`       | Switch between log tab and explorer                   |           |
| `<BACKSPACE>` | Go to previous directory in stack                     |           |
| `<RIGHT>`     | Move to remote explorer tab                           |           |
| `<LEFT>`      | Move to local explorer tab                            |           |
| `<UP>`        | Move up in selected list                              |           |
| `<DOWN>`      | Move down in selected list                            |           |
| `<PGUP>`      | Move up in selected list by 8 rows                    |           |
| `<PGDOWN>`    | Move down in selected list by 8 rows                  |           |
| `<ENTER>`     | Enter directory                                       |           |
| `<SPACE>`     | Upload / download selected file                       |           |
| `<C>`         | Copy file/directory                                   | Copy      |
| `<D>`         | Make directory                                        | Directory |
| `<E>`         | Delete file (Same as `CANC`)                          | Erase     |
| `<G>`         | Go to supplied path                                   | Go to     |
| `<H>`         | Show help                                             | Help      |
| `<I>`         | Show info about selected file or directory            | Info      |
| `<L>`         | Reload current directory's content                    | List      |
| `<O>`         | Edit file; see [Text editor](#text-editor-)           | Open      |
| `<Q>`         | Quit TermSCP                                          | Quit      |
| `<R>`         | Rename file                                           | Rename    |
| `<U>`         | Go to parent directory                                | Upper     |
| `<DEL>`       | Delete file                                           |           |
| `<CTRL+C>`    | Abort file transfer process                           |           |

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/termscp>

---

## Known issues 🧻

- Ftp:
  - Time in explorer is `1 Jan 1970`, but shouldn't be: that's because chrono can't parse date in a different locale. So if your server has a locale different from the one on your machine, it won't be able to parse the date.
  - Some servers don't work: yes, some kind of ftp server don't work correctly, sometimes it won't display any files in the directories, some other times uploading files will fail. Up to date, `vsftpd` is the only one server which I saw working correctly with TermSCP. Am I going to solve this? I'd like to, but it's not my fault at all. Unfortunately [rust-ftp](https://github.com/mattnenterprise/rust-ftp) is an abandoned project (up to 2020), indeed I had to patch many stuff by myself. I'll try to solve these issues, but it will take a long time.
- `NoSuchFileOrDirectory` on connect: let me guess, you're running on WSL and you've installed termscp through cargo. I know about this issue and it's a glitch of WSL I guess. Don't worry about it, just move the termscp executable into another PATH location, such as `/usr/bin`.

---

## Upcoming Features 🧪

- **SSH Key storage**: termscp 0.3.0 will (finally) support the SSH key storage. From the configuration interface, you will be able to add SSH keys to the termscp's storage as you do indeed with other similiar clients.
- **User customizations**: termscp 0.3.0 will support some user customizations, such as the possibility to setup the text editor directly from termscp and the default communication protocol. Everything will be configurable directly from the termscp user interface.
- **Find command in explorer**: possibility to search for files in explorers.

---

## Contributions 🤝🏻

Contributions are welcome! 😉

If you think you can contribute to TermSCP, please follow [TermSCP's contributions guide](CONTRIBUTING.md)

## Changelog ⏳

View TermSCP's changelog [HERE](CHANGELOG.md)

---

## Powered by 🚀

TermSCP is powered by these aweseome projects:

- [bytesize](https://github.com/hyunsik/bytesize)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [edit](https://github.com/milkey-mouse/edit)
- [rpassword](https://github.com/conradkleinespel/rpassword)
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)
- [textwrap](https://github.com/mgeisler/textwrap)
- [tui-rs](https://github.com/fdehau/tui-rs)
- [whoami](https://github.com/libcala/whoami)

---

## Gallery 🎬

> Termscp Home

![Auth](assets/images/auth.gif)

> Bookmarks

![Bookmarks](assets/images/bookmarks.gif)

> Text editor

![TextEditor](assets/images/text-editor.gif)

---

## License 📃

Licensed under the GNU GPLv3 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.gnu.org/licenses/gpl-3.0.txt>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

You can read the entire license [HERE](LICENSE)
