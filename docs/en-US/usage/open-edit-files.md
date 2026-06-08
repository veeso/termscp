# Opening and editing files

termscp can open files with an external application and edit text files in your configured editor. Both local and remote files are supported.

## Open and Open With

Press `<V>` to open a file with the system default application for its file type. termscp relies on your operating system's default opener (powered by the [open](https://docs.rs/crate/open/1.7.0) crate), so make sure at least one of the following is available on your system.

- **Windows**: handled automatically through the `start` command.
- **macOS**: handled automatically through `open`, which is already installed.
- **Linux**: one of `xdg-open`, `gio`, `gnome-open`, or `kde-open` must be installed.
- **WSL**: `wslview` is required. Install [wslu](https://github.com/wslutilities/wslu).

Press `<W>` to open a file with a program you specify.

## Editing text files

Press `<O>` to open a file in your configured text editor. Only text files are supported; binary files are not.

If the file is located on the remote host, it is first downloaded into your temporary file directory, and then re-uploaded to the remote host **only** if you changed it. termscp detects changes by checking the file's last modification time.

## Editing remote files

You cannot edit a remote file in place directly from the remote panel. When you open a remote file, it is downloaded into a temporary directory, but termscp cannot create a watcher to detect when the external program you used to open it has closed, so it cannot tell when you are done editing. To edit a remote file, download it to a local directory first, edit it there, and then upload it again.
