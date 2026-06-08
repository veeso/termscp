# 主题

termscp 允许你为应用程序中的多个组件设置颜色。有两种方式可以自定义 termscp：

- 通过 **配置菜单**
- 导入 **主题文件**

## 通过配置菜单自定义

要通过配置菜单自定义 termscp，请在认证界面按 `<CTRL+C>` 进入配置，然后按两次 `<TAB>` 到达 `themes` 面板。使用 `<UP>` 和 `<DOWN>` 移动以选择你想更改的样式，如下面的 gif 所示：

![Themes](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)

## 导入主题文件

你也可以导入主题文件。你可以从仓库 `themes/` 目录中随 termscp 附带的某个主题获取灵感，或直接使用它。通过运行以下命令导入主题：

```sh
termscp theme <theme_file>
```

如果一切正常，termscp 会确认主题已导入。

## 颜色语法

termscp 接受以下颜色格式：

- 显式十六进制：`#rrggbb`
- RGB：`rgb(r, g, b)`
- [CSS 颜色名称](https://www.w3schools.com/cssref/css_colors.asp)（例如 `crimson`）
- 特殊关键字 `Default`，它使用与情境相关的默认前景色或背景色（文本和线条使用前景色，其余使用背景色）

## 从无法加载的主题中恢复

更新后，已保存的主题可能无法加载。这发生在向主题添加新键时：之前保存的主题不再包含该键。有两种快速修复方法：

1. 重新导入官方主题。每次发布后，官方主题都会被修补，因此从仓库下载更新后的主题并重新导入：

    ```sh
    termscp theme <theme.toml>
    ```

2. 手动编辑你的主题。如果你使用自定义主题，请编辑该文件并添加缺失的键。主题位于 `$CONFIG_DIR/termscp/theme.toml`，其中 `$CONFIG_DIR` 为：

    - FreeBSD/Linux：`$HOME/.config/`
    - macOS：`$HOME/Library/Application Support`
    - Windows：`%appdata%`

    缺失的键会在你刚安装的版本的 CHANGELOG 中 `BREAKING CHANGES` 部分列出。

## 样式

下面的表格描述了每个样式字段。请注意，样式 **不** 适用于配置页面，因此即使你不小心更改了某些内容，配置页面也始终保持可用。

### 认证页面

| 键               | 说明                       |
| ---------------- | -------------------------- |
| `auth_address`   | IP 地址输入框的颜色        |
| `auth_bookmarks` | 书签面板的颜色             |
| `auth_password`  | 密码输入框的颜色           |
| `auth_port`      | 端口号输入框的颜色         |
| `auth_protocol`  | 协议单选框组的颜色         |
| `auth_recents`   | 最近记录面板的颜色         |
| `auth_username`  | 用户名输入框的颜色         |

### 传输页面

| 键                                     | 说明                                               |
| -------------------------------------- | -------------------------------------------------- |
| `transfer_local_explorer_background`   | 本地主机浏览器的背景色                             |
| `transfer_local_explorer_foreground`   | 本地主机浏览器的前景色                             |
| `transfer_local_explorer_highlighted`  | 本地主机浏览器的边框及高亮颜色                     |
| `transfer_remote_explorer_background`  | 远程浏览器的背景色                                 |
| `transfer_remote_explorer_foreground`  | 远程浏览器的前景色                                 |
| `transfer_remote_explorer_highlighted` | 远程浏览器的边框及高亮颜色                         |
| `transfer_log_background`              | 日志面板的背景色                                   |
| `transfer_log_window`                  | 日志面板的窗口颜色                                 |
| `transfer_progress_bar_partial`        | 部分进度条的颜色                                   |
| `transfer_progress_bar_total`          | 总进度条的颜色                                     |
| `transfer_status_hidden`               | 状态栏 "hidden" 标签的颜色                         |
| `transfer_status_sorting`              | 状态栏 "sorting" 标签的颜色；也适用于文件排序对话框 |
| `transfer_status_sync_browsing`        | 状态栏 "sync browsing" 标签的颜色                 |

### 杂项

这些样式适用于应用程序的不同部分。

| 键                  | 说明                             |
| ------------------- | -------------------------------- |
| `misc_error_dialog` | 错误消息的颜色                   |
| `misc_info_dialog`  | 信息对话框的颜色                 |
| `misc_input_dialog` | 输入对话框的颜色（例如复制文件） |
| `misc_keys`         | 按键文本的颜色                   |
| `misc_quit_dialog`  | 退出对话框的颜色                 |
| `misc_save_dialog`  | 保存对话框的颜色                 |
| `misc_warn_dialog`  | 警告对话框的颜色                 |
