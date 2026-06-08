# SSH key storage

Along with configuration, termscp provides an essential feature for SFTP/SCP
clients: the SSH key storage.

To access the SSH key storage, enter the configuration and move to the
`SSH Keys` tab.

## Manage keys

From the `SSH Keys` tab you can:

- **Add a new key**: press `<CTRL+N>`. You are prompted to provide the
  hostname/IP address and the username associated with the key, then a text
  editor opens: paste the **private** SSH key into the editor, save and quit.
- **Remove an existing key**: press `<DEL>` or `<CTRL+E>` on the key you want to
  remove to delete it persistently from termscp.
- **Edit an existing key**: press `<ENTER>` on the key you want to edit to change
  the private key.

## Password-protected keys

Password-protected private keys are supported. The password you provide for
authentication in termscp is valid both for username/password authentication and
for key authentication.
