# 密码安全

书签连同其密码一起保存在你的配置目录中。密码不会以明文存储：它们使用 AES 加密。

用于加密密码的密钥会尽可能存储在操作系统的密钥存储中：

- Windows 上的 Windows Vault
- Linux 上的系统密钥环
- macOS 上的 Keychain

这由你的操作系统直接管理。

在 BSD 和 WSL 上没有这样的密钥存储，因此加密密钥会保存在磁盘上的 `$HOME/.config/termscp`。该位置通过文件权限保护密钥，使其无法被其他用户读取，但你仍应避免在这些系统上为暴露于互联网上的服务器保存密码。

## Linux 密钥环

在 Linux 上，你的系统中可能没有安装密钥环。密钥存储需要一个在 D-Bus 上暴露 `org.freedesktop.secrets` 的服务，而只有少数服务提供它：

- 如果你使用 GNOME 作为桌面环境（例如 Ubuntu 用户），密钥环已经由 `gnome-keyring` 提供，一切应该开箱即用。
- 对于其他桌面环境，你可以使用 [KeepassXC](https://keepassxc.org/) 来获取一个密钥环。它必须经过设置才能与 termscp 配合使用；参见下面的 [KeepassXC 设置](#keepassxc-设置)。
- 如果你不想安装这些服务中的任何一个，termscp 会照常工作，并回退到将密钥保存在文件中，正如它在 BSD 和 WSL 上所做的那样。

### KeepassXC 设置

按照以下步骤为 termscp 设置 KeepassXC：

1. 安装 KeepassXC。
2. 在工具栏中进入 "Tools" > "Settings"。
3. 选择 "Secret service integration" 并启用 "Enable KeepassXC freedesktop.org secret service integration"。
4. 如果你还没有数据库，请创建一个：在工具栏中，"Database" > "New database"。
5. 在工具栏中，进入 "Database" > "Database settings"。
6. 选择 "Secret service integration" 并启用 "Expose entries under this group"。
7. 选择将保存 termscp 密钥的组。请注意，任何其他应用程序都可以读取通过 D-Bus 为该组暴露的密钥。
