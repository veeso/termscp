# 日志

termscp 会为每个会话写入一个日志文件，位于：

- Linux/BSD 上的 `$HOME/.cache/termscp/termscp.log`
- macOS 上的 `$HOME/Library/Caches/termscp/termscp.log`
- Windows 上的 `FOLDERID_LocalAppData\termscp\termscp.log`

日志不会轮转：每次启动 termscp 时都会被截断。如果你想报告问题并附上日志文件，请在再次启动 termscp 之前将日志保存到安全的位置。

默认情况下，日志以 `INFO` 级别记录，因此不是很详细。

## 以 TRACE 级别复现问题

要提交问题，请通过使用 `-D` 命令行选项启动 termscp，将日志级别设置为 `TRACE` 来复现问题。

## 禁用日志

要关闭日志，请使用 `-q` 或 `--quiet` 选项启动 termscp。你可以为 termscp 设置别名，使其永久生效。

## 安全性

日志文件不包含任何明文密码。它暴露的信息与同级的 `bookmarks` 文件相同。
