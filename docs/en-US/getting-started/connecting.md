# Connecting to a server

termscp can start in three different ways depending on the arguments you pass.

- No arguments: termscp opens the authentication form, where you provide the
  parameters required to connect to the remote host.
- An address argument: termscp skips the authentication form and connects
  directly to the remote host.
- A bookmark name with `-b <bookmark-name>`: termscp resolves the argument as a
  saved bookmark and connects. Repeat `-b` to open several bookmarks.

When you provide an address argument or a bookmark name, you can also provide a
start working directory for the local host.

## The authentication form

When termscp starts without an address, it shows the authentication form. Fill
in the protocol, address, port, username, and password, then connect. termscp
will open the dual-pane explorer once the connection succeeds.

## Address argument syntax

The generic address argument has the following syntax:

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

This syntax is convenient, and you will probably use it instead of the
interactive form. Here are some examples.

Connect using the default protocol (defined in your configuration) to
`192.168.1.31`. If the port is not provided, the default port for the selected
protocol is used. The username is the current user's name.

```sh
termscp 192.168.1.31
```

Connect using the default protocol to `192.168.1.31` as user `root`:

```sh
termscp root@192.168.1.31
```

Connect using SCP to `192.168.1.31` on port `4022` as user `omar`:

```sh
termscp scp://omar@192.168.1.31:4022
```

Connect using SCP to `192.168.1.31` on port `4022` as user `omar`, starting in
directory `/tmp`:

```sh
termscp scp://omar@192.168.1.31:4022:/tmp
```

For protocol-specific address syntax (S3, Kube, WebDAV, and SMB), see
[Connection parameters](connection-parameters.md).

## How the password is provided

When you provide the address as an argument, there is no field for the password
in the address itself. You can provide the password in three ways:

- You will be prompted for it. This is the default: if you don't use any of the
  methods below, termscp prompts for the password, like classic tools such as
  `scp` and `ssh`.
- `-P, --password` option: pass the password directly on the command line. This
  method is discouraged because it is insecure: the password may be kept in
  your shell history.
- Via `sshpass`: provide the password through `sshpass`, for example:

  ```sh
  sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31
  ```
