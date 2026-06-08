# 开发者手册

欢迎阅读 termscp 的开发者手册。本章不包含 termscp 各模块的文档，相关文档可以在 Rust Docs 上找到：<https://docs.rs/termscp>。本章描述 termscp 的工作原理，以及实现诸如文件传输和用户界面扩展等功能的指南。

termscp 使用 Rust 编写（edition 2024，MSRV 1.89.0）。用户界面使用 [tuirealm](https://github.com/veeso/tui-realm) v3 构建，它运行在 [crossterm](https://github.com/crossterm-rs/crossterm) 之上。

## termscp 的工作原理

termscp 基本上由 3 个核心模块组成：

- **host**：host 模块提供与文件系统交互的函数。它暴露 `HostBridge` trait，该 trait 抽象了对本地主机（`Localhost`）和远程主机（`RemoteBridged`）的文件操作。
- **ui**：该模块包含用户界面的实现。如下一章所示，这是通过 **activities** 实现的。
- **activity_manager**：activity manager 负责管理 activities。它运行用户界面的 activities，并根据它们的状态决定何时终止当前 activity 以及接下来运行哪个 activity。

除了这 3 个核心模块之外，随着时间推移又添加了其他模块：

- **config**：提供配置 schema 及其序列化方法。
- **explorer**：暴露 explorer 结构，用于处理 ui 中的文件资源管理器。它们存储当前目录模型和视图状态（例如排序、是否显示隐藏文件、传输队列）。
- **filetransfer**：定义 `FileTransferProtocol` 枚举和 `RemoteFsBuilder`，后者根据连接参数构造合适的 `RemoteFs` 客户端。
- **system**：提供与配置、ssh 密钥存储和书签交互的方式。
- **utils**：包含几乎整个项目都会使用的工具。

termscp 支持以下协议：SFTP、SCP、FTP/FTPS、Kube、S3、SMB 和 WebDAV。

## Activities

本段对 activities 做一个简短的概述。请阅读代码和文档，以清晰地了解 ui 的工作方式。

实现用户界面有很多方法。本项目借鉴了不同框架中各自最佳的部分：

- **顶层的 Activities**：每个“视图”都是一个 Activity，并由 `Activity Manager` 来处理它们。这种方法受 Android 启发。它适用于具有不同视图的 ui，每个视图都有自己的组件和逻辑。Activities 与 `Context` 协作，`Context` 是用于在 activities 之间共享数据的数据持有者。
- **Activities 显示 Applications**：每个 activity 可以显示不同的 **Applications**。一个 application 包含一个 **View**，它基本上是一个 **components** 列表，每个组件都有其属性。view 是组件的门面，同时也处理焦点，即当前处于活动状态的组件。你不能拥有多个活动组件，因此必须对此进行处理；与此同时，如果当前组件被销毁，焦点必须交还给之前处于活动状态的组件。**Application** 负责处理所有这些工作。要了解更多信息，请阅读 <https://github.com/veeso/tui-realm>。
- **Components**：components 是围绕 tui 构建的，以便复用控件。这是通过 `Component` trait 实现的，该 trait 受 [React](https://reactjs.org/) 启发。每个组件都有其 *Properties*，并且可以拥有其 *States*。每个组件必须处理输入事件、接受新的属性，并提供一个用于**渲染**自身的方法。这一逻辑现在位于 [tui-realm](https://github.com/veeso/tui-realm) 中。
- **Messages：基于 Elm 的方法**：输入事件采用受 [Elm](https://elm-lang.org/) 启发的方法来处理。在 Elm 中，你使用三个基本函数来实现 ui：**update**、**view** 和 **init**。termscp 将 Elm update 函数的等价实现编写为一个递归函数内部的大型 match 分支，你可以在每个 activity 的 `update.rs` 文件中找到它。这个 match 分支处理组件为响应传入的输入事件而产生的消息，并促使 activity 改变其状态。

termscp 实现了一个名为 `Activity` 的 trait，它是 Android activity 的一个大幅精简版本。该 trait 提供以下方法：

- `on_create`：初始化 activity。context 被传递给 activity，activity 成为 Context 的唯一所有者，直到该 activity 终止。
- `on_draw`：每当用户界面应当更新时被调用。它基本上就是 activity 的运行方法，同时也处理输入事件。界面不应在每次调用时都被绘制（该方法每秒可能被调用数百次），而只应在确实发生变化时（例如在某次输入事件之后）才绘制。
- `will_umount`：返回 activity 是否应当被销毁。如果是，它会返回一个 `ExitReason`，用于指示该 activity 应当终止的原因。activity manager 会根据该原因决定是停止 termscp 的执行，还是启动一个新的 activity 以及启动哪一个。
- `on_destroy`：终结 activity 并将其释放。该方法将 Context 返回给调用方（activity manager）。

### Context

context 是一个保存 activities 之间共享数据的结构。每次某个 Activity 启动时，Context 都会被该 activity 取走，直到它被销毁时，context 才最终返回给 activity manager。context 保存以下数据：

- **Localhost**：本地主机结构。
- **File Transfer Params**：当前用于连接到远程主机的参数。
- **Config Client**：一个提供访问用户配置的函数的结构。
- **Store**：一个可以保存任意类型数据的键值存储。它可用于在 activities 之间共享状态，或为繁重或缓慢的任务（例如检查更新）保持持久化。
- **Terminal**：用于在终端上渲染 tui。

## 实现抽象的文件传输客户端

当 termscp 的实现于 2020 年 12 月开始时，文件传输处于设计的核心，因为它是 termscp 的核心所在。最初的实现由一个 `filetransfer` 模块组成，该模块暴露了一个名为 `FileTransfer` 的 trait，它提供了与远程文件系统进行通用交互的方法。

随着时间推移，由于不同用户都希望有一个专门的库，这种情况发生了变化。在 2021 年最后一个季度，[remotefs](https://github.com/veeso/remotefs-rs) 诞生了：一个用于操作远程设备文件系统的抽象库。remotefs 提供了一个 `RemoteFs` trait，它暴露了所有核心文件系统功能，并且自 0.8.0 版本起，它已经取代了 `FileTransfer` trait。

文件传输模块仍然存在，但它唯一的任务是通过 `RemoteFsBuilder` 根据文件传输参数构建一个 `RemoteFs` 客户端实现。
