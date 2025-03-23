# 操作指南 🎓

- [操作指南 🎓](#操作指南-)
  - [用法](#用法)
    - [地址参数](#地址参数)
      - [AWS S3 地址参数](#aws-s3-地址参数)
      - [Kube 地址参数](#kube-地址参数)
      - [WebDAV 地址参数](#webdav-地址参数)
      - [SMB 地址参数](#smb-地址参数)
      - [如何输入密码](#如何输入密码)
  - [S3 连接参数](#s3-连接参数)
    - [Aws S3 凭证](#aws-s3-凭证)
  - [文件浏览](#文件浏览)
    - [快捷键](#快捷键)
    - [操作多个文件 🥷](#操作多个文件-)
      - [示例](#示例)
    - [同步浏览](#同步浏览)
    - [打开/打开方式](#打开打开方式)
  - [书签](#书签)
    - [我的密码安全吗？](#我的密码安全吗)
      - [Linux Keyring](#linux-keyring)
        - [用于 termscp 的 KeepassXC 设置](#用于-termscp-的-keepassxc-设置)
  - [配置](#配置)
    - [SSH Key Storage](#ssh-key-storage)
    - [资源管理器格式](#资源管理器格式)
  - [主题](#主题)
    - [样式](#样式)
    - [我的主题无法加载](#我的主题无法加载)
      - [登录页](#登录页)
      - [文件传输页](#文件传输页)
      - [Misc](#misc)
  - [文本编辑器](#文本编辑器)
  - [日志](#日志)
  - [通知](#通知)
  - [文件观察者🔭](#文件观察者)

## 用法

termscp启动时可以使用以下选项:

`termscp [options]... [protocol://user@address:port:wrkdir] [protocol://user@address:port:wrkdir] [local-wrkdir]`

或作为

`termscp [options]... -b [bookmark-name] -b [bookmark-name] [local-wrkdir]`

- `-P, --password <password>` 登陆密码
- `-b, --address-as-bookmark` 将地址参数解析为书签名称
- `-q, --quiet` 禁用日志
- `-v, --version` 打印版本信息
- `-h, --help` 打开帮助

termscp有两种不同的启动模式，不带参数时，termscp将显示登录表单页，用户可以填写连接到远程服务端所需的参数。

或者，用户可以提供一个url作为参数，跳过认证页，直接与远程服务器进行连接。

如果提供了url参数，你也可以提供本地主机的起始工作目录。

### 地址参数

地址参数的格式如下：

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

让我们通过一些例子熟悉这个特殊语法，它很好上手，你会很快习惯并且替代其他工具......

- 使用默认协议（*在配置中定义*）连接到192.168.1.31，如果没有提供端口，则为所选协议的默认端口（取决于你的配置）；用户名是系统当前用户名

    ```sh
    termscp 192.168.1.31
    ```

- 使用默认协议（*在配置中定义*）连接到192.168.1.31，用户名为`root`

    ```sh
    termscp root@192.168.1.31
    ```

- 使用scp连接到192.168.1.31, 端口号为4022; 用户名为 `omar`

    ```sh
    termscp scp://omar@192.168.1.31:4022
    ```

- 使用scp连接到192.168.1.31, 端口号为4022; 用户名为 `omar`。工作目录为 `/tmp`

    ```sh
    termscp scp://omar@192.168.1.31:4022:/tmp
    ```

#### AWS S3 地址参数

出于显而易见的原因，Aws S3 对 CLI 地址参数有不同的语法，但我设法使其与通用地址参数尽可能相似：

```txt
s3://<bucket-name>@<region>[:profile][:/wrkdir]
```

例如

```txt
s3://buckethead@eu-central-1:default:/assets
```

#### Kube 地址参数

如果您想连接到 Kube，请使用以下语法

```txt
kube://[namespace][@<cluster_url>][$</path>]
```

#### WebDAV 地址参数

如果您想要连接到 WebDAV，请使用以下语法

```txt
http://<username>:<password>@<url></path>
或者如果您想要使用 https
```

```txt
https://<username>:<password>@<url></path>
```

#### SMB 地址参数

SMB 对 CLI 地址参数有不同的语法，无论您是在 Windows 还是其他系统上，这都是不同的：

**Windows** 句法:

```txt
\\[username@]<server-name>\<share>[\path\...]
```

**其他系统** 句法:

```txt
smb://[username@]<server-name>[:port]/<share>[/path/.../]
```

#### 如何输入密码

你可能已经注意到，url参数中没有办法直接附加密码，你可以通过以下三种方式提供密码：

- `-P, --password` 不推荐：直接在参数中填写明文密码。强烈不推荐这种方法，因为它非常不安全（因为你可能会把密码保留在shell历史记录中）。
- 通过 `sshpass`: 你可以通过 `sshpass` 传入密码, 例如： `sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31`
- 提示输入密码：如果你不使用前面的任何方法，你会被提示输入密码，就像 `scp`、`ssh` 等比较经典的工具上一样。

---

## S3 连接参数

这些参数是连接到 aws s3 和其他 s3 兼容服务器所必需的：

- AWS S3:
  - **bucket name**
  - **region**
  - *profile* (如果未提供：“default”)
  - *access key* (除非公开)
  - *secret access key* (除非公开)
  - *security token* (如果需要的话)
  - *session token* (如果需要的话)
  - new path style: **NO**
- 其他 S3 端点:
  - **bucket name**
  - **endpoint**
  - *access key* (如果需要的话)
  - *secret access key* (如果需要的话)
  - new path style: **YES**

### Aws S3 凭证

为了连接到 Aws S3 存储桶，您显然必须提供一些凭据。
因此，您可以通过以下方式为 s3 提供凭据：

1. 认证形式：
   1. 您可以提供 `access_key`（应该是强制性的）、`secret_access_key`（应该是强制性的）、`security_token` 和`session_token`
   2. 如果您将 s3 连接保存为书签，这些凭据将在您的书签文件中保存为加密的 AES-256/BASE64 字符串（安全令牌和会话令牌除外，它们是临时凭据）。.
2. 使用您的凭证文件：只需通过`aws configure` 配置AWS cli，您的凭证应该已经位于`~/.aws/credentials`。 如果您使用的配置文件不同于“默认”，只需在身份验证表单的配置文件字段中提供它。
3. **环境变量**: 您始终可以将您的凭据作为环境变量提供。 请记住，这些凭据**将始终覆盖**位于 `credentials` 文件中的凭据。 下面看看如何配置环境：

    这些应该始终是强制性的:

    - `AWS_ACCESS_KEY_ID`: aws 访问密钥 ID（通常以 `AKIA...` 开头）
    - `AWS_SECRET_ACCESS_KEY`: 秘密访问密钥

    如果您配置了更强的安全性，您*可能*也需要这些：

    - `AWS_SECURITY_TOKEN`: 安全令牌
    - `AWS_SESSION_TOKEN`: 会话令牌

⚠️ 您的凭据是安全的：termscp 不会直接操作这些值！ 您的凭据直接由 **s3** crate 使用。
如果您对安全有一些担忧，请联系 [Github](https://github.com/durch/rust-s3) 上的库作者 ⚠️

---

## 文件浏览

termscp中的文件资源管理器是指你与远程建立连接后可以看到的面板。
面板由3个部分组成（是的，就这三个）：

- 本地资源管理器面板：它显示在你的屏幕左侧，显示localhost的当前目录文件列表。
- 远程资源管理器面板：它显示在你屏幕的右边，显示远程主机的当前目录文件列表。
- 查找结果面板：根据你搜索文件的位置（本地/远程），它将取代对应资源管理器面板。这个面板显示与你执行的搜索查询相匹配的条目。

为了切换面板，你需要输入 `<LEFT>` 来移动远程资源管理器面板，`<RIGHT>` 来移动回本地资源管理器面板。当在查找结果面板时，你需要按`<ESC>`来退出面板，回到前一个面板。

### 快捷键

| 按键           | 命令                                               | 助记词    |
|---------------|-------------------------------------------------------|-------------|
| `<ESC>`       | 断开远程连接；回到登录页                              |             |
| `<BACKSPACE>` | 返回上一次目录                                        |             |
| `<TAB>`       | 切换资源管理器选项卡                                  |             |
| `<RIGHT>`     | 切换到远程管理器面板                                  |             |
| `<LEFT>`      | 切换到本地管理器面板                                  |             |
| `<UP>`        | 在当前列表中向上移动光标                              |             |
| `<DOWN>`      | 在当前列表中向下移动光标                              |             |
| `<PGUP>`      | 在当前列表中光标上移8行                               |             |
| `<PGDOWN>`    | 在当前列表中光标下移8行                               |             |
| `<ENTER>`     | 进入文件夹                                            |             |
| `<SPACE>`     | 上传 / 下载选中文件                                   |             |
| `<BACKTAB>`   | 在日志面板和管理器面板之间切换                        |             |
| `<A>`         | 是否显示隐藏文件                                      | All         |
| `<B>`         | 按..排序                                              | Bubblesort? |
| `<C|F5>`      | 复制文件（夹）                                        | Copy        |
| `<D|F7>`      | 创建文件夹                                            | Directory   |
| `<E|F8|DEL>`  | 删除文件                                    | Erase       |
| `<F>`         | 文件搜索 (支持通配符)                                 | Find        |
| `<G>`         | 跳转到指定路径                                        | Go to       |
| `<H|F1>`      | 显示帮助                                              | Help        |
| `<I>`         | 显示选中文件（夹）信息                                | Info        |
| `<K>`         | 创建指向当前选定条目的符号链接 | symlinK     |
| `<L>`         | 刷新当前目录列表 / 清除选中状态                       | List        |
| `<M>`         | 选中文件                                              | Mark        |
| `<N>`         | 使用键入的名称新建文件                                | New         |
| `<O|F4>`      | 编辑文件；参考文本编辑器文档                          | Open        |
| `<P>`         | 打开日志面板                                          | Panel       |
| `<Q|F10>`     | 退出termscp                                           | Quit        |
| `<R|F7>`      | 重命名文件                                            | Rename      |
| `<S|F2>`      | 另存为...                                             | Save        |
| `<T>`         | 显示所有同步路径          | Track       |
| `<U>`         | 进入上层目录                                          | Upper       |
| `<V|F3>`      | 使用默认方式打开文件                                  | View        |
| `<W>`         | 使用指定程序打开文件                                  | With        |
| `<X>`         | 运行命令                                              | eXecute     |
| `<Y>`         | 是否开启同步浏览                                      | sYnc        |
| `<Z>`         | 更改文件权限                                      |             |
| `</>`         | 过滤文件（支持正则表达式和通配符匹配）    |             |
| `<CTRL+A>`    | 选中所有文件                                          |             |
| `<ALT+A>`    | 取消选择所有文件                                          |             |
| `<CTRL+C>`    | 终止文件传输                                          |             |
| `<CTRL+T>`    | 显示所有同步路径                             | Track       |

### 操作多个文件 🥷

你可以通过以下简单的控制操作多个文件：

- `<M>`：标记文件以进行选择
- `<CTRL+A>`：选择当前目录下的所有文件
- `<ALT+A>`：取消选择所有文件

被标记的文件将会以**高亮背景** 显示。
当进行选择操作时，只有被选中的文件会执行操作，而当前高亮显示的项目会被忽略。

即使是在查找结果面板中，也可以操作多个文件。

在操作多个文件时，所有功能都可用，但某些功能会有些许不同。具体如下：

- *复制*：复制时会提示你输入目标名称。操作多个文件时，该名称是目标目录，所有文件将被复制到此目录中。
- *重命名*：与复制相同，但文件将被移动到该目录。
- *另存为*：与复制相同，但文件将被写入该目录。

如果你在某个目录（如 `/home`）中选择了文件，然后切换目录，文件仍会保持被选中状态，并在底部面板的**传输队列** 中显示。
文件被选中时，会将当前*远程*目录与该文件关联；如果文件被传输，它将被传输到与之关联的目录中。

#### 示例

如果我们在本地选择 `/home/a.txt`，此时远程目录是 `/tmp`，然后我们切换到 `/var`，选择 `/var/b.txt`，而此时远程目录为 `/home`，执行传输后的结果为：

- `/home/a.txt` 传输到 `/tmp/a.txt`
- `/var/b.txt` 传输到 `/home/b.txt`

### 同步浏览

启用时，同步浏览将允许你在两个面板之间同步导航操作。这意味着，每当你在一个面板上改变工作目录时，同样的动作会在另一个面板上重现。如果你想启用同步浏览，只需按下`<Y>`；按两次就可以禁用。当启用时，同步浏览的状态将在状态栏上显示为`ON`。

### 打开/打开方式

打开和打开方式的功能是由 [open-rs](https://docs.rs/crate/open/2.1.0)提供的。
执行视图命令（`<V>`）时，关联该文件类型的系统默认应用程序会被调用以打开当前文件。这依赖于操作系统默认的服务，所以要确保你的系统中至少安装了一个这样的服务：

- **Windows** 用户: 无需额外操作，程序内部会调用 `start` 命令。
- **MacOS** 用户: 同样无需额外操作，程序内部会调用系统内置的 `open` 命令。
- **Linux** 用户: 以下程序之一需要被安装：
  - *xdg-open*
  - *gio*
  - *gnome-open*
  - *kde-open*
- **WSL** 用户: *wslview* 是必要的，你需要安装 [wslu](https://github.com/wslutilities/wslu).

> Q: 我可以使用V命令编辑远程文件吗？
> A: 不可以，至少不能在 "远程管理面板 "上直接操作。你必须先把它下载到本地目录，这是由于当你打开一个远程文件时，该文件会被下载到一个临时目录中，但没有办法监控这个文件的状态，同时也无法得知你用来打开它的程序何时被关闭。也就是说，termscp无法获知你何时完成对该文件的编辑。

---

## 书签

在termscp中，你可以保存常用的服务器，随后可以从termscp的主界面中快速连接到这些服务器。termscp也会在历史记录中保存你最后连接的16个主机。这个功能保留了连接到某个远程服务器的所有参数，只需在登录页下方的Tab中选中书签即可。

书签会尝试被保存在以下路径：

- `$HOME/.config/termscp/` -- Linux/BSD
- `$HOME/Library/Application Support/termscp` -- MacOs
- `FOLDERID_RoamingAppData\termscp\` -- Windows

对于书签（不包括服务器连接历史记录）而言，也可以保存用于验证的密码。注意默认情况下不保存密码，必须在保存新书签时通过提示指定密码。

如果您担心为您的书签保存的密码的安全性，请阅读[以下章节](#我的密码安全吗？)👀

请按照以下步骤新建书签：

1. 在认证页中输入待连接服务器的参数
2. 按 `<CTRL+S>`
3. 输入书签名称
4. 选择是否保留密码
5. 按 `<ENTER>` 提交

无论何时你想使用以前保存的连接，只需按下`<TAB>`导航到书签列表，然后按`<ENTER>`将书签参数加载到表格中。

![Bookmarks](https://github.com/veeso/termscp/blob/main/assets/images/bookmarks.gif?raw=true)

### 我的密码安全吗？

这是当然 😉。
如前所述，书签与密码一起保存在你的配置目录中。密码显然不是纯文本，它们是用**AES-128**加密的。这够不够安全？绝对够 (BSD和WSL用户除外😢)

在**Windows**、**Linux**和**MacOS**上，如果可能的话（但应该是），密码会被分别存储在*Windows Vault*、*系统钥匙串*和*Keychain*中。这实际上是超级安全的，并且是由你的操作系统直接管理的。

❗请注意，如果你是一个Linux用户，你真的应该阅读[下面的章节👀](#linux-keyring)，因为你的系统可能没有启用或支持钥匙串功能

另一方面，在*BSD*和*WSL*上，用于加密密码的密钥是存储在你的驱动器上的（在$HOME/.config/termscp）。那么，仍然有可能检索到密钥来解密密码。幸运的是，密钥的位置保证了你的密钥不能被与你不同的用户读取，但是，是的，我仍然不会为暴露在互联网上的服务器保存密码😉。

#### Linux Keyring

我们都喜欢Linux，因为它给了用户自由。作为一个Linux用户，你基本上可以做任何你想做的事情，但这也有一些缺点，比如说，不同的发行版之间往往没有标准的应用程序。这也涉及到钥匙串。
这意味着，在Linux上，你的系统可能没有安装钥匙串。不幸的是，我们用来处理钥匙存储的库需要一个在D-BUS上公开`org.freedesktop.secrets`的服务，而最糟糕的事实是，只有两个服务在公开它。

- ❗ 如果你使用GNOME作为桌面环境（例如ubuntu用户），那么你是幸运的，因为钥匙串已经由`gnome-keyring`提供，一切都应该已经准备就绪了。
- ❗对于其他桌面环境的用户，有一个很好的程序，你可以用它来获得钥匙串，这就是[KeepassXC](https://keepassxc.org/)，我在我的Manjaro中使用它（带KDE），一切都很正常。唯一的问题是，你必须设置它与termscp一起使用（但这很简单）。要开始使用KeepassXC，请阅读更多[这里]（#keepassxc-setup-for-termscp）。
- ❗如果你不想安装任何这些服务呢？好吧，这没有问题! **termscp依然能正常工作**，但它会将密钥保存在一个文件中，就像它通常为BSD和WSL做的那样。

##### 用于 termscp 的 KeepassXC 设置

参照以下步骤，为termscp配置keepassXC：

1. 安装KeepassXC
2. 进入工具栏的 "工具">"设置"。
3. 选择 "秘密服务集成 "并切换 "启用KeepassXC freedesktop.org secret服务集成"
4. 创建一个数据库，如果你还没有：在工具栏的 "数据库">"新数据库"
5. 从工具条上 "数据库" > "数据库设置"
6. 选择 "secret服务集成 "并切换 "在此组下暴露条目"
7. 7.在列表中选择你希望termscp secret保存到的组。记住，这个组可能被任何其他应用程序通过DBUS存储密钥。

---

## 配置

termscp支持一些用户定义参数，这些参数可以通过配置来修改。
termscp有一个TOML文件和一些其他的目录，所有的参数都会被保存在这些目录中，但是不用担心，你不必手动编辑这些文件，因为我为termscp制作了可交互的用户界面。

termscp和书签一样，只需要保证这些路径是可访问的：

- `$HOME/.config/termscp/` -- Linux/BSD
- `$HOME/Library/Application Support/termscp` -- MacOs
- `FOLDERID_RoamingAppData\termscp\` -- Windows

要访问配置，你只需在termscp的主界面上按`<CTRL+C>`。

这些参数可以自定义：

- **Text Editor**：使用的文本编辑器。默认情况下，termscp将为你找到默认的编辑器；通过这个选项，你可以强制使用一个编辑器（如`vim`）。**也支持GUI编辑器**，除非它们从父进程中`nohup`。所以，如果这是你的问题：是的，你可以使用`notepad.exe`，然而，**Visual Studio Code不工作**。
- **Default Protocol**：默认协议是termscp中默认使用的文件传输协议。这适用于登录页和地址中的CLI参数。
- **Show Hidden Files**：选择是否应默认显示隐藏文件。你可以在运行时按 `A` 来切换是否显示隐藏的文件。
- **Check for updates**：如果设置为 `yes`，termscp将通过Github API检查是否有新版本的termscp。
- **Prompt when replacing existing files?**: 如果设置为 `yes`，则在文件传输会导致目标主机上的现有文件被替换时，termscp 将提示您确认。
- **Group Dirs**：选择在文件浏览器中是否对文件夹进行分组。如果选择 `Display first`，目录将根据设置的方法排序，但仍显示在文件之前；如果选择 `Display last`，则正好相反。
- **Remote File formatter syntax**：在远程资源管理器中为每个文件显示文件信息的语法。参见[资源管理器格式](#资源管理器格式)
- **Local File formatter syntax**：在本地资源管理器中显示每个文件的文件信息的语法。参见[资源管理器格式](#资源管理器格式)
- **Enable notifications?**: 如果设置为 `Yes`，则会显示通知。
- **Notifications: minimum transfer size**: 如果传输大小大于或等于指定值，将显示传输通知。 接受的值格式为 `{UNSIGNED} B/KB/MB/GB/TB/PB`
- **SSH Configuration path**：设置连接到 SCP/SFTP 服务器时使用的 SSH 配置文件。 如果未设置（空），则不会使用任何文件。 你可以指定一个以 `~` 开头的路径来表示主路径（例如 `~/.ssh/config`）. 指定了 termscp 支持的参数 [HERE](https://github.com/veeso/ssh2-config#exposed-attributes).

### SSH Key Storage

配置选项还包括termscp为**SFTP/SCP客户端**提供的一个**必要**功能：SSH密钥存储。

你可以从配置中切换到到 `SSH Keys` tab页来访问SSH密钥存储，在那里你可以：

- **添加新密钥**：只需按下`<CTRL+N>`，你将被提示创建一个新的密钥。提供主机名/ip地址和与该钥匙关联的用户名，最后会打开一个文本编辑器：将**PRIVATE** SSH key粘贴到文本编辑器中，保存并退出。
- **删除现有密钥**：只要在你想删除的密钥上按下`<DEL>`或`<CTRL+E>`，就可以从 termscp 中永久删除该密钥。
- **编辑现有密钥**：只需在你想编辑的密钥上按下`<ENTER>`，就可以修改私钥。

> 问：等等，我的私钥受密码保护，也是可以用的吗？
> 答：当然可以。termscp中提供的认证密码，对用户名/密码认证和RSA密钥认证都有效。

### 资源管理器格式

可以为文件浏览器配置自定义的格式，本地和远程主机允许进行单独设定，所以你可以使用两种不同的语法。这些字段的名称为 `File formatter syntax (local)` 和 `File formatter syntax (remote)`，将定义文件条目如何在文件资源管理器中显示。
格式化的语法如下 `{KEY1}... {KEY2:LENGTH}... {KEY3:LENGTH:EXTRA} {KEYn}...`。
花括号内的每个键将被替换成相关的属性，而括号外的所有内容将保持不变。

- 键名是固定的，必须是下面的关键字之一
- 长度指定了为显示该字段而保留的长度。静态属性不支持这个参数（GROUP、PEX、SIZE、USER）。
- Extra只被一些特定字段支持，也是可选的。请看各字段详细描述来判断是否支持Extra参数。

以下是自定义格式支持的键名：

- `ATIME`: 最后访问时间（默认语法为`%b %d %Y %H:%M`）；Extra参数可以指定时间显示语法（例如：`{ATIME:8:%H:%M}`）
- `CTIME`: 创建时间（语法为`%b %d %Y %H:%M`）；Extra参数可以指定时间显示语法（例如：`{CTIME:8:%H:%M}`）
- `GROUP`: 所属组
- `MTIME`: 最后修改时间（语法为`%b %d %Y %H:%M`）；Extra参数可以指定时间显示语法（例如：`{MTIME:8:%H:%M}`）
- `NAME`: 文件名（超过 LENGTH 个字符的部分会被省略）
- `PATH`:文件绝对路径（如果长于 LENGTH，则根目录和第一个祖先之间的文件夹将被排除）
- `PEX`: 文件权限（UNIX格式）
- `SIZE`: 文件大小（目录不显示）
- `SYMLINK`: 超链接（如果存在的话`-> {FILE_PATH}`）。
- `USER`: 所属用户

如果留空，将使用默认的格式化语法。`{NAME:24} {PEX} {USER} {SIZE} {MTIME:17:%b %d %Y %H:%M}`。

---

## 主题

Termscp为你提供了一个很棒的功能：可以为应用程序中的几个组件配置颜色。
如果你想自定义termscp，有两种可用的途径：

- 从**配置菜单**
- 导入一个**配置文件**

为了从termscp创建你的私人定制，你所要做的就是从auth活动页进入配置，按`<CTRL+C>`，然后`<TAB>`两次。你现在应该已经移到了 `themes` 面板。

在这里你可以用`<UP>`和`<DOWN>`移动来选择你想改变的样式，如下图所示：

![Themes](https://github.com/veeso/termscp/blob/main/assets/images/themes.gif?raw=true)

termscp支持传统的十六进制（`#rrggbb`）和RGB`rgb(r, g, b)`语法来表示颜色，但也接受 **[css颜色](https://www.w3schools.com/cssref/css_colors.asp)**（如`crimson`）😉。还有一个特殊的关键词是`Default`，意味着使用的颜色将是基于情景的默认前景或背景颜色（文本和线条的前景色，以及容器的背景色，你猜是什么）。

如前所述，你也可以导入主题文件。你可以从themscp提供的主题中获取灵感或直接使用其中的一个，位于这个代码仓库的`themes/`目录下，运行themscp以导入它们 `termscp -t <theme_file>`。最后，如果一切正常，它应该提示你主题已经成功导入。

### 样式

你可以在下面的表格中找到每个样式字段的描述。
请注意，**样式在配置页面不起作用**，以保证它总是可以访问，以防你把一切都弄乱了。

### 我的主题无法加载

这可能是由于最近的更新破坏了主题。 每当我向主题添加新密钥时，保存的主题都不会加载。 要解决此问题，有两个真正的快速修复解决方案：

1. 重新加载主题：每当我发布更新时，我也会修补“官方”主题，因此您只需再次从存储库下载它并通过 `-t` 选项重新导入主题

    ```sh
    termscp -t <theme.toml>
    ```

2. 修复您的主题：如果您使用自定义主题，那么您可以通过 `vim` 进行编辑并添加缺少的键。 主题位于 `$CONFIG_DIR/termscp/theme.toml`，其中 `$CONFIG_DIR` 是：

    - FreeBSD/GNU-Linux: `$HOME/.config/`
    - MacOs: `$HOME/Library/Application Support`
    - Windows: `%appdata%`

    ❗ 对于您刚刚安装的版本，在 `BREAKING CHANGES` 下的 `CHANGELOG` 中报告了丢失的键。

#### 登录页

| 字段           | 描述                                     |
|----------------|------------------------------------------|
| auth_address   | IP地址输入框的颜色                       |
| auth_bookmarks | 书签面板的颜色                           |
| auth_password  | 密码输入框的颜色                         |
| auth_port      | 端口输入框的颜色                         |
| auth_protocol  | 协议选项组的颜色                         |
| auth_recents   | 历史记录面板的颜色                       |
| auth_username  | 用户名输入框的颜色                       |

#### 文件传输页

| 字段                                 | 描述                                                                      |
|--------------------------------------|---------------------------------------------------------------------------|
| transfer_local_explorer_background   | 本地资源浏览器的背景色                                                    |
| transfer_local_explorer_foreground   | 本地资源浏览器的前景色                                                    |
| transfer_local_explorer_highlighted  | 本地资源浏览器的边框和高亮色                                              |
| transfer_remote_explorer_background  | 远程资源浏览器的背景色                                                    |
| transfer_remote_explorer_foreground  | 远程资源浏览器的前景色                                                    |
| transfer_remote_explorer_highlighted | 远程资源浏览器的边框和高亮色                                              |
| transfer_log_background              | 日志面板的背景色                                                          |
| transfer_log_window                  | 日志面板的窗口颜色                                                        |
| transfer_progress_bar_partial        | 进度条完成部分颜色                                                        |
| transfer_progress_bar_total          | 进度条背景色颜色                                                          |
| transfer_status_hidden               | 状态栏 "hidden" 标签的颜色                                                |
| transfer_status_sorting              | 状态栏 "sorting" 标签的颜色；同时适用于文件排序对话框                     |
| transfer_status_sync_browsing        | 状态栏 "sync browsing" 标签的颜色                                         |

#### Misc

以下这些样式会在程序不同的位置起作用：

| 字段              | 描述                                        |
|-------------------|---------------------------------------------|
| misc_error_dialog | 报错信息的颜色                              |
| misc_info_dialog  | 信息对话框的颜色                      |
| misc_input_dialog | 输入对话框的颜色（比如拷贝文件时）          |
| misc_keys         | 键盘输入文字的颜色                          |
| misc_quit_dialog  | 退出窗口的颜色                              |
| misc_save_dialog  | 保存窗口的颜色                              |
| misc_warn_dialog  | 警告窗口的颜色                              |

---

## 文本编辑器

Termscp有很多功能，你可能已经注意到了，其中之一就是可以查看和编辑文本文件。不管文件是在本地主机还是在远程主机上，termscp都提供了在你喜欢的文本编辑器中打开文件的功能。
如果文件位于远程主机上，该文件将首先被下载到你的临时文件目录中，然后，**只有**在对该文件进行了修改的情况下，才会重新上传至远程主机上。

> ❗ 多说一句，**你只能编辑文本文件**；二进制文件是不可以的。

---

## 日志

termscp会为每个会话创建一个日志文件，该文件在

- `$HOME/.cache/termscp/termscp.log` -- Linux/BSD
- `$HOME/Library/Caches/termscp/termscp.log` -- MacOs
- `FOLDERID_LocalAppData\termscp\termscp.log` -- Windows

日志不会被轮换，但只会在每次启动 termcp 后被截断，因此如果您想报告问题并希望附加您的日志文件，请记住在使用前将日志文件保存在安全的地方 再次termscp。
默认情况下，日志记录在 *INFO* 级别报告，因此它不是很详细。

如果你想提交一个问题，如果可以的话，请在级别设置为`TRACE`的情况下重现问题，为此，启动termscp
`-D` CLI 选项。

我知道您可能对日志文件有一些疑问，所以我做了一个问答：

> 我不希望有日志记录，我可以把它关掉吗？

可以的。只要用`-q or --quiet`选项启动termscp。你可以用别名来启动termscp从而使这个选项一直生效。记住，日志是用来诊断故障的，所以在每个开源项目的背后，都应该有这样的互动反馈，保留日志文件可能是你支持项目的途径😉。我不想让你感到内疚，只是想提一句。

> 日志是安全的吗？

如果你担心安全问题，日志文件不包含任何普通的密码，所以不用担心，它暴露的信息与同级文件 `书签` 报告的信息相同。

## 通知

termscp 将针对这些类型的事件发送桌面通知：

- **传输完成**： 传输成功完成后将发送通知。
  - ❗ 仅当传输总大小至少为配置中指定的 `Notifications: minimum transfer size` 时才会显示通知。
- **传输失败**：一旦传输因错误而失败，将发送通知。
  - ❗ 仅当传输总大小至少为配置中指定的 `Notifications: minimum transfer size` 时才会显示通知。
- **更新可用**：每当有新版本的termscp 可用时，都会显示通知。
- **更新已安装**：每当安装了新版本的termscp 时，都会显示通知。
- **更新失败**：每当更新安装失败时，都会显示通知。

❗ 如果您希望保持关闭通知，您只需进入设置并将 `Enable notifications?` 设置为 `No`😉。  
❗ 如果您想更改最小传输大小以显示通知，您可以使用键 `Notifications: minimum transfer size` 更改配置中的值，并将其设置为更适合您的任何值🙂。

## 文件观察者🔭

文件观察器允许您设置与远程主机同步的路径列表。
这意味着每当在同步路径上检测到本地文件系统的更改时，该更改将在 5 秒内自动报告给配置的远程主机路径。

您可以根据需要设置尽可能多的同步路径：

1.将光标放在本地资源管理器上要保持同步的目录/文件上
2. 转到远程主机上要向其报告更改的目录
3. 按`<T>`
4. 对无线电弹出窗口回答 `<YES>`

要取消观看，只需在本地同步路径（或其任何子文件夹）上按 `<T>`
或者，您可以按 `<CTRL + T>` 并按 `<ENTER>` 进入要取消观看的同步路径。

这些更改将报告给远程主机：

- 新文件，文件更改
- 文件移动/重命名
- 文件删除/取消链接

> ❗ 观察者只在一个方向工作（本地>远程）。不可能自动同步远程到本地的更改。
