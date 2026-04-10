use std::io::{Read, Seek, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs as _};
use std::path::{Path, PathBuf};
use std::str::FromStr as _;
use std::time::{Duration, SystemTime};

use remotefs::fs::stream::{ReadAndSeek, WriteAndSeek};
use remotefs::fs::{FileType, Metadata, ReadStream, UnixPex, WriteStream};
use remotefs::{File, RemoteError, RemoteErrorType, RemoteResult};
use ssh2::{FileStat, OpenType, RenameFlags};

use super::SshSession;
use crate::ssh::backend::Sftp;
use crate::ssh::config::Config;
use crate::{SshAgentIdentity, SshOpts};

/// An implementation of [`SshSession`] using libssh2 as the backend.
pub struct LibSsh2Session {
    session: ssh2::Session,
}

/// A wrapper around [`ssh2::Sftp`] to provide a SFTP client for [`LibSsh2Session`]
pub struct LibSsh2Sftp {
    inner: ssh2::Sftp,
}

/// Authentication method
#[derive(Debug, Clone, PartialEq, Eq)]
enum Authentication {
    RsaKey(PathBuf),
    Password(String),
}

impl SshSession for LibSsh2Session {
    type Sftp = LibSsh2Sftp;

    fn connect(opts: &SshOpts) -> RemoteResult<Self> {
        // parse configuration
        let ssh_config = Config::try_from(opts)?;
        // Resolve host
        debug!("Connecting to '{}'", ssh_config.address);
        // setup tcp stream
        let socket_addresses: Vec<SocketAddr> = match ssh_config.address.to_socket_addrs() {
            Ok(s) => s.collect(),
            Err(err) => {
                return Err(RemoteError::new_ex(
                    RemoteErrorType::BadAddress,
                    err.to_string(),
                ));
            }
        };
        let mut stream = None;
        for _ in 0..ssh_config.connection_attempts {
            for socket_addr in socket_addresses.iter() {
                trace!(
                    "Trying to connect to socket address '{}' (timeout: {}s)",
                    socket_addr,
                    ssh_config.connection_timeout.as_secs()
                );
                if let Ok(tcp_stream) = tcp_connect(socket_addr, ssh_config.connection_timeout) {
                    debug!("Connection established with address {socket_addr}");
                    stream = Some(tcp_stream);
                    break;
                }
            }
            // break from attempts cycle if some
            if stream.is_some() {
                break;
            }
        }
        // If stream is None, return connection timeout
        let stream = match stream {
            Some(s) => s,
            None => {
                error!("No suitable socket address found; connection timeout");
                return Err(RemoteError::new_ex(
                    RemoteErrorType::ConnectionError,
                    "connection timeout",
                ));
            }
        };
        // Create session
        let mut session = match ssh2::Session::new() {
            Ok(s) => s,
            Err(err) => {
                error!("Could not create session: {err}");
                return Err(RemoteError::new_ex(RemoteErrorType::ConnectionError, err));
            }
        };
        // Set TCP stream
        session.set_tcp_stream(stream);
        // configure algos
        set_algo_prefs(&mut session, opts, &ssh_config)?;
        // Open connection and initialize handshake
        if let Err(err) = session.handshake() {
            error!("SSH handshake failed: {err}");
            return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
        }

        // if use_ssh_agent is enabled, try to authenticate with ssh agent
        if let Some(ssh_agent_config) = &opts.ssh_agent_identity {
            match session_auth_with_agent(&mut session, &ssh_config.username, ssh_agent_config) {
                Ok(_) => {
                    info!("Authenticated with ssh agent");
                    return Ok(Self { session });
                }
                Err(err) => {
                    error!("Could not authenticate with ssh agent: {err}");
                }
            }
        }

        // Authenticate with password or key
        if !session.authenticated() {
            let mut methods = vec![];
            // first try with ssh agent
            if let Some(rsa_key) = opts.key_storage.as_ref().and_then(|x| {
                x.resolve(ssh_config.host.as_str(), ssh_config.username.as_str())
                    .or(x.resolve(
                        ssh_config.resolved_host.as_str(),
                        ssh_config.username.as_str(),
                    ))
            }) {
                methods.push(Authentication::RsaKey(rsa_key.clone()));
            }
            // then try with password
            if let Some(password) = opts.password.as_ref() {
                methods.push(Authentication::Password(password.clone()));
            }

            // try with methods
            let mut last_err = None;
            for auth_method in methods {
                match session_auth(&mut session, opts, &ssh_config, auth_method) {
                    Ok(_) => {
                        info!("Authenticated successfully");
                        return Ok(Self { session });
                    }
                    Err(err) => {
                        error!("Authentication failed: {err}",);
                        last_err = Some(err);
                    }
                }
            }

            return Err(match last_err {
                Some(err) => err,
                None => RemoteError::new_ex(
                    RemoteErrorType::AuthenticationFailed,
                    "no authentication method provided",
                ),
            });
        }

        Ok(Self { session })
    }

    fn disconnect(&self) -> RemoteResult<()> {
        self.session
            .disconnect(None, "Mandi!", None)
            .map_err(|err| RemoteError::new_ex(RemoteErrorType::ConnectionError, err))
    }

    fn authenticated(&self) -> RemoteResult<bool> {
        Ok(self.session.authenticated())
    }

    fn banner(&self) -> RemoteResult<Option<String>> {
        Ok(self.session.banner().map(String::from))
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

        self.session
            .scp_recv(path)
            .map(|(reader, _stat)| Box::new(reader) as Box<dyn Read + Send>)
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!("Could not receive file over SCP: {err}"),
                )
            })
    }

    fn scp_send(
        &self,
        remote_path: &Path,
        mode: i32,
        size: u64,
        times: Option<(u64, u64)>,
    ) -> RemoteResult<Box<dyn Write + Send>> {
        self.session.set_blocking(true);

        self.session
            .scp_send(remote_path, mode, size, times)
            .map(|writer| Box::new(writer) as Box<dyn Write + Send>)
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!("Could not send file over SCP: {err}"),
                )
            })
    }

    fn sftp(&self) -> RemoteResult<Self::Sftp> {
        self.session.set_blocking(true);

        Ok(LibSsh2Sftp {
            inner: self.session.sftp().map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!("Could not create SFTP session: {err}"),
                )
            })?,
        })
    }
}

struct SftpFileReader(ssh2::File);

struct SftpFileWriter(ssh2::File);

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

impl Sftp for LibSsh2Sftp {
    fn mkdir(&self, path: &Path, mode: i32) -> RemoteResult<()> {
        self.inner.mkdir(path, mode).map_err(|err| {
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
            .open(path)
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
                ssh2::OpenFlags::WRITE | ssh2::OpenFlags::APPEND | ssh2::OpenFlags::CREATE
            }
            super::WriteMode::Truncate => {
                ssh2::OpenFlags::WRITE | ssh2::OpenFlags::CREATE | ssh2::OpenFlags::TRUNCATE
            }
        };

        self.inner
            .open_mode(path, flags, mode, OpenType::File)
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
            .readdir(dirname)
            .map(|files| {
                files
                    .into_iter()
                    .map(|(path, metadata)| self.make_fsentry(path.as_path(), &metadata))
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
        self.inner.realpath(path).map_err(|err| {
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
            .rename(src, dest, Some(RenameFlags::OVERWRITE))
            .map_err(|err| {
                RemoteError::new_ex(
                    RemoteErrorType::ProtocolError,
                    format!("Could not rename file '{src}': {err}", src = src.display()),
                )
            })
    }

    fn rmdir(&self, path: &Path) -> RemoteResult<()> {
        self.inner.rmdir(path).map_err(|err| {
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
            .setstat(path, Self::metadata_to_filestat(metadata))
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
            .stat(filename)
            .map(|metadata| self.make_fsentry(filename, &metadata))
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
        self.inner.symlink(path, target).map_err(|err| {
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
        self.inner.unlink(path).map_err(|err| {
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

impl LibSsh2Sftp {
    fn metadata_to_filestat(metadata: Metadata) -> FileStat {
        let atime = metadata
            .accessed
            .and_then(|x| x.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|x| x.as_secs());
        let mtime = metadata
            .modified
            .and_then(|x| x.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|x| x.as_secs());
        FileStat {
            size: Some(metadata.size),
            uid: metadata.uid,
            gid: metadata.gid,
            perm: metadata.mode.map(u32::from),
            atime,
            mtime,
        }
    }

    fn make_fsentry(&self, path: &Path, metadata: &FileStat) -> File {
        let name = match path.file_name() {
            None => "/".to_string(),
            Some(name) => name.to_string_lossy().to_string(),
        };
        debug!("Found file {name}");
        // parse metadata
        let uid = metadata.uid;
        let gid = metadata.gid;
        let mode = metadata.perm.map(UnixPex::from);
        let size = metadata.size.unwrap_or(0);
        let accessed = metadata.atime.map(|x| {
            SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_secs(x))
                .unwrap_or(SystemTime::UNIX_EPOCH)
        });
        let modified = metadata.mtime.map(|x| {
            SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_secs(x))
                .unwrap_or(SystemTime::UNIX_EPOCH)
        });
        let symlink = match metadata.file_type().is_symlink() {
            false => None,
            true => match self.inner.readlink(path) {
                Ok(p) => Some(p),
                Err(err) => {
                    error!(
                        "Failed to read link of {} (even it's supposed to be a symlink): {}",
                        path.display(),
                        err
                    );
                    None
                }
            },
        };
        let file_type = if symlink.is_some() {
            FileType::Symlink
        } else if metadata.is_dir() {
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

fn perform_shell_cmd<S: AsRef<str>>(session: &mut ssh2::Session, cmd: S) -> RemoteResult<String> {
    // Create channel
    trace!("Running command: {}", cmd.as_ref());
    let mut channel = match session.channel_session() {
        Ok(ch) => ch,
        Err(err) => {
            return Err(RemoteError::new_ex(
                RemoteErrorType::ProtocolError,
                format!("Could not open channel: {err}"),
            ));
        }
    };

    // escape single quotes in command
    let cmd = cmd.as_ref().replace('\'', r#"'\''"#); // close, escape, and reopen

    // Execute command; always execute inside of sh -c to have proper shell behavior.
    // if the remote peer has fish or other non-bash shell as default, commands like
    // "cd /some/dir; somecommand" may fail.
    if let Err(err) = channel.exec(format!("sh -c '{cmd}'").as_str()) {
        return Err(RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Could not execute command \"{cmd}\": {err}"),
        ));
    }
    // Read output
    let mut output: String = String::new();
    match channel.read_to_string(&mut output) {
        Ok(_) => {
            // Wait close
            let _ = channel.wait_close();
            trace!("Command output: {output}");
            Ok(output)
        }
        Err(err) => Err(RemoteError::new_ex(
            RemoteErrorType::ProtocolError,
            format!("Could not read output: {err}"),
        )),
    }
}

/// connect to socket address with provided timeout.
/// If timeout is zero, don't set timeout
fn tcp_connect(address: &SocketAddr, timeout: Duration) -> std::io::Result<TcpStream> {
    if timeout.is_zero() {
        TcpStream::connect(address)
    } else {
        TcpStream::connect_timeout(address, timeout)
    }
}

/// Configure algorithm preferences into session
fn set_algo_prefs(
    session: &mut ssh2::Session,
    opts: &SshOpts,
    config: &Config,
) -> RemoteResult<()> {
    // Configure preferences from config
    let params = &config.params;
    trace!("Configuring algorithm preferences...");
    if let Some(compress) = params.compression {
        trace!("compression: {compress}");
        session.set_compress(compress);
    }

    // kex
    let algos = params.kex_algorithms.algorithms().join(",");
    trace!("Configuring KEX algorithms: {algos}");
    if let Err(err) = session.method_pref(ssh2::MethodType::Kex, algos.as_str()) {
        error!("Could not set KEX algorithms: {err}");
        return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
    }

    // HostKey
    let algos = params.host_key_algorithms.algorithms().join(",");
    trace!("Configuring HostKey algorithms: {algos}");
    if let Err(err) = session.method_pref(ssh2::MethodType::HostKey, algos.as_str()) {
        error!("Could not set host key algorithms: {err}");
        return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
    }

    // ciphers
    let algos = params.ciphers.algorithms().join(",");
    trace!("Configuring Crypt algorithms: {algos}");
    if let Err(err) = session.method_pref(ssh2::MethodType::CryptCs, algos.as_str()) {
        error!("Could not set crypt algorithms (client-server): {err}");
        return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
    }
    if let Err(err) = session.method_pref(ssh2::MethodType::CryptSc, algos.as_str()) {
        error!("Could not set crypt algorithms (server-client): {err}");
        return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
    }

    // MAC
    let algos = params.mac.algorithms().join(",");
    trace!("Configuring MAC algorithms: {algos}");
    if let Err(err) = session.method_pref(ssh2::MethodType::MacCs, algos.as_str()) {
        error!("Could not set MAC algorithms (client-server): {err}");
        return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
    }
    if let Err(err) = session.method_pref(ssh2::MethodType::MacSc, algos.as_str()) {
        error!("Could not set MAC algorithms (server-client): {err}");
        return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
    }

    // -- configure algos from opts
    for method in opts.methods.iter() {
        let algos = method.prefs();
        trace!("Configuring {:?} algorithm: {}", method.method_type, algos);
        if let Err(err) = session.method_pref(method.method_type.into(), algos.as_str()) {
            error!("Could not set {:?} algorithms: {}", method.method_type, err);
            return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
        }
    }
    Ok(())
}

/// Authenticate on session with ssh agent
fn session_auth_with_agent(
    session: &mut ssh2::Session,
    username: &str,
    ssh_agent_config: &SshAgentIdentity,
) -> RemoteResult<()> {
    let mut agent = session
        .agent()
        .map_err(|err| RemoteError::new_ex(RemoteErrorType::ConnectionError, err))?;

    agent
        .connect()
        .map_err(|err| RemoteError::new_ex(RemoteErrorType::ConnectionError, err))?;

    agent
        .list_identities()
        .map_err(|err| RemoteError::new_ex(RemoteErrorType::ConnectionError, err))?;

    let mut connection_result = Err(RemoteError::new(RemoteErrorType::AuthenticationFailed));

    for identity in agent
        .identities()
        .map_err(|err| RemoteError::new_ex(RemoteErrorType::ConnectionError, err))?
    {
        if ssh_agent_config.pubkey_matches(identity.blob()) {
            debug!("Trying to authenticate with ssh agent with key: {identity:?}");
        } else {
            continue;
        }
        match agent.userauth(username, &identity) {
            Ok(()) => {
                connection_result = Ok(());
                debug!("Authenticated with ssh agent with key: {identity:?}");
                break;
            }
            Err(err) => {
                debug!("SSH agent auth failed: {err}");
                connection_result = Err(RemoteError::new_ex(
                    RemoteErrorType::AuthenticationFailed,
                    err,
                ));
            }
        }
    }

    if let Err(err) = agent.disconnect() {
        warn!("Could not disconnect from ssh agent: {err}");
    }

    connection_result
}

/// Authenticate on session with private key
fn session_auth_with_rsakey(
    session: &mut ssh2::Session,
    username: &str,
    private_key: &Path,
    password: Option<&str>,
    identity_file: Option<&[PathBuf]>,
) -> RemoteResult<()> {
    debug!("Authenticating with username '{username}' and RSA key");
    let mut keys = vec![private_key];
    if let Some(identity_file) = identity_file {
        let other_keys: Vec<&Path> = identity_file.iter().map(|x| x.as_path()).collect();
        keys.extend(other_keys);
    }
    // iterate over keys
    for key in keys.into_iter() {
        trace!("Trying to authenticate with RSA key at '{}'", key.display());
        match session.userauth_pubkey_file(username, None, key, password) {
            Ok(_) => {
                debug!("Authenticated with key at '{}'", key.display());
                return Ok(());
            }
            Err(err) => {
                error!("Authentication failed: {err}");
            }
        }
    }
    Err(RemoteError::new_ex(
        RemoteErrorType::AuthenticationFailed,
        "could not find any suitable RSA key to authenticate with",
    ))
}

/// Authenticate on session with the provided [`Authentication`] method.
fn session_auth(
    session: &mut ssh2::Session,
    opts: &SshOpts,
    ssh_config: &Config,
    authentication: Authentication,
) -> RemoteResult<()> {
    match authentication {
        Authentication::RsaKey(private_key) => session_auth_with_rsakey(
            session,
            &ssh_config.username,
            private_key.as_path(),
            opts.password.as_deref(),
            ssh_config.params.identity_file.as_deref(),
        ),
        Authentication::Password(password) => {
            session_auth_with_password(session, &ssh_config.username, &password)
        }
    }
}

/// Authenticate on session with username and password
fn session_auth_with_password(
    session: &mut ssh2::Session,
    username: &str,
    password: &str,
) -> RemoteResult<()> {
    // Username / password
    debug!("Authenticating with username '{username}' and password");
    if let Err(err) = session.userauth_password(username, password) {
        error!("Authentication failed: {err}");
        Err(RemoteError::new_ex(
            RemoteErrorType::AuthenticationFailed,
            err,
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use ssh2_config::ParseRule;

    use super::*;
    use crate::mock::ssh as ssh_mock;

    #[test]
    fn should_connect_to_ssh_server_auth_user_password() {
        use crate::ssh::container::OpensshServer;

        let container = OpensshServer::start();
        let port = container.port();

        crate::mock::logger();
        let config_file = ssh_mock::create_ssh_config(port);
        let opts = SshOpts::new("sftp")
            .config_file(config_file.path(), ParseRule::ALLOW_UNKNOWN_FIELDS)
            .password("password");

        if let Err(err) = LibSsh2Session::connect(&opts) {
            panic!("Could not connect to server: {err}");
        }
        let session = LibSsh2Session::connect(&opts).unwrap();
        assert!(session.authenticated().unwrap());

        drop(container);
    }

    #[test]
    fn should_connect_to_ssh_server_auth_key() {
        use crate::ssh::container::OpensshServer;

        let container = OpensshServer::start();
        let port = container.port();

        crate::mock::logger();
        let config_file = ssh_mock::create_ssh_config(port);
        let opts = SshOpts::new("sftp")
            .config_file(config_file.path(), ParseRule::ALLOW_UNKNOWN_FIELDS)
            .key_storage(Box::new(ssh_mock::MockSshKeyStorage::default()));
        let session = LibSsh2Session::connect(&opts).unwrap();
        assert!(session.authenticated().unwrap());
    }

    #[test]

    fn should_perform_shell_command_on_server() {
        crate::mock::logger();
        let container = crate::ssh::container::OpensshServer::start();
        let port = container.port();

        let opts = SshOpts::new("127.0.0.1")
            .port(port)
            .username("sftp")
            .password("password");
        let mut session = LibSsh2Session::connect(&opts).unwrap();
        assert!(session.authenticated().unwrap());
        // run commands
        assert!(session.cmd("pwd").is_ok());
    }

    #[test]

    fn should_perform_shell_command_on_server_and_return_exit_code() {
        crate::mock::logger();
        let container = crate::ssh::container::OpensshServer::start();
        let port = container.port();

        let opts = SshOpts::new("127.0.0.1")
            .port(port)
            .username("sftp")
            .password("password");
        let mut session = LibSsh2Session::connect(&opts).unwrap();
        assert!(session.authenticated().unwrap());
        // run commands
        assert_eq!(
            session.cmd_at("pwd", Path::new("/tmp")).ok().unwrap(),
            (0, String::from("/tmp\n"))
        );
        assert_eq!(
            session
                .cmd_at("pippopluto", Path::new("/tmp"))
                .ok()
                .unwrap()
                .0,
            127
        );
    }

    #[test]
    fn should_fail_authentication() {
        crate::mock::logger();
        let container = crate::ssh::container::OpensshServer::start();
        let port = container.port();

        let opts = SshOpts::new("127.0.0.1")
            .port(port)
            .username("sftp")
            .password("ippopotamo");
        assert!(LibSsh2Session::connect(&opts).is_err());
    }

    #[test]
    fn test_filetransfer_sftp_bad_server() {
        crate::mock::logger();
        let opts = SshOpts::new("myverybad.verybad.server")
            .port(10022)
            .username("sftp")
            .password("ippopotamo");
        assert!(LibSsh2Session::connect(&opts).is_err());
    }
}
