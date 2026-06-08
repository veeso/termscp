# 配置

termscp 支持大量用户自定义参数。termscp 将这些参数存储在一个 TOML 文件和若干目录中，但你无需手动编辑这些文件：所有配置都完全在用户界面中完成。

要进入配置界面，请在 termscp 主页按下 `<CTRL+C>`。

termscp 要求以下路径可访问：

- Linux/BSD 上的 `$HOME/.config/termscp/`
- macOS 上的 `$HOME/Library/Application Support/termscp`
- Windows 上的 `FOLDERID_RoamingAppData\termscp\`

## 参数

可以配置以下参数：

- **文本编辑器**：要使用的文本编辑器。默认情况下，termscp 会为你查找默认编辑器；通过此选项你可以强制使用某个编辑器（例如 `vim`）。也支持 GUI 编辑器，除非它们会从父进程中分离（`nohup`）。
- **默认协议**：termscp 中使用的文件传输协议的默认值。它适用于登录页面以及地址命令行参数。
- **显示隐藏文件**：是否默认显示隐藏文件。你也可以在运行时按 `A` 切换隐藏文件的显示。
- **检查更新**：如果设置为 `yes`，termscp 会查询 GitHub API 以检查是否有新版本的 termscp 可用。
- **替换已有文件时提示**：如果设置为 `yes`，每当文件传输会替换目标主机上已有的文件时，termscp 都会请求确认。
- **目录分组**：文件浏览器中是否将目录分组。如果选择 `Display first`，目录会按配置的方法排序，但显示在文件之前；如果选择 `Display last`，则显示在文件之后。
- **远程文件格式化语法**：用于在远程浏览器中显示每个文件信息的语法。参见 [文件浏览器格式](explorer-format.md)。
- **本地文件格式化语法**：用于在本地浏览器中显示每个文件信息的语法。参见 [文件浏览器格式](explorer-format.md)。
- **启用通知**：如果设置为 `Yes`，则会显示桌面通知。参见 [通知](notifications.md)。
- **通知：最小传输大小**：如果传输大小大于或等于指定值，则显示传输通知。可接受的格式为 `{UNSIGNED} B/KB/MB/GB/TB/PB`。
- **SSH 配置路径**：连接到 SCP/SFTP 服务器时使用的 SSH 配置文件。如果留空，则不使用任何文件。你可以指定以 `~` 开头的路径来表示主目录（例如 `~/.ssh/config`）。termscp 支持的属性列于 [ssh2-config 公开的属性](https://github.com/veeso/ssh2-config#exposed-attributes)。另请参见 [SSH 密钥存储](ssh-keys.md)。
