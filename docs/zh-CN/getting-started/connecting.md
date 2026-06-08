# 连接到服务器

termscp 可以根据你传入的参数以三种不同的方式启动。

- 不带参数：termscp 打开认证表单，你在其中提供连接到远程主机所需的参数。
- 带地址参数：termscp 跳过认证表单，直接连接到远程主机。
- 通过 `-b <bookmark-name>` 传入书签名称：termscp 将参数解析为已保存的书签并进行连接。重复使用 `-b` 可打开多个书签。

当你提供地址参数或书签名称时，还可以为本地主机提供一个起始工作目录。

## The authentication form

当 termscp 在不带地址的情况下启动时，会显示认证表单。填写协议、地址、端口、用户名和密码，然后进行连接。连接成功后，termscp 将打开双面板浏览器。

## Address argument syntax

通用地址参数采用以下语法：

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

这种语法很方便，你很可能会用它来代替交互式表单。下面是一些示例。

使用默认协议（在你的配置中定义）连接到 `192.168.1.31`。如果未提供端口，则使用所选协议的默认端口。用户名为当前用户的名称。

```sh
termscp 192.168.1.31
```

使用默认协议连接到 `192.168.1.31`，用户名为 `root`：

```sh
termscp root@192.168.1.31
```

使用 SCP 连接到 `192.168.1.31`，端口为 `4022`，用户名为 `omar`：

```sh
termscp scp://omar@192.168.1.31:4022
```

使用 SCP 连接到 `192.168.1.31`，端口为 `4022`，用户名为 `omar`，并以目录 `/tmp` 作为起始目录：

```sh
termscp scp://omar@192.168.1.31:4022:/tmp
```

有关各协议专属的地址语法（S3、Kube、WebDAV 和 SMB），请参阅[连接参数](connection-parameters.md)。

## How the password is provided

当你以参数形式提供地址时，地址本身没有用于填写密码的字段。你可以通过三种方式提供密码：

- 系统会提示你输入密码。这是默认方式：如果你不使用下面的任何方法，termscp 会像 `scp`、`ssh` 等经典工具一样提示你输入密码。
- `-P, --password` 选项：直接在命令行上传入密码。不推荐这种方法，因为它不安全：密码可能会保留在你的 shell 历史记录中。
- 通过 `sshpass`：借助 `sshpass` 提供密码，例如：

  ```sh
  sshpass -f ~/.ssh/topsecret.key termscp cvisintin@192.168.1.31
  ```
