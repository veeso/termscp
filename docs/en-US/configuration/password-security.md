# Password security

Bookmarks are saved in your configuration directory along with their passwords.
Passwords are not stored in plaintext: they are encrypted with AES.

The key used to encrypt passwords is stored, where possible, in the operating
system secret store:

- The Windows Vault on Windows
- The system keyring on Linux
- The Keychain on macOS

This is managed directly by your operating system.

On BSD and WSL there is no such secret store, so the encryption key is saved on
disk at `$HOME/.config/termscp`. The location protects the key with file
permissions so that it cannot be read by other users, but you should still avoid
saving passwords for servers exposed on the internet on these systems.

## Linux keyring

On Linux there might be no keyring installed on your system. The key storage
requires a service that exposes `org.freedesktop.secrets` on D-Bus, and only a
few services provide it:

- If you use GNOME as your desktop environment (e.g. Ubuntu users), the keyring
  is already provided by `gnome-keyring` and everything should work out of the
  box.
- For other desktop environments, you can use [KeepassXC](https://keepassxc.org/)
  to obtain a keyring. It must be set up to work with termscp; see
  [KeepassXC setup](#keepassxc-setup) below.
- If you do not want to install any of these services, termscp keeps working as
  usual and falls back to saving the key in a file, as it does for BSD and WSL.

### KeepassXC setup

Follow these steps to set up KeepassXC for termscp:

1. Install KeepassXC.
2. Go to "Tools" > "Settings" in the toolbar.
3. Select "Secret service integration" and enable "Enable KeepassXC
   freedesktop.org secret service integration".
4. Create a database, if you do not have one yet: from the toolbar, "Database" >
   "New database".
5. From the toolbar, go to "Database" > "Database settings".
6. Select "Secret service integration" and enable "Expose entries under this
   group".
7. Select the group where the termscp secret will be kept. Note that any other
   application can read secrets exposed via D-Bus for that group.
