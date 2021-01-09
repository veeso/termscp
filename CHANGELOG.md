# Changelog

- [Changelog](#changelog)
  - [0.3.0](#030)
  - [0.2.0](#020)
  - [0.1.3](#013)
  - [0.1.2](#012)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.3.0

FIXME: Released on

> The SSH Key Storage Update

- **SSH Key Storage**
  - Added the possibility to store SSH private keys to access to remote hosts; this feature is supported in both SFTP and SCP.
  - SSH Keys can be manipulated through the new **Setup Interface**
- **Setup Interface**
  - Added a new area in the interface, where is possible to customize termscp. Access to this interface is achieved pressing `<CTRL+C>` from the home page (`AuthActivity`).
- **Configuration**:
  - Added configuration; configuration is stored at
    - Linux: `/home/alice/.config/termscp/config.toml`
    - MacOS: `/Users/Alice/Library/Application Support/termscp/config.toml`
    - Windows: `C:\Users\Alice\AppData\Roaming\termscp\config.toml`
  - Added Text editor to configuration
  - Added Default File transfer protocol to configuration
  - Added "Show hidden files" to configuration
  - Added "Group directories" to configuration
  - Added SSH keys to configuration; SSH keys will be stored at
    - Linux: `/home/alice/.config/termscp/.ssh/`
    - MacOS: `/Users/Alice/Library/Application Support/termscp/.ssh/`
    - Windows: `C:\Users\Alice\AppData\Roaming\termscp\.ssh\`
- Enhancements:
  - Replaced `sha256` sum with last modification time check, to verify if a file has been changed in the text editor
  - **FTP**
    - Added `LIST` command parser for Windows server (DOS-like syntax)
  - Default protocol changed to default protocol in configuration when providing address as CLI argument
  - Explorers:
    - Hidden files are now not shown by default; use `A` to show hidden files.
    - Append `/` to directories name.
- Keybindings:
  - `A`: Toggle hidden files
  - `B`: Sort files by (name, size, creation time, modify time)
  - `N`: New file
- Bugfix:
  - SCP client didn't show file types for files
  - FTP client didn't show file types for files
  - FTP file transfer not working properly with `STOR` and `RETR`.
  - Fixed `0 B/S` transfer rate displayed after completing download in less than 1 second
- Dependencies:
  - added `bitflags 1.2.1`
  - removed `data-encoding`
  - updated `ftp` to `4.0.2`
  - updated `rand` to `0.8.0`
  - removed `ring`
  - updated `textwrap` to `0.13.1`
  - updated `toml` to `0.5.8`
  - updated `whoami` to `1.0.1`

## 0.2.0

Released on 21/12/2020

> The Bookmarks Update

- **Bookmarks**
  - Bookmarks and recent connections are now displayed in the home page
  - Bookmarks are saved at
    - Linux: `/home/alice/.config/termscp/bookmarks.toml`
    - MacOS: `/Users/Alice/Library/Application Support/termscp/bookmarks.toml`
    - Windows: `C:\Users\Alice\AppData\Roaming\termscp\bookmarks.toml`
- **Text Editor**
  - Added text editor feature to explorer view
  - Added `o` to keybindings to open a text file
- Keybindings:
  - `C`: Copy file/directory
  - `O`: Open text file in editor
- Enhancements:
  - User interface
    - Collpased borders to make everything more *aesthetic*
    - Rounded input field boards
    - File explorer:
      - Log how long it took to upload/download a file and the transfer speed
      - Display in progress bar the transfer speed (bytes/seconds)
- Bugfix:
  - File mode of file on remote is now reported on local file after being downloaded (unix, linux, macos only)
  - Scp: when username was not provided, it didn't fallback to current username
  - Explorer: fixed UID format in Windows

## 0.1.3

Released on 14/12/2020

- Enhancements:
  - File transfer:
    - Read buffer is now 65536 bytes long
  - File explorer:
    - Fixed color mismatch in local explorer
    - Explorer tabs have now 70% of layout height, while logging area is 30%
    - Highlight selected entry in tabs, only when the tab is active
  - Auth page:
    - align popup text to center
- Keybindings:
  - `L`: Refresh directory content
- Bugfix:
  - Fixed memory vulnerability in Windows version

## 0.1.2

Released on 13/12/2020

- General performance and code improvements
- Improved symlinks management
- Possibility to abort file transfers
- Enhancements:
  - File explorer:
    - When file index is at the end of the list, moving down will set the current index to the first element and viceversa.
    - Selected file has now colourful background, instead of foreground, for a better readability.
- Keybindings:
  - `E`: Delete file (Same as `DEL`); added because some keyboards don't have `DEL` (hey, that's my MacBook Air's keyboard!)
  - `Ctrl+C`: Abort transfer process

## 0.1.1

Released on 10/12/2020

- enhancements:
  - password prompt: ask before performing terminal clear
  - file explorer:
    - file names are now sorted ignoring capital letters
    - file names longer than 23, are now cut to 20 and followed by `...`
    - paths which exceed tab size in explorer are elided with the following formato `ANCESTOR[1]/.../PARENT/DIRNAME`
- keybindings:
  - `I`: show info about selected file or directory
  - Removed `CTRL`; just use keys now.
- bugfix:
  - prevent panic in set_progress, for progress values `> 100.0 or < 0.0`
  - Fixed FTP get, which didn't finalize the reader
- dependencies:
  - updated `textwrap` to `0.13.0`
  - updated `ftp4` to `4.0.1`

## 0.1.0

Released on 06/12/2020

- First release
