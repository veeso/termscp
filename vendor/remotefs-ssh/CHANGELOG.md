# Changelog

- [Changelog](#changelog)
  - [0.7.1](#071)
  - [0.7.0](#070)
  - [0.6.4](#064)
  - [0.6.3](#063)
  - [0.6.2](#062)
  - [0.6.1](#061)
  - [0.6.0](#060)
  - [0.5.0](#050)
  - [0.4.1](#041)
  - [0.4.0](#040)
  - [0.3.1](#031)
  - [0.3.0](#030)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.6](#016)
  - [0.1.5](#015)
  - [0.1.3](#013)
  - [0.1.2](#012)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.7.1

Released on 09/11/2025

- MSRV bumped to 1.88.0
- Fixed compatibility with hosts running with fish set as default shell

## 0.7.0

Released on 01/09/2025

- **BREAKING**: Support for multiple SSH backends:
  - Added new feature to enable **libssh2** backend:
    - Use `libssh2` feature to enable the backend
    - Use `libssh2-vendored` to build the backend with vendored libssh2
  - Added support for [libssh](https://www.libssh.org/) backend
    - Use `libssh` feature to enable the backend
    - Use `libssh-vendored` to build the backend with vendored libssh
  - Removed `new`; use `libssh2` and `libssh` constructors instead.

## 0.6.4

Released on 15/08/2025

- ssh2-config 0.6.0

## 0.6.3

Released on 21/07/2025

- Fixed issue with SSH authentication:
  - if the key is resolved and it fails to authenticate, if a password is provided, try to authenticate with the password before returning an error.

## 0.6.2

Released on 16/05/2025

- [Issue 9](https://github.com/remotefs-rs/remotefs-rs-ssh/pull/9): fixed label regex groups and add support for parsing ls output on systems that include SELinux labels, POSIX ACLs, and extended attributes

## 0.6.1

Released on 27/03/2025

- bump `ssh2-config` to `0.5.4`

## 0.6.0

Released on 15/03/2025

- bump `ssh2-config` to `0.4.0`
- edition `2024`

## 0.5.0

Released on 26/10/2024

- `SshKeyStorage` must be `Sync` and `Send`

## 0.4.1

Released on 07/10/2024

- Removed unused dep: `users`

## 0.4.0

Released on 30/09/2024

- bump `remotefs` to `0.3.0`

## 0.3.1

Released on 09/07/2024

- Fix: parse special permissions `StT` in ls output

## 0.3.0

Released on 09/07/2024

- Fix: resolved_host from configuration wasn't used to connect
- `SshOpts::method` now requires `KeyMethod` and `MethodType` to setup key method
- Feat: Implemented `SshAgentIdentity` to specify the ssh agent configuration to be used to authenticate.
  - use `SshOpts.ssh_agent_identity()` to set the option

## 0.2.1

Released on 06/07/2023

- If ssh configuration timeout is `0`, don't set connection timeout

## 0.2.0

Released on 09/05/2023

- `SshOpts::config_file` now requires `SshConfigParseRule` as argument to specify the rules to parse the configuration file

## 0.1.6

Released on 19/04/2023

- Fixed relative paths resolve on Windows

## 0.1.5

Released on 18/04/2023

- Fixed relative paths resolve on Windows

## 0.1.3

Released on 10/02/2023

- Fixed client using ssh2 config parameter `HostName` to resolve configuration parameters.
- Bump `ssh2-config` to `0.1.4`

## 0.1.2

Released on 30/08/2022

- SshKeyStorage trait MUST return `PathBuf` instead of `Path`

## 0.1.1

Released on 20/07/2022

- Added `ssh2-vendored` feature to build libssl statically

## 0.1.0

Released on 04/01/2022

- First release
