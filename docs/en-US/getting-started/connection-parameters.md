# Connection parameters

Each protocol has its own set of authentication-form fields and its own
command-line address syntax. This page describes them protocol by protocol.

## SFTP / SCP

Authentication-form fields:

- Host (address)
- Port (default `22`)
- Username
- Password or SSH key

You can authenticate either with a username and password or with an SSH key.
See [SSH key storage](../configuration/ssh-keys.md) for how to manage keys.

Address syntax:

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

## FTP / FTPS

Authentication-form fields:

- Host (address)
- Port (default `21`)
- Username
- Password
- Secure (FTPS): enable TLS to use FTPS instead of plain FTP

Address syntax:

```txt
[protocol://][username@]<address>[:port][:wrkdir]
```

## Kube

Authentication-form fields:

- Namespace
- Cluster URL (Kubernetes API URL)
- Username
- Client certificate path
- Client key path

Address syntax:

```txt
kube://[namespace][@<cluster_url>][$</path>]
```

## S3

termscp supports both AWS S3 and other S3-compatible endpoints.

Authentication-form fields:

- Bucket name
- Region (for AWS S3) or endpoint (for other S3-compatible servers)
- Profile
- Access key
- Secret access key
- Security token
- Session token
- New path style

The required and optional fields differ depending on the endpoint:

- AWS S3:
  - bucket name (required)
  - region (required)
  - profile (optional; defaults to `default`)
  - access key (required unless the bucket is public)
  - secret access key (required unless the bucket is public)
  - security token (if required)
  - session token (if required)
  - new path style: NO
- Other S3 endpoints:
  - bucket name (required)
  - endpoint (required)
  - access key (required unless the bucket is public)
  - secret access key (required unless the bucket is public)
  - new path style: YES

Address syntax:

```txt
s3://<bucket>@<region>[:profile][:/wrkdir]
```

For example:

```txt
s3://buckethead@eu-central-1:default:/assets
```

### S3 credentials

To connect to an AWS S3 bucket you must provide credentials. There are three
ways to do this.

1. Authentication form: provide the access key (usually mandatory), the secret
   access key (usually mandatory), the security token, and the session token.
   If you save the S3 connection as a bookmark, the access key and secret access
   key are saved as an encrypted AES-256/BASE64 string in your bookmarks file.
   The security token and session token are not saved, since they are meant to
   be temporary credentials.
2. Credentials file: configure the AWS CLI with `aws configure`. Your
   credentials are then stored at `~/.aws/credentials`. If you use a profile
   other than `default`, provide it in the profile field of the authentication
   form.
3. Environment variables: provide your credentials as environment variables.
   These always override the credentials in the credentials file. The following
   are usually mandatory:

   - `AWS_ACCESS_KEY_ID`: AWS access key ID (usually starts with `AKIA...`)
   - `AWS_SECRET_ACCESS_KEY`: the secret access key

   If you have configured stronger security, you may also need:

   - `AWS_SECURITY_TOKEN`: security token
   - `AWS_SESSION_TOKEN`: session token

Your credentials are safe: termscp does not manipulate these values directly.
They are consumed directly by the `s3` crate.

## SMB

Authentication-form fields:

- Server (address)
- Share
- Username
- Password
- Port (other systems only; default `445`)
- Workgroup (other systems only)

On Windows the port and workgroup fields are not used.

Windows address syntax:

```txt
\\[username@]<server-name>\<share>[\path\...]
```

Other systems address syntax:

```txt
smb://[username@]<server-name>[:port]/<share>[/path/.../]
```

## WebDAV

Authentication-form fields:

- URI (the base WebDAV endpoint)
- Username
- Password

Address syntax:

```txt
http(s)://<username>:<password>@<url></path>
```
