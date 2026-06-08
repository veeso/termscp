# 书签与最近主机

termscp 允许你将常用的主机保存为书签，随后可从主界面快速加载它们。termscp 还会保留你最近连接过的 16 个主机。这两项功能都让你只需在认证表单下方的选项卡中选择一个条目，即可重新加载连接到某个远程所需的全部参数。

## 书签的存储位置

书签会在可能的情况下保存在配置目录中：

- Linux/BSD 上为 `$HOME/.config/termscp/`
- macOS 上为 `$HOME/Library/Application Support/termscp`
- Windows 上为 `FOLDERID_RoamingAppData\termscp\`

## 保存密码

仅对于书签，你可以选择性地保存用于认证的密码。这不适用于最近主机，它们从不保存密码。默认情况下不保存密码；在你创建新书签时，系统会提示你选择是否保存。

如果你担心为书签保存的密码的安全性，请参阅[我的密码安全吗？](../configuration/password-security.md)

## 创建书签

1. 在认证表单中填写连接到远程服务器的参数。
2. 按 `<CTRL+S>`。
3. 输入你想为书签指定的名称。
4. 选择是否记住密码。
5. 按 `<ENTER>` 提交。

## 加载书签

要使用先前保存的连接，请按 `<TAB>` 导航到书签列表，然后按 `<ENTER>` 将书签参数加载到表单中。

![Bookmarks](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)
