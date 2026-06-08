# 通知

termscp 会针对以下事件发送桌面通知：

- **传输完成**：在传输成功完成后发送。仅当总传输大小至少达到所配置的 `Notifications: minimum transfer size` 时才会显示。
- **传输失败**：在传输因错误而失败后发送。仅当总传输大小至少达到所配置的 `Notifications: minimum transfer size` 时才会显示。
- **有可用更新**：每当有新版本的 termscp 可用时发送。
- **更新已安装**：每当新版本的 termscp 安装完成时发送。
- **更新失败**：每当更新安装失败时发送。

## 禁用通知

要关闭通知，请进入设置并将 `Enable notifications?` 设置为 `No`。

## 更改最小传输大小

要更改用于控制传输通知的阈值，请进入设置并将 `Notifications: minimum transfer size` 设置为适合你的值。
