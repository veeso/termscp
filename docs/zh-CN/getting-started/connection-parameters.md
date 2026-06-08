# 连接参数

每种协议都有其各自的认证表单字段以及各自的命令行地址语法。本页将逐个协议加以说明。

## SFTP / SCP

认证表单字段：

- 主机（地址）
- 端口（默认 `22`）
- 用户名
- 密码或 SSH 密钥

你可以使用用户名和密码进行认证，也可以使用 SSH 密钥进行认证。有关如何管理密钥，请参阅 [SSH 密钥存储](../configuration/ssh-keys.md)。

地址语法：

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

## FTP / FTPS

认证表单字段：

- 主机（地址）
- 端口（默认 `21`）
- 用户名
- 密码
- 安全（FTPS）：启用 TLS 以使用 FTPS 而非明文 FTP

地址语法：

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

## Kube

认证表单字段：

- 命名空间
- 集群 URL（Kubernetes API URL）
- 用户名
- 客户端证书路径
- 客户端密钥路径

地址语法：

```txt
kube://[namespace][@<cluster_url>][$</path>]
```

## S3

termscp 同时支持 AWS S3 和其他兼容 S3 的端点。

认证表单字段：

- 存储桶名称
- 区域（用于 AWS S3）或端点（用于其他兼容 S3 的服务器）
- 配置文件
- 访问密钥
- 私有访问密钥
- 安全令牌
- 会话令牌
- 新路径风格

所需字段和可选字段会因端点不同而有所差异：

- AWS S3：
  - 存储桶名称（必填）
  - 区域（必填）
  - 配置文件（可选；默认为 `default`）
  - 访问密钥（除非存储桶为公开，否则必填）
  - 私有访问密钥（除非存储桶为公开，否则必填）
  - 安全令牌（如有需要）
  - 会话令牌（如有需要）
  - 新路径风格：否
- 其他 S3 端点：
  - 存储桶名称（必填）
  - 端点（必填）
  - 访问密钥（除非存储桶为公开，否则必填）
  - 私有访问密钥（除非存储桶为公开，否则必填）
  - 新路径风格：是

地址语法：

```txt
s3://<bucket>@<region>[:profile][:/wrkdir]
```

例如：

```txt
s3://buckethead@eu-central-1:default:/assets
```

### S3 凭证

要连接到 AWS S3 存储桶，你必须提供凭据。有三种方式可以做到这一点。

1. 认证表单：提供访问密钥（通常必填）、私有访问密钥（通常必填）、安全令牌和会话令牌。如果你将该 S3 连接保存为书签，访问密钥和私有访问密钥将以加密的 AES-256/BASE64 字符串形式保存在你的书签文件中。安全令牌和会话令牌不会被保存，因为它们本身就是临时凭据。
2. 凭据文件：使用 `aws configure` 配置 AWS CLI。随后你的凭据将被存储在 `~/.aws/credentials`。如果你使用的是 `default` 以外的配置文件，请在认证表单的配置文件字段中提供它。
3. 环境变量：以环境变量的形式提供你的凭据。这些变量始终会覆盖凭据文件中的凭据。以下变量通常是必需的：

   - `AWS_ACCESS_KEY_ID`：AWS 访问密钥 ID（通常以 `AKIA...` 开头）
   - `AWS_SECRET_ACCESS_KEY`：私有访问密钥

   如果你配置了更强的安全机制，可能还需要：

   - `AWS_SECURITY_TOKEN`：安全令牌
   - `AWS_SESSION_TOKEN`：会话令牌

你的凭据是安全的：termscp 不会直接操作这些值。它们由 `s3` crate 直接使用。

## SMB

认证表单字段：

- 服务器（地址）
- 共享
- 用户名
- 密码
- 端口（仅其他系统；默认 `445`）
- 工作组（仅其他系统）

在 Windows 上，端口和工作组字段不会被使用。

Windows 地址语法：

```txt
\\[username@]<server-name>\<share>[\path\...]
```

其他系统地址语法：

```txt
smb://[username@]<server-name>[:port]/<share>[/path/.../]
```

## WebDAV

认证表单字段：

- URI（WebDAV 的基础端点）
- 用户名
- 密码

地址语法：

```txt
http(s)://<username>:<password>@<url></path>
```
