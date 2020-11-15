# TermSCP

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/TermSCP.svg)](https://github.com/ChristianVisintin/TermSCP) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/TermSCP.svg)](https://github.com/ChristianVisintin/TermSCP/issues) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/termscp) [![Build](https://api.travis-ci.org/ChristianVisintin/TermSCP.svg?branch=main)](https://travis-ci.org/ChristianVisintin/TermSCP) [![codecov](https://codecov.io/gh/ChristianVisintin/TermSCP/branch/main/graph/badge.svg)](https://codecov.io/gh/ChristianVisintin/TermSCP)

~ Basically, WinSCP on a terminal ~  
Developed by Christian Visintin  
Current version: 0.1.0 (??/??/2020)

⚠ This project is **still under development**; installation instructions won't work until release ⚠

---

- [TermSCP](#termscp)
  - [About TermSCP](#about-termscp)
    - [Why TermSCP](#why-termscp)
  - [Features](#features)
  - [Installation](#installation)
    - [Cargo](#cargo)
    - [Deb / Rpm](#deb--rpm)
    - [Usage](#usage)
  - [Documentation](#documentation)
  - [Known issues](#known-issues)
  - [Upcoming Features](#upcoming-features)
  - [Contributions](#contributions)
  - [Changelog](#changelog)
  - [License](#license)

---

## About TermSCP

TermSCP is basically a porting of WinSCP to terminal. So basically is a terminal tool with an UI to connect to a remote server to retrieve and upload files. It works both on Linux, MacOS and Windows (TODO: double check) and supports SFTP and FTPS.

### Why TermSCP

It happens very often to me when using SCP at work to forget the path of a file on a remote machine, which forces me then to connect through SSH, gather the file path and finally download it through SCP. I could use WinSCP, but I use Linux and I pratically use the terminal for everything, so I wanted something like WinSCP on my terminal.

## Features

- Different communication protocols
  - SFTP
  - FTPS
- Practical user interface to explore the remote machine file system and to select the files to upload and download
- Written in Rust
- Easy to extend with new protocols

## Installation

If you're considering to install TermSCP I want to thank you 💛 ! I hope this project can be useful for you!  
If you want to contribute to this project, don't forget to check out our contribute guide. [Read More](CONTRIBUTING.md)

### Cargo

```sh
# Install termscp through cargo
cargo install termscp
```

### Deb / Rpm

Coming soon

### Usage

TermSCP can be started with the following options:

- `-v, --version` Print version info
- `-h, --help` Print help page

## Documentation

The developer documentation can be found on Rust Docs at <https://docs.rs/termscp>

---

## Known issues

TODO:

---

## Upcoming Features

TODO:

---

## Contributions

Contributions are welcome! 😉

If you think you can contribute to TermSCP, please follow [TermSCP's contributions guide](CONTRIBUTING.md)

## Changelog

See the enire changelog [HERE](CHANGELOG.md)

---

## License

Licensed under the GNU GPLv3 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.gnu.org/licenses/gpl-3.0.txt>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

You can read the entire license [HERE](LICENSE)
