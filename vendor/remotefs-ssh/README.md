# remotefs SSH

<p align="center">
  <a href="https://veeso.github.io/remotefs-ssh/blob/main/CHANGELOG.md" target="_blank">Changelog</a>
  ·
  <a href="#get-started">Get started</a>
  ·
  <a href="https://docs.rs/remotefs-ssh" target="_blank">Documentation</a>
</p>

<p align="center">~ Remotefs SSH client ~</p>

<p align="center">Developed by <a href="https://veeso.me/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.7.1 (09/11/2025)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/remotefs-rs/remotefs-rs-ssh/stargazers"
    ><img
      src="https://img.shields.io/github/stars/remotefs-rs/remotefs-rs-ssh.svg?style=badge&icon=github"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/remotefs-ssh"
    ><img
      src="https://img.shields.io/crates/d/remotefs-ssh.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/remotefs-ssh"
    ><img
      src="https://img.shields.io/crates/v/remotefs-ssh.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/remotefs-rs/remotefs-rs-ssh/actions/workflows/linux.yml"
    ><img
      src="https://github.com/remotefs-rs/remotefs-rs-ssh/actions/workflows/test.yml/badge.svg"
      alt="Linux CI"
  /></a>
  <a href="https://coveralls.io/github/remotefs-rs/remotefs-rs-ssh"
    ><img
      src="https://coveralls.io/repos/github/remotefs-rs/remotefs-rs-ssh/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/remotefs-ssh"
    ><img
      src="https://docs.rs/remotefs-ssh/badge.svg"
      alt="Docs"
  /></a>
</p>

---

## About remotefs-ssh ☁️

remotefs-ssh is a client implementation for [remotefs](https://github.com/remotefs-rs/remotefs-rs), providing support for the SFTP/SCP protocol.

---

## Get started 🚀

First of all, add `remotefs-ssh` to your project dependencies:

```toml
remotefs = "0.3"
remotefs-ssh = "^0.7"
```

> [!NOTE]
> The library supports multiple ssh backends.
> Currently `libssh2` and `libssh` are supported.
>
> By default the library is using `libssh2`.

### Available backends

Each backend can be set as a feature in your `Cargo.toml`. Multiple backends can be enabled at the same time.

- `libssh2`: The default backend, using the `libssh2` library for SSH connections.
- `libssh`: An alternative backend, using the `libssh` library for SSH connections.

Each backend can be built with the vendored version, using the vendored feature instead:

- `libssh2-vendored`: Build the `libssh2` backend with the vendored version of the library.
- `libssh-vendored`: Build the `libssh` backend with the vendored version of the library.

If the vendored feature is **NOT** provided, you will need to have the corresponding system libraries installed on your machine.

> [!NOTE]
> If you need SftpFs to be `Sync` YOU MUST use libssh2.

### Other features

these features are supported:

- `find`: enable `find()` method on client (*enabled by default*)
- `no-log`: disable logging. By default, this library will log via the `log` crate.

## Ssh client

Here is a basic usage example, with the `Sftp` client, which is very similiar to the `Scp` client.

Both the `SftpFs` and `ScpFs` constructors are respectively `SftpFs::libssh2` and `SftpFs::libssh` accordingly to the enabled backends.

```rust,ignore
// import remotefs trait and client
use remotefs::RemoteFs;
use remotefs_ssh::{SshConfigParseRule, SftpFs, SshOpts};
use std::path::Path;

let opts = SshOpts::new("127.0.0.1")
    .port(22)
    .username("test")
    .password("password")
    .config_file(Path::new("/home/cvisintin/.ssh/config"), ParseRule::STRICT);

let mut client = SftpFs::libssh2(opts);

// connect
assert!(client.connect().is_ok());
// get working directory
println!("Wrkdir: {}", client.pwd().ok().unwrap().display());
// change working directory
assert!(client.change_dir(Path::new("/tmp")).is_ok());
// disconnect
assert!(client.disconnect().is_ok());
```

---

### Client compatibility table ✔️

The following table states the compatibility for the client client and the remote file system trait method.

Note: `connect()`, `disconnect()` and `is_connected()` **MUST** always be supported, and are so omitted in the table.

| Client/Method  | Scp | Sftp |
|----------------|-----|------|
| append_file    | No  | Yes  |
| append         | No  | Yes  |
| change_dir     | Yes | Yes  |
| copy           | Yes | Yes  |
| create_dir     | Yes | Yes  |
| create_file    | Yes | Yes  |
| create         | Yes | Yes  |
| exec           | Yes | Yes  |
| exists         | Yes | Yes  |
| list_dir       | Yes | Yes  |
| mov            | Yes | Yes  |
| open_file      | Yes | Yes  |
| open           | Yes | Yes  |
| pwd            | Yes | Yes  |
| remove_dir_all | Yes | Yes  |
| remove_dir     | Yes | Yes  |
| remove_file    | Yes | Yes  |
| setstat        | Yes | Yes  |
| stat           | Yes | Yes  |
| symlink        | Yes | Yes  |

---

## Support the developer ☕

If you like remotefs-ssh and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features, and questions are welcome! 😉
If you have any questions or concerns, or you want to suggest a new feature, or you want just want to improve remotefs, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View remotefs-ssh changelog [HERE](CHANGELOG.md)

---

## Powered by 💪

remotefs-ssh is powered by these aweseome projects:

- [ssh2-config](https://github.com/veeso/ssh2-config)
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)

---

## License 📃

remotefs-ssh is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
