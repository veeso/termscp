//! ## SSH
//!
//! implements the file transfer for SSH based protocols: SFTP and SCP

// -- ext
use std::path::{Path, PathBuf};
use std::time::Duration;

// -- modules
mod backend;
mod config;
#[cfg(test)]
mod container;
mod key_method;
mod scp;
mod sftp;

pub use ssh2_config::ParseRule;

#[cfg(feature = "libssh2")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh2")))]
pub use self::backend::LibSsh2Session;
#[cfg(feature = "libssh")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh")))]
pub use self::backend::LibSshSession;
pub use self::backend::SshSession;
pub use self::key_method::{KeyMethod, MethodType};
pub use self::scp::ScpFs;
pub use self::sftp::SftpFs;

// -- Ssh key storage

/// This trait must be implemented in order to use ssh keys for authentication for sftp/scp.
pub trait SshKeyStorage: Send + Sync {
    /// Return RSA key path from host and username
    fn resolve(&self, host: &str, username: &str) -> Option<PathBuf>;
}

// -- ssh options

/// Ssh agent identity
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SshAgentIdentity {
    /// Try all identities
    All,
    /// Use a specific identity
    Pubkey(Vec<u8>),
}

impl From<Vec<u8>> for SshAgentIdentity {
    fn from(v: Vec<u8>) -> Self {
        SshAgentIdentity::Pubkey(v)
    }
}

impl From<&[u8]> for SshAgentIdentity {
    fn from(v: &[u8]) -> Self {
        SshAgentIdentity::Pubkey(v.to_vec())
    }
}

impl SshAgentIdentity {
    /// Check if the provided public key matches the identity
    ///
    /// If [`SshAgentIdentity::All`] is provided, this method will always return `true`
    pub(crate) fn pubkey_matches(&self, blob: &[u8]) -> bool {
        match self {
            SshAgentIdentity::All => true,
            SshAgentIdentity::Pubkey(v) => v == blob,
        }
    }
}

/// Ssh options;
/// used to build and configure SCP/SFTP client.
///
/// ### Conflict resolution
///
/// You may specify some options that can be in conflict (e.g. `port` and `Port` parameter in ssh configuration).
/// In these cases, the resolution is performed in this order (from highest, to lower priority):
///
/// 1. [`SshOpts`] attribute (e.g. `port` or `username`)
/// 2. Ssh configuration
///
/// This applies also to ciphers and key exchange methods.
///
pub struct SshOpts {
    /// hostname of the remote ssh server
    host: String,
    /// Port of the remote ssh server
    port: Option<u16>,
    /// Username to authenticate with
    username: Option<String>,
    /// Password to authenticate or to decrypt RSA key
    password: Option<String>,
    /// Connection timeout (default 30 seconds)
    connection_timeout: Option<Duration>,
    /// SSH configuration file. If provided will be parsed on connect.
    config_file: Option<PathBuf>,
    /// Key storage
    key_storage: Option<Box<dyn SshKeyStorage>>,
    /// Preferred key exchange methods.
    methods: Vec<KeyMethod>,
    /// Ssh config parser ruleset
    parse_rules: ParseRule,
    /// Ssh agent configuration for authentication
    ssh_agent_identity: Option<SshAgentIdentity>,
}

impl SshOpts {
    /// Initialize [`SshOpts`].
    /// You must define the host you want to connect to.
    /// Host may be resolved by ssh configuration, if specified.
    ///
    /// Other options can be specified with other constructors.
    pub fn new<S: AsRef<str>>(host: S) -> Self {
        Self {
            host: host.as_ref().to_string(),
            port: None,
            username: None,
            password: None,
            connection_timeout: None,
            config_file: None,
            key_storage: None,
            methods: Vec::default(),
            parse_rules: ParseRule::STRICT,
            ssh_agent_identity: None,
        }
    }

    /// Specify the port the remote server is listening to.
    /// This option will override an eventual port specified for the current host in the ssh configuration
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set username to log in as
    /// This option will override an eventual username specified for the current host in the ssh configuration
    pub fn username<S: AsRef<str>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_string());
        self
    }

    /// Set password to authenticate with
    pub fn password<S: AsRef<str>>(mut self, password: S) -> Self {
        self.password = Some(password.as_ref().to_string());
        self
    }

    /// Set connection timeout
    /// This option will override an eventual connection timeout specified for the current host in the ssh configuration
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = Some(timeout);
        self
    }

    /// Set configuration for ssh agent
    ///
    /// If `None` the ssh agent will be disabled
    ///
    /// If `Some(SshAgentIdentity::All)` all identities will be tried
    /// Otherwise the provided public key will be used
    pub fn ssh_agent_identity(mut self, ssh_agent_identity: Option<SshAgentIdentity>) -> Self {
        self.ssh_agent_identity = ssh_agent_identity;
        self
    }

    /// Set SSH configuration file to read
    ///
    /// The supported options are:
    ///
    /// - Host block
    /// - HostName
    /// - Port
    /// - User
    /// - Ciphers
    /// - MACs
    /// - KexAlgorithms
    /// - HostKeyAlgorithms
    /// - ConnectionAttempts
    /// - ConnectTimeout
    pub fn config_file<P: AsRef<Path>>(mut self, p: P, rules: ParseRule) -> Self {
        self.config_file = Some(p.as_ref().to_path_buf());
        self.parse_rules = rules;
        self
    }

    /// Set key storage to read RSA keys from
    pub fn key_storage(mut self, storage: Box<dyn SshKeyStorage>) -> Self {
        self.key_storage = Some(storage);
        self
    }

    /// Add key method to ssh options
    pub fn method(mut self, method: KeyMethod) -> Self {
        self.methods.push(method);
        self
    }
}

#[cfg(feature = "libssh")]
impl From<SshOpts> for SftpFs<LibSshSession> {
    fn from(opts: SshOpts) -> Self {
        Self::libssh(opts)
    }
}

#[cfg(feature = "libssh")]
impl From<SshOpts> for ScpFs<LibSshSession> {
    fn from(opts: SshOpts) -> Self {
        Self::libssh(opts)
    }
}

#[cfg(feature = "libssh2")]
impl From<SshOpts> for SftpFs<LibSsh2Session> {
    fn from(opts: SshOpts) -> Self {
        Self::libssh2(opts)
    }
}

#[cfg(feature = "libssh2")]
impl From<SshOpts> for ScpFs<LibSsh2Session> {
    fn from(opts: SshOpts) -> Self {
        Self::libssh2(opts)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::ssh::MockSshKeyStorage;

    #[test]
    fn should_create_key_method() {
        let key_method = KeyMethod::new(
            MethodType::CryptClientServer,
            &[
                "aes128-ctr".to_string(),
                "aes192-ctr".to_string(),
                "aes256-ctr".to_string(),
                "aes128-cbc".to_string(),
                "3des-cbc".to_string(),
            ],
        );
        assert_eq!(
            key_method.prefs().as_str(),
            "aes128-ctr,aes192-ctr,aes256-ctr,aes128-cbc,3des-cbc"
        );
    }

    #[test]
    fn test_should_tell_whether_pubkey_matches() {
        let identity = SshAgentIdentity::Pubkey(b"hello".to_vec());
        assert!(identity.pubkey_matches(b"hello"));
        assert!(!identity.pubkey_matches(b"world"));

        let identity = SshAgentIdentity::All;
        assert!(identity.pubkey_matches(b"hello"));
    }

    #[test]
    fn should_initialize_ssh_opts() {
        let opts = SshOpts::new("localhost");
        assert_eq!(opts.host.as_str(), "localhost");
        assert!(opts.port.is_none());
        assert!(opts.username.is_none());
        assert!(opts.password.is_none());
        assert!(opts.connection_timeout.is_none());
        assert!(opts.config_file.is_none());
        assert!(opts.key_storage.is_none());
        assert!(opts.methods.is_empty());
    }

    #[test]
    fn should_build_ssh_opts() {
        let opts = SshOpts::new("localhost")
            .port(22)
            .username("foobar")
            .password("qwerty123")
            .connection_timeout(Duration::from_secs(10))
            .config_file(Path::new("/home/pippo/.ssh/config"), ParseRule::STRICT)
            .key_storage(Box::new(MockSshKeyStorage::default()))
            .method(KeyMethod::new(
                MethodType::CryptClientServer,
                &[
                    "aes128-ctr".to_string(),
                    "aes192-ctr".to_string(),
                    "aes256-ctr".to_string(),
                    "aes128-cbc".to_string(),
                    "3des-cbc".to_string(),
                ],
            ));
        assert_eq!(opts.host.as_str(), "localhost");
        assert_eq!(opts.port.unwrap(), 22);
        assert_eq!(opts.username.as_deref().unwrap(), "foobar");
        assert_eq!(opts.password.as_deref().unwrap(), "qwerty123");
        assert_eq!(opts.connection_timeout.unwrap(), Duration::from_secs(10));
        assert_eq!(
            opts.config_file.as_deref().unwrap(),
            Path::new("/home/pippo/.ssh/config")
        );
        assert!(opts.key_storage.is_some());
        assert_eq!(opts.methods.len(), 1);
    }
}
