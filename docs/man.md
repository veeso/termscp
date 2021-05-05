# User manual 🎓

- [User manual 🎓](#user-manual-)
  - [Usage ❓](#usage-)
    - [Address argument 🌎](#address-argument-)
      - [How Password can be provided 🔐](#how-password-can-be-provided-)
  - [Keybindings ⌨](#keybindings-)
  - [Bookmarks ⭐](#bookmarks-)
    - [Are my passwords Safe 😈](#are-my-passwords-safe-)
  - [Configuration ⚙️](#configuration-️)
    - [SSH Key Storage 🔐](#ssh-key-storage-)
    - [File Explorer Format](#file-explorer-format)
  - [Text Editor ✏](#text-editor-)
    - [How do I configure the text editor 🦥](#how-do-i-configure-the-text-editor-)

---

## Usage ❓

termscp can be started with the following options:

`termscp [options]... [protocol://user@address:port:wrkdir] [local-wrkdir]`

- `-P, --password <password>` if address is provided, password will be this argument
- `-v, --version` Print version info
- `-h, --help` Print help page

termscp can be started in two different mode, if no extra arguments is provided, termscp will show the authentication form, where the user will be able to provide the parameters required to connect to the remote peer.

Alternatively, the user can provide an address as argument to skip the authentication form and starting directly the connection to the remote server.

If address argument is provided you can also provide the start working directory for local host

### Address argument 🌎

The address argument has the following syntax:

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

Let's see some example of this particular syntax, since it's very comfortable and you'll probably going to use this instead of the other one...

- Connect using default protocol (*defined in configuration*) to 192.168.1.31, port if not provided is default for the selected protocol (in this case depends on your configuration); username is current user's name

    ```sh
    termscp 192.168.1.31
    ```

- Connect using default protocol (*defined in configuration*) to 192.168.1.31; username is `root`

    ```sh
    termscp root@192.168.1.31
    ```

- Connect using scp to 192.168.1.31, port is 4022; username is `omar`

    ```sh
    termscp scp://omar@192.168.1.31:4022
    ```

- Connect using scp to 192.168.1.31, port is 4022; username is `omar`. You will start in directory `/tmp`

    ```sh
    termscp scp://omar@192.168.1.31:4022:/tmp
    ```

#### How Password can be provided 🔐

You have probably noticed, that, when providing the address as argument, there's no way to provide the password.
Password can be basically provided through 3 ways when address argument is provided:

- `-P, --password` option: just use this CLI option providing the password. I strongly unrecommend this method, since it's very unsecure (since you might keep the password in the shell history)
- Via `sshpass`: you can provide password via `sshpass`, e.g. `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`
- You will be prompted for it: if you don't use any of the previous methods, you will be prompted for the password, as happens with the more classics tools such as `scp`, `ssh`, etc.

---

## Keybindings ⌨

| Key           | Command                                               | Reminder    |
|---------------|-------------------------------------------------------|-------------|
| `<ESC>`       | Disconnect from remote; return to authentication page |             |
| `<TAB>`       | Switch between log tab and explorer                   |             |
| `<BACKSPACE>` | Go to previous directory in stack                     |             |
| `<RIGHT>`     | Move to remote explorer tab                           |             |
| `<LEFT>`      | Move to local explorer tab                            |             |
| `<UP>`        | Move up in selected list                              |             |
| `<DOWN>`      | Move down in selected list                            |             |
| `<PGUP>`      | Move up in selected list by 8 rows                    |             |
| `<PGDOWN>`    | Move down in selected list by 8 rows                  |             |
| `<ENTER>`     | Enter directory                                       |             |
| `<SPACE>`     | Upload / download selected file                       |             |
| `<A>`         | Toggle hidden files                                   | All         |
| `<B>`         | Sort files by                                         | Bubblesort? |
| `<C>`         | Copy file/directory                                   | Copy        |
| `<D>`         | Make directory                                        | Directory   |
| `<E>`         | Delete file (Same as `DEL`)                           | Erase       |
| `<F>`         | Search for files (wild match is supported)            | Find        |
| `<G>`         | Go to supplied path                                   | Go to       |
| `<H>`         | Show help                                             | Help        |
| `<I>`         | Show info about selected file or directory            | Info        |
| `<L>`         | Reload current directory's content                    | List        |
| `<N>`         | Create new file with provided name                    | New         |
| `<O>`         | Edit file; see [Text editor](#text-editor-)           | Open        |
| `<Q>`         | Quit termscp                                          | Quit        |
| `<R>`         | Rename file                                           | Rename      |
| `<S>`         | Save file as...                                       | Save        |
| `<U>`         | Go to parent directory                                | Upper       |
| `<X>`         | Execute a command                                     | eXecute     |
| `<Y>`         | Toggle synchronized browsing                          | sYnc        |
| `<DEL>`       | Delete file                                           |             |
| `<CTRL+C>`    | Abort file transfer process                           |             |

---

## Bookmarks ⭐

In termscp it is possible to save favourites hosts, which can be then loaded quickly from the main layout of termscp.
termscp will also save the last 16 hosts you connected to.
This feature allows you to load all the parameters required to connect to a certain remote, simply selecting the bookmark in the tab under the authentication form.

Bookmarks will be saved, if possible at:

- `$HOME/.config/termscp/` on Linux/BSD
- `$HOME/Library/Application Support/termscp` on MacOs
- `FOLDERID_RoamingAppData\termscp\` on Windows

For bookmarks only (this won't apply to recent hosts) it is also possible to save the password used to authenticate. The password is not saved by default and must be specified through the prompt when saving a new Bookmark.

> I was very undecided about storing passwords in termscp. The reason? Saving a password on your computer might give access to a hacker to any server you've registered. But I must admit by myself that for many machines typing the password everytime is really boring, also many times I have to work with machines in LAN, which wouldn't provide any advantage to an attacker, So I came out with a good compromise for passwords.

I warmly suggest you to follow these guidelines in order to decide whether you should or you shouldn't save passwords:

- **DON'T** save passwords for machines which are exposed on the internet, save passwords only for machines in LAN
- Make sure your machine is protected by attackers. If possible encrypt your disk and don't leave your PC unlocked while you're away.
- Preferably, save passwords only when a compromising of the target machine wouldn't be a problem.

In order to create a new bookmark, just follow these steps:

1. Type in the authentication form the parameters to connect to your remote server
2. Press `<CTRL+S>`
3. Type in the name you want to give to the bookmark
4. Choose whether to remind the password or not
5. Press `<ENTER>` to submit

whenever you want to use the previously saved connection, just press `<TAB>` to navigate to the bookmarks list and load the bookmark parameters into the form pressing `<ENTER>`.

![Bookmarks](assets/images/bookmarks.gif)

### Are my passwords Safe 😈

Well, kinda.
As said before, bookmarks are saved in your configuration directory along with passwords. Passwords are obviously not plain text, they are encrypted with **AES-128**. Does this make them safe? Well, depends on your operating system:

On Windows and MacOS the passwords are stored, if possible (but should be), in respectively the Windows Vault and the Keychain. This is actually super-safe and is directly managed by your operating system.

On Linux and BSD, on the other hand, the key used to encrypt your passwords is stored on your drive (at $HOME/.config/termscp). It is then, still possible to retrieve the key to decrypt passwords. Luckily, the location of the key guarantees your key can't be read by users different from yours, but yeah, I still wouldn't save the password for a server exposed on the internet 😉.
Actually [keyring-rs](https://github.com/hwchen/keyring-rs), supports Linux, but for different reasons I preferred not to make it available for this configuration. If you want to read more about my decision read [this issue](https://github.com/veeso/termscp/issues/2), while if you think this might have been implemented differently feel free to open an issue with your proposal.

---

## Configuration ⚙️

termscp supports some user defined parameters, which can be defined in the configuration.
Underhood termscp has a TOML file and some other directories where all the parameters will be saved, but don't worry, you won't touch any of these files manually, since I made possible to configure termscp from its user interface entirely.

termscp, like for bookmarks, just requires to have these paths accessible:

- `$HOME/.config/termscp/` on Linux/BSD
- `$HOME/Library/Application Support/termscp` on MacOs
- `FOLDERID_RoamingAppData\termscp\` on Windows

To access configuration, you just have to press `<CTRL+C>` from the home of termscp.

These parameters can be changed:

- **Text Editor**: the text editor to use. By default termscp will find the default editor for you; with this option you can force an editor to be used (e.g. `vim`). **Also GUI editors are supported**, unless they `nohup` from the parent process so if you ask: yes, you can use `notepad.exe`, and no: **Visual Studio Code doesn't work**.
- **Default Protocol**: the default protocol is the default value for the file transfer protocol to be used in termscp. This applies for the login page and for the address CLI argument.
- **Show Hidden Files**: select whether hidden files shall be displayed by default. You will be able to decide whether to show or not hidden files at runtime pressing `A` anyway.
- **Check for updates**: if set to `yes`, termscp will fetch the Github API to check if there is a new version of termscp available.
- **Group Dirs**: select whether directories should be groupped or not in file explorers. If `Display first` is selected, directories will be sorted using the configured method but displayed before files, viceversa if `Display last` is selected.
- **File formatter syntax**: syntax to display file info for each file in the explorer. See [File explorer format](#file-explorer-format)

### SSH Key Storage 🔐

Along with configuration, termscp provides also an **essential** feature for **SFTP/SCP clients**: the SSH key storage.

You can access the SSH key storage, from configuration moving to the `SSH Keys` tab, once there you can:

- **Add a new key**: just press `<CTRL+N>` and you will be prompted to create a new key. Provide the hostname/ip address and the username associated to the key and finally a text editor will open up: paste the **PRIVATE** ssh key into the text editor, save and quit.
- **Remove an existing key**: just press `<DEL>` or `<CTRL+E>` on the key you want to remove, to delete persistently the key from termscp.
- **Edit an existing key**: just press `<ENTER>` on the key you want to edit, to change the private key.

> Q: Wait, my private key is protected with password, can I use it?  
> A: Of course you can. The password provided for authentication in termscp, is valid both for username/password authentication and for RSA key authentication.

### File Explorer Format

It is possible through configuration to define a custom format for the file explorer. This is possible both for local and remote host, so you can have two different syntax in use. These fields, with name `File formatter syntax (local)` and `File formatter syntax (remote)` will define how the file entries will be displayed in the file explorer.
The syntax for the formatter is the following `{KEY1}... {KEY2:LENGTH}... {KEY3:LENGTH:EXTRA} {KEYn}...`.
Each key in bracket will be replaced with the related attribute, while everything outside brackets will be left unchanged.

- The key name is mandatory and must be one of the keys below
- The length describes the length reserved to display the field. Static attributes doesn't support this (GROUP, PEX, SIZE, USER)
- Extra is supported only by some parameters and is an additional options. See keys to check if extra is supported.

These are the keys supported by the formatter:

- `ATIME`: Last access time (with default syntax `%b %d %Y %H:%M`); Extra might be provided as the time syntax (e.g. `{ATIME:8:%H:%M}`)
- `CTIME`: Creation time (with syntax `%b %d %Y %H:%M`); Extra might be provided as the time syntax (e.g. `{CTIME:8:%H:%M}`)
- `GROUP`: Owner group
- `MTIME`: Last change time (with syntax `%b %d %Y %H:%M`); Extra might be provided as the time syntax (e.g. `{MTIME:8:%H:%M}`)
- `NAME`: File name (Elided if longer than 24)
- `PEX`: File permissions (UNIX format)
- `SIZE`: File size (omitted for directories)
- `SYMLINK`: Symlink (if any `-> {FILE_PATH}`)
- `USER`: Owner user

If left empty, the default formatter syntax will be used: `{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`

---

## Text Editor ✏

termscp has, as you might have noticed, many features, one of these is the possibility to view and edit text file. It doesn't matter if the file is located on the local host or on the remote host, termscp provides the possibility to open a file in your favourite text editor.
In case the file is located on remote host, the file will be first downloaded into your temporary file directory and then, **only** if changes were made to the file, re-uploaded to the remote host. termscp checks if you made changes to the file verifying the last modification time of the file.

Just a reminder: **you can edit only textual file**; binary files are not supported.

### How do I configure the text editor 🦥

Text editor is automatically found using this [awesome crate](https://github.com/milkey-mouse/edit), if you want to change the text editor to use, change it in termscp configuration. [Read more](#configuration-️)
