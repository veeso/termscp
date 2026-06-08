# Configuration

termscp supports a number of user-defined parameters. termscp stores them in a
TOML file and a few directories, but you never edit these files by hand:
configuration is done entirely from the user interface.

To enter the configuration, press `<CTRL+C>` from the termscp home.

termscp requires these paths to be accessible:

- `$HOME/.config/termscp/` on Linux/BSD
- `$HOME/Library/Application Support/termscp` on macOS
- `FOLDERID_RoamingAppData\termscp\` on Windows

## Parameters

The following parameters can be configured:

- **Text Editor**: the text editor to use. By default termscp finds the default
  editor for you; with this option you can force an editor to be used (e.g.
  `vim`). GUI editors are also supported, unless they detach (`nohup`) from the
  parent process.
- **Default Protocol**: the default value for the file transfer protocol to be
  used in termscp. It applies to the login page and to the address CLI argument.
- **Show Hidden Files**: whether hidden files are displayed by default. You can
  also toggle hidden files at runtime by pressing `A`.
- **Check for updates**: if set to `yes`, termscp queries the GitHub API to
  check whether a new version of termscp is available.
- **Prompt when replacing existing files**: if set to `yes`, termscp prompts for
  confirmation whenever a file transfer would replace an existing file on the
  target host.
- **Group Dirs**: whether directories are grouped in the file explorers. If
  `Display first` is selected, directories are sorted with the configured method
  but displayed before files; with `Display last` they are displayed after
  files.
- **Remote file formatter syntax**: syntax used to display file info for each
  file in the remote explorer. See [File explorer format](explorer-format.md).
- **Local file formatter syntax**: syntax used to display file info for each file
  in the local explorer. See [File explorer format](explorer-format.md).
- **Enable notifications**: if set to `Yes`, desktop notifications are displayed.
  See [Notifications](notifications.md).
- **Notifications: minimum transfer size**: if the transfer size is greater than
  or equal to the specified value, transfer notifications are displayed. The
  accepted format is `{UNSIGNED} B/KB/MB/GB/TB/PB`.
- **SSH configuration path**: SSH configuration file to use when connecting to a
  SCP/SFTP server. If left empty, no file is used. You can specify a path
  starting with `~` to indicate the home directory (e.g. `~/.ssh/config`). The
  attributes supported by termscp are listed at
  [the ssh2-config exposed attributes](https://github.com/veeso/ssh2-config#exposed-attributes).
  See also [SSH key storage](ssh-keys.md).
