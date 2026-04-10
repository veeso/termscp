use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr as _;
use std::time::UNIX_EPOCH;

use libssh_rs::{AuthMethods, AuthStatus, OpenFlags, SshKey, SshOption};
use remotefs::fs::stream::{ReadAndSeek, WriteAndSeek};
use remotefs::fs::{FileType, Metadata, ReadStream, UnixPex, WriteStream};
use remotefs::{File, RemoteError, RemoteErrorType, RemoteResult};

use super::SshSession;
use crate::SshOpts;
use crate::ssh::backend::Sftp;
use crate::ssh::config::Config;

/// An implementation of [`SshSession`] using libssh as the backend.
///
/// See <https://docs.rs/libssh-rs/0.3.6/libssh_rs/struct.Session.html>
pub struct LibSshSession {
    session: libssh_rs::Session,
}

/// A wrapper around [`libssh_rs::Sftp`] to provide a SFTP client for [`LibSshSession`]
///
/// See <https://docs.rs/libssh-rs/0.3.6/libssh_rs/struct.Sftp.html>
pub struct LibSshSftp {
    inner: libssh_rs::Sftp,
}

/// A wrapper around [`libssh_rs::Channel`] to provide a SCP recv channel for [`LibSshSession`]
struct ScpRecvChannel {
    channel: libssh_rs::Channel,
    /// We must keep track of the total file size
    /// otherwise read will hang
    filesize: usize,
    read: usize,
}

impl Read for ScpRecvChannel {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.read >= self.filesize {
            return Ok(0);
        }

        // read up to
        let max_read = (self.filesize - self.read).min(buf.len());
        let res = self.channel.stdout().read(&mut buf[..max_read])?;

        self.read += res;
        Ok(res)
    }
}

impl Drop for ScpRecvChannel {
    fn drop(&mut self) {
        debug!("Dropping SCP recv channel");
        if let Err(err) = self.channel.send_eof() {
            debug!("Error sending EOF: {err}");
        }
        if let Err(err) = self.channel.close() {
            debug!("Error closing channel: {err}");
        }
    }
}

/// A wrapper around [`libssh_rs::Channel`] to provide a SCP send channel for [`LibSshSession`]
struct ScpSendChannel {
    channel: libssh_rs::Channel,
}

impl Write for ScpSendChannel {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.channel.stdin().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.channel.stdin().flush()
    }
}

impl Drop for ScpSendChannel {
    fn drop(&mut self) {
        debug!("Dropping SCP send channel");
        if let Err(err) = self.channel.send_eof() {
            debug!("Error sending EOF: {err}");
        }
        if let Err(err) = self.channel.close() {
            debug!("Error closing channel: {err}");
        }
    }
}

impl SshSession for LibSshSession {
    type Sftp = LibSshSftp;

    fn connect(opts: &SshOpts) -> remotefs::RemoteResult<Self> {
        // Resolve host
        debug!("Connecting to '{}'", opts.host);

        // Create session
        let mut session = match libssh_rs::Session::new() {
            Ok(s) => s,
            Err(err) => {
                error!("Could not create session: {err}");
                return Err(RemoteError::new_ex(RemoteErrorType::ConnectionError, err));
            }
        };

        // set hostname
        session
            .set_option(SshOption::Hostname(opts.host.clone()))
            .map_err(|e| RemoteError::new_ex(RemoteErrorType::ConnectionError, e))?;
        if let Some(port) = opts.port {
            debug!("Using port: {port}");
            session
                .set_option(SshOption::Port(port))
                .map_err(|e| RemoteError::new_ex(RemoteErrorType::ConnectionError, e))?;
        }

        let config_file_str = opts.config_file.as_ref().map(|p| p.display().to_string());
        debug!("Using config file: {:?}", config_file_str);
        session
            .options_parse_config(config_file_str.as_deref())
            .map_err(|e| RemoteError::new_ex(RemoteErrorType::ConnectionError, e))?;

        // set methods
        for opt in opts.methods.iter().filter_map(|method| method.ssh_opts()) {
            debug!("Setting SSH option: {opt:?}");
            session
                .set_option(opt)
                .map_err(|e| RemoteError::new_ex(RemoteErrorType::ConnectionError, e))?;
        }

        // Open connection and initialize handshake
        if let Err(err) = session.connect() {
            error!("SSH handshake failed: {err}");
            return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
        }

        // try to authenticate userauth_none
        authenticate(&mut session, opts)?;

        Ok(Self { session })
    }

    fn authenticated(&self) -> RemoteResult<bool> {
        Ok(self.session.is_connected())
    }

    fn banner(&self) -> RemoteResult<Option<String>> {
        self.session.get_server_banner().map(Some).map_err(|e| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Failed to get banner: {e}"),
            )
        })
    }

    fn disconnect(&self) -> RemoteResult<()> {
        self.session.disconnect();

        Ok(())
    }

    fn cmd<S>(&mut self, cmd: S) -> RemoteResult<(u32, String)>
    where
        S: AsRef<str>,
    {
        let output = perform_shell_cmd(&mut self.session, format!("{}; echo $?", cmd.as_ref()))?;
        if let Some(index) = output.trim().rfind('\n') {
            trace!("Read from stdout: '{output}'");
            let actual_output = (output[0..index + 1]).to_string();
            trace!("Actual output '{actual_output}'");
            trace!("Parsing return code '{}'", output[index..].trim());
            let rc = match u32::from_str(output[index..].trim()).ok() {
                Some(val) => val,
                None => {
                    return Err(RemoteError::new_ex(
                        RemoteErrorType::ProtocolError,
                        "Failed to get command exit code",
                    ));
                }
            };
            debug!(r#"Command output: "{actual_output}"; exit code: {rc}"#);
            Ok((rc, actual_output))
        } else {
            match u32::from_str(output.trim()).ok() {
                Some(val) => Ok((val, String::new())),
                None => Err(RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    "Failed to get command exit code",
                )),
            }
        }
    }

    fn scp_recv(&self, path: &Path) -> RemoteResult<Box<dyn Read + Send>> {
        self.session.set_blocking(true);

        // open channel
        debug!("Opening channel for scp recv");
        let channel = self.session.new_channel().map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not open channel: {err}"),
            )
        })?;
        debug!("Opening channel session");
        channel.open_session().map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not open session: {err}"),
            )
        })?;
        // exec `scp -f %s`
        let cmd = format!("scp -f {}", path.display());
        channel.request_exec(cmd.as_ref()).map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not request command execution: {err}"),
            )
        })?;
        debug!("ACK with 0");
        // write \0
        channel.stdin().write_all(b"\0").map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not write to channel: {err}"),
            )
        })?;

        // read header
        debug!("Reading SCP header");
        let mut header = [0u8; 1024];
        let bytes = channel.stdout().read(&mut header).map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not read from channel: {err}"),
            )
        })?;
        // read filesize from header
        let filesize = parse_scp_header_filesize(&header[..bytes])?;
        debug!("File size: {filesize}");
        // send OK
        debug!("Sending OK");
        channel.stdin().write_all(b"\0").map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not write to channel: {err}"),
            )
        })?;

        debug!("Creating SCP recv channel");
        let reader = ScpRecvChannel {
            channel,
            filesize,
            read: 0,
        };

        Ok(Box::new(reader) as Box<dyn Read + Send>)
    }

    fn scp_send(
        &self,
        remote_path: &Path,
        mode: i32,
        size: u64,
        _times: Option<(u64, u64)>,
    ) -> RemoteResult<Box<dyn Write + Send>> {
        self.session.set_blocking(true);

        // open channel
        debug!("Opening channel for scp send");
        let channel = self.session.new_channel().map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not open channel: {err}"),
            )
        })?;
        debug!("Opening channel session");
        channel.open_session().map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not open session: {err}"),
            )
        })?;
        // exec `scp -t %s`
        let cmd = format!("scp -t {}", remote_path.display());
        channel.request_exec(cmd.as_ref()).map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not request command execution: {err}"),
            )
        })?;

        // wait for ACK
        wait_for_ack(&channel)?;

        let Some(filename) = remote_path.file_name().map(|f| f.to_string_lossy()) else {
            return Err(RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not get file name: {remote_path:?}"),
            ));
        };

        // send file header
        let header = format!("C{mode:04o} {size} {filename}\n", mode = mode & 0o7777,);
        debug!("Sending SCP header: {header}");
        channel
            .stdin()
            .write_all(header.as_bytes())
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!("Could not write to channel: {err}"),
                )
            })?;

        // wait for ACK
        wait_for_ack(&channel)?;

        // return channel
        let writer = ScpSendChannel { channel };
        Ok(Box::new(writer) as Box<dyn Write + Send>)
    }

    fn sftp(&self) -> RemoteResult<Self::Sftp> {
        self.session
            .sftp()
            .map(|sftp| LibSshSftp { inner: sftp })
            .map_err(|e| RemoteError::new_ex(RemoteErrorType::ProtocolError, e))
    }
}

struct SftpFileReader(libssh_rs::SftpFile);

struct SftpFileWriter(libssh_rs::SftpFile);

impl Write for SftpFileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl Seek for SftpFileWriter {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.0.seek(pos)
    }
}

impl WriteAndSeek for SftpFileWriter {}

impl Read for SftpFileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Seek for SftpFileReader {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.0.seek(pos)
    }
}

impl ReadAndSeek for SftpFileReader {}

impl Sftp for LibSshSftp {
    fn mkdir(&self, path: &Path, mode: i32) -> RemoteResult<()> {
        self.inner
            .create_dir(conv_path_to_str(path), mode as u32)
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::FileCreateDenied,
                    format!(
                        "Could not create directory '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }

    fn open_read(&self, path: &Path) -> RemoteResult<ReadStream> {
        self.inner
            .open(conv_path_to_str(path), OpenFlags::READ_ONLY, 0)
            .map(|file| ReadStream::from(Box::new(SftpFileReader(file)) as Box<dyn ReadAndSeek>))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!(
                        "Could not open file at '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }

    fn open_write(
        &self,
        path: &Path,
        flags: super::WriteMode,
        mode: i32,
    ) -> RemoteResult<WriteStream> {
        let flags = match flags {
            super::WriteMode::Append => {
                OpenFlags::WRITE_ONLY | OpenFlags::APPEND | OpenFlags::CREATE
            }
            super::WriteMode::Truncate => {
                OpenFlags::WRITE_ONLY | OpenFlags::CREATE | OpenFlags::TRUNCATE
            }
        };

        //panic!("Figa");

        self.inner
            .open(conv_path_to_str(path), flags, mode as u32)
            .map(|file| WriteStream::from(Box::new(SftpFileWriter(file)) as Box<dyn WriteAndSeek>))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!(
                        "Could not open file at '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }

    fn readdir<T>(&self, dirname: T) -> RemoteResult<Vec<remotefs::File>>
    where
        T: AsRef<Path>,
    {
        self.inner
            .read_dir(conv_path_to_str(dirname.as_ref()))
            .map(|files| {
                files
                    .into_iter()
                    .filter(|metadata| {
                        metadata.name() != Some(".") && metadata.name() != Some("..")
                    })
                    .map(|metadata| {
                        self.make_fsentry(MakePath::Directory(dirname.as_ref()), metadata)
                    })
                    .collect()
            })
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!("Could not read directory: {err}",),
                )
            })
    }

    fn realpath(&self, path: &Path) -> RemoteResult<PathBuf> {
        self.inner
            .read_link(conv_path_to_str(path))
            .map(PathBuf::from)
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!(
                        "Could not resolve real path for '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }

    fn rename(&self, src: &Path, dest: &Path) -> RemoteResult<()> {
        self.inner
            .rename(conv_path_to_str(src), conv_path_to_str(dest))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!("Could not rename file '{src}': {err}", src = src.display()),
                )
            })
    }

    fn rmdir(&self, path: &Path) -> RemoteResult<()> {
        self.inner
            .remove_dir(conv_path_to_str(path))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::CouldNotRemoveFile,
                    format!(
                        "Could not remove directory '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }

    fn setstat(&self, path: &Path, metadata: Metadata) -> RemoteResult<()> {
        self.inner
            .set_metadata(conv_path_to_str(path), &Self::set_attributes(metadata))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!(
                        "Could not set file attributes for '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }

    fn stat(&self, filename: &Path) -> RemoteResult<File> {
        self.inner
            .metadata(conv_path_to_str(filename))
            .map(|metadata| self.make_fsentry(MakePath::File(filename), metadata))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!(
                        "Could not get file attributes for '{filename}': {err}",
                        filename = filename.display()
                    ),
                )
            })
    }

    fn symlink(&self, path: &Path, target: &Path) -> RemoteResult<()> {
        self.inner
            .symlink(conv_path_to_str(path), conv_path_to_str(target))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::FileCreateDenied,
                    format!(
                        "Could not create symlink '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }

    fn unlink(&self, path: &Path) -> RemoteResult<()> {
        self.inner
            .remove_file(conv_path_to_str(path))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::CouldNotRemoveFile,
                    format!(
                        "Could not remove file '{path}': {err}",
                        path = path.display()
                    ),
                )
            })
    }
}

fn conv_path_to_str(path: &Path) -> &str {
    path.to_str().unwrap_or_default()
}

enum MakePath<'a> {
    Directory(&'a Path),
    File(&'a Path),
}

impl LibSshSftp {
    fn set_attributes(metadata: Metadata) -> libssh_rs::SetAttributes {
        let atime = metadata.accessed.unwrap_or(UNIX_EPOCH);
        let mtime = metadata.modified.unwrap_or(UNIX_EPOCH);

        let uid_gid = match (metadata.uid, metadata.gid) {
            (Some(uid), Some(gid)) => Some((uid, gid)),
            _ => None,
        };

        libssh_rs::SetAttributes {
            size: Some(metadata.size),
            uid_gid,
            permissions: metadata.mode.map(|m| m.into()),
            atime_mtime: Some((atime, mtime)),
        }
    }

    fn make_fsentry(&self, path: MakePath<'_>, metadata: libssh_rs::Metadata) -> File {
        let name = match metadata.name() {
            None => "/".to_string(),
            Some(name) => name.to_string(),
        };
        debug!("Found file {name}");

        let path = match path {
            MakePath::Directory(dir) => dir.join(&name),
            MakePath::File(file) => file.to_path_buf(),
        };
        debug!("Computed path for {name}: {path}", path = path.display());

        // parse metadata
        let uid = metadata.uid();
        let gid = metadata.gid();
        let mode = metadata.permissions().map(UnixPex::from);
        let size = metadata.len().unwrap_or(0);
        let accessed = metadata.accessed();
        let modified = metadata.modified();
        let symlink = match metadata.file_type() {
            Some(libssh_rs::FileType::Symlink) => match self.realpath(&path) {
                Ok(p) => Some(p),
                Err(err) => {
                    error!(
                        "Failed to read link of {} (even it's supposed to be a symlink): {err}",
                        path.display(),
                    );
                    None
                }
            },
            _ => None,
        };
        let file_type = if symlink.is_some() {
            FileType::Symlink
        } else if matches!(metadata.file_type(), Some(libssh_rs::FileType::Directory)) {
            FileType::Directory
        } else {
            FileType::File
        };
        let entry_metadata = Metadata {
            accessed,
            created: None,
            file_type,
            gid,
            mode,
            modified,
            size,
            symlink,
            uid,
        };
        trace!("Metadata for {}: {:?}", path.display(), entry_metadata);
        File {
            path: path.to_path_buf(),
            metadata: entry_metadata,
        }
    }
}

fn authenticate(session: &mut libssh_rs::Session, opts: &SshOpts) -> RemoteResult<()> {
    // parse configuration
    let ssh_config = Config::try_from(opts)?;
    let username = ssh_config.username.clone();

    debug!("Authenticating to {}", opts.host);
    session
        .set_option(SshOption::User(Some(username)))
        .map_err(|e| {
            RemoteError::new_ex(
                RemoteErrorType::AuthenticationFailed,
                format!("Failed to set username: {e}"),
            )
        })?;

    debug!("Trying with userauth_none");
    match session.userauth_none(opts.username.as_deref()) {
        Ok(AuthStatus::Success) => {
            debug!("Authenticated with userauth_none");
            return Ok(());
        }
        Ok(status) => {
            debug!("userauth_none returned status: {status:?}");
        }
        Err(err) => {
            debug!("userauth_none failed: {err}");
        }
    }

    let auth_methods = session
        .userauth_list(opts.username.as_deref())
        .map_err(|e| RemoteError::new_ex(RemoteErrorType::AuthenticationFailed, e))?;
    debug!("Available authentication methods: {auth_methods:?}");

    if auth_methods.contains(AuthMethods::PUBLIC_KEY) {
        debug!("Trying public key authentication");
        // try with known key to config
        match session.userauth_public_key_auto(None, None) {
            Ok(AuthStatus::Success) => {
                debug!("Authenticated with public key");
                return Ok(());
            }
            Ok(status) => {
                debug!("userauth_public_key_auto returned status: {status:?}");
            }
            Err(err) => {
                debug!("userauth_public_key_auto failed: {err}");
            }
        }

        // try with storage
        match key_storage_auth(session, opts, &ssh_config) {
            Ok(()) => {
                debug!("Authenticated with public key from storage");
                return Ok(());
            }
            Err(err) => {
                debug!("Key storage authentication failed: {err}");
            }
        }
    }

    if auth_methods.contains(AuthMethods::PASSWORD) {
        debug!("Trying password authentication");

        // NOTE: you cannot pass password None. It causes SEGFAULT
        match session.userauth_password(None, Some(opts.password.as_deref().unwrap_or_default())) {
            Ok(AuthStatus::Success) => {
                debug!("Authenticated with password");
                return Ok(());
            }
            Ok(status) => {
                debug!("userauth_password returned status: {status:?}");
            }
            Err(err) => {
                debug!("userauth_password failed: {err}");
            }
        }
    }

    Err(RemoteError::new_ex(
        RemoteErrorType::AuthenticationFailed,
        "all authentication methods failed",
    ))
}

fn key_storage_auth(
    session: &mut libssh_rs::Session,
    opts: &SshOpts,
    ssh_config: &Config,
) -> RemoteResult<()> {
    let Some(key_storage) = &opts.key_storage else {
        return Err(RemoteError::new_ex(
            RemoteErrorType::AuthenticationFailed,
            "no key storage available",
        ));
    };

    let Some(priv_key_path) = key_storage
        .resolve(&ssh_config.host, &ssh_config.username)
        .or(key_storage.resolve(
            ssh_config.resolved_host.as_str(),
            ssh_config.username.as_str(),
        ))
    else {
        return Err(RemoteError::new_ex(
            RemoteErrorType::AuthenticationFailed,
            "no key found in storage",
        ));
    };

    let Ok(privkey) =
        SshKey::from_privkey_file(conv_path_to_str(&priv_key_path), opts.password.as_deref())
    else {
        return Err(RemoteError::new_ex(
            RemoteErrorType::AuthenticationFailed,
            format!(
                "could not load private key from file: {}",
                priv_key_path.display()
            ),
        ));
    };

    match session
        .userauth_publickey(opts.username.as_deref(), &privkey)
        .map_err(|e| RemoteError::new_ex(RemoteErrorType::AuthenticationFailed, e))
    {
        Ok(AuthStatus::Success) => Ok(()),
        Ok(status) => Err(RemoteError::new_ex(
            RemoteErrorType::AuthenticationFailed,
            format!("authentication failed: {status:?}"),
        )),
        Err(err) => Err(err),
    }
}

fn perform_shell_cmd<S: AsRef<str>>(
    session: &mut libssh_rs::Session,
    cmd: S,
) -> RemoteResult<String> {
    // Create channel
    trace!("Running command: {}", cmd.as_ref());
    let channel = match session.new_channel() {
        Ok(ch) => ch,
        Err(err) => {
            return Err(RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not open channel: {err}"),
            ));
        }
    };

    debug!("Opening channel session");
    channel.open_session().map_err(|err| {
        RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Could not open session: {err}"),
        )
    })?;

    // escape single quotes in command
    let cmd = cmd.as_ref().replace('\'', r#"'\''"#); // close, escape, and reopen

    debug!("Requesting command execution: {cmd}",);
    channel
        .request_exec(&format!("sh -c '{cmd}'"))
        .map_err(|err| {
            RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not execute command \"{cmd}\": {err}"),
            )
        })?;
    // send EOF
    debug!("Sending EOF");
    channel.send_eof().map_err(|err| {
        RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Could not send EOF: {err}"),
        )
    })?;

    // Read output
    let mut output: String = String::new();
    match channel.stdout().read_to_string(&mut output) {
        Ok(_) => {
            // Wait close
            let res = channel.get_exit_status();
            trace!("Command output (res: {res:?}): {output}");
            Ok(output)
        }
        Err(err) => Err(RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Could not read output: {err}"),
        )),
    }
}

/// Read filesize from scp header
fn parse_scp_header_filesize(header: &[u8]) -> RemoteResult<usize> {
    // Header format: C<mode> <size> <filename>\n
    let header_str = std::str::from_utf8(header).map_err(|e| {
        RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Could not parse header: {e}"),
        )
    })?;
    let parts: Vec<&str> = header_str.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            "Invalid SCP header: not enough parts",
        ));
    }
    if !parts[0].starts_with('C') {
        return Err(RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            "Invalid SCP header: missing 'C'",
        ));
    }
    let size = parts[1].parse::<usize>().map_err(|e| {
        RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Invalid file size: {e}"),
        )
    })?;

    Ok(size)
}

/// Wait for channel ACK
fn wait_for_ack(channel: &libssh_rs::Channel) -> RemoteResult<()> {
    debug!("Waiting for channel acknowledgment");
    // read ACK
    let mut ack = [0u8; 1024];
    let n = channel.stdout().read(&mut ack).map_err(|err| {
        RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Could not read from channel: {err}"),
        )
    })?;
    if n == 1 && ack[0] != 0 {
        Err(RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Unexpected ACK: {ack:?} (read {n} bytes)"),
        ))
    } else {
        Ok(())
    }
}
