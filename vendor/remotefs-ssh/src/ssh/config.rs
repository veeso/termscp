//! ## Config
//!
//! implements configuration resolver for ssh

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

use remotefs::{RemoteError, RemoteErrorType, RemoteResult};
use ssh2_config::{DefaultAlgorithms, HostParams, ParseRule, SshConfig};

use super::SshOpts;

/// Ssh configuration params
pub struct Config {
    pub params: HostParams,
    pub host: String,
    /// Host resolved from configuration
    pub resolved_host: String,
    /// Address is host:port
    pub address: String,
    pub username: String,
    pub connection_timeout: Duration,
    pub connection_attempts: usize,
}

impl Config {
    // -- private

    /// Create `Config` from `HostParams` and `SshOpts`
    fn from_params(params: HostParams, opts: &SshOpts) -> Self {
        Config {
            host: opts.host.to_string(),
            resolved_host: Self::resolve_host(&params, opts),
            address: Self::resolve_address(&params, opts),
            username: Self::resolve_username(&params, opts),
            connection_timeout: Self::resolve_connection_timeout(&params, opts),
            connection_attempts: Self::resolve_connection_attempts(&params),
            params,
        }
    }

    /// Parse config at `p` and get params for `host`
    fn parse(p: &Path, host: &str, rules: ParseRule) -> RemoteResult<HostParams> {
        trace!("Parsing configuration at {}", p.display());
        let mut reader = BufReader::new(File::open(p).map_err(|e| {
            RemoteError::new_ex(
                RemoteErrorType::IoError,
                format!("Could not open configuration file: {e}"),
            )
        })?);
        SshConfig::default()
            .parse(&mut reader, rules)
            .map_err(|e| {
                RemoteError::new_ex(
                    RemoteErrorType::IoError,
                    format!("Could not parse configuration file: {e}"),
                )
            })
            .map(|x| x.query(host))
    }

    /// Given host params and ssh options, returns resolved remote host
    fn resolve_host(params: &HostParams, opts: &SshOpts) -> String {
        // Host should be overridden
        match params.host_name.as_deref() {
            Some(h) => h.to_string(),
            None => opts.host.to_string(),
        }
    }

    /// Given host params and ssh options, returns resolved remote address
    fn resolve_address(params: &HostParams, opts: &SshOpts) -> String {
        let host = Self::resolve_host(params, opts);
        // Opts.port has priority
        let port = match opts.port {
            None => params.port.unwrap_or(22),
            Some(p) => p,
        };
        format!("{host}:{port}")
    }

    /// Resolve username from opts and params.
    /// If defined in opts, get username in opts,
    /// if define in params and not in opts, get from params,
    /// otherwise empty string
    fn resolve_username(params: &HostParams, opts: &SshOpts) -> String {
        match opts.username.as_ref() {
            Some(u) => u.to_string(),
            None => params.user.as_deref().unwrap_or("").to_string(),
        }
    }

    /// Given host params, resolve connection timeout
    fn resolve_connection_timeout(params: &HostParams, opts: &SshOpts) -> Duration {
        match opts.connection_timeout {
            Some(t) => t,
            None => params
                .connect_timeout
                .unwrap_or_else(|| Duration::from_secs(30)),
        }
    }

    /// Given host params, resolve connection attempts.
    /// If `none`, gets 1
    fn resolve_connection_attempts(params: &HostParams) -> usize {
        params.connection_attempts.unwrap_or(1)
    }
}

impl TryFrom<&SshOpts> for Config {
    type Error = RemoteError;

    fn try_from(opts: &SshOpts) -> Result<Self, Self::Error> {
        if let Some(p) = opts.config_file.as_deref() {
            let params = Self::parse(p, opts.host.as_str(), opts.parse_rules)?;
            Ok(Self::from_params(params, opts))
        } else {
            let params = HostParams::new(&DefaultAlgorithms::default());
            Ok(Self::from_params(params, opts))
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::mock::ssh as ssh_mock;

    #[test]
    fn should_init_config_from_default_ssh_opts() {
        let opts = SshOpts::new("192.168.1.1");
        let config = Config::try_from(&opts).ok().unwrap();
        assert_eq!(config.connection_attempts, 1);
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
        assert_eq!(config.address.as_str(), "192.168.1.1:22");
        assert_eq!(config.host.as_str(), "192.168.1.1");
        assert!(config.username.is_empty());
        assert_eq!(
            config.params,
            HostParams::new(&DefaultAlgorithms::default())
        );
    }

    #[test]
    fn should_init_config_from_custom_opts() {
        let opts = SshOpts::new("192.168.1.1")
            .connection_timeout(Duration::from_secs(10))
            .port(2222)
            .username("omar");
        let config = Config::try_from(&opts).ok().unwrap();
        assert_eq!(config.connection_attempts, 1);
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
        assert_eq!(config.host.as_str(), "192.168.1.1");
        assert_eq!(config.address.as_str(), "192.168.1.1:2222");
        assert_eq!(config.username.as_str(), "omar");
        assert_eq!(
            config.params,
            HostParams::new(&DefaultAlgorithms::default())
        );
    }

    #[test]
    fn should_init_config_from_file() {
        let config_file = ssh_mock::create_ssh_config(22);
        let opts = SshOpts::new("sftp").config_file(config_file.path(), ParseRule::STRICT);
        let config = Config::try_from(&opts).ok().unwrap();
        assert_eq!(config.connection_attempts, 3);
        assert_eq!(config.connection_timeout, Duration::from_secs(60));
        assert_eq!(config.host.as_str(), "sftp");
        assert_eq!(config.resolved_host.as_str(), "127.0.0.1");
        assert_eq!(config.address.as_str(), "127.0.0.1:22");
        assert_eq!(config.username.as_str(), "sftp");
        assert_ne!(
            config.params,
            HostParams::new(&DefaultAlgorithms::default())
        );
    }

    #[test]
    fn should_init_config_from_file_with_override() {
        let config_file = ssh_mock::create_ssh_config(22);
        let opts = SshOpts::new("sftp")
            .config_file(config_file.path(), ParseRule::STRICT)
            .connection_timeout(Duration::from_secs(10))
            .port(22)
            .username("omar");
        let config = Config::try_from(&opts).ok().unwrap();
        assert_eq!(config.connection_attempts, 3);
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
        assert_eq!(config.host.as_str(), "sftp");
        assert_eq!(config.resolved_host.as_str(), "127.0.0.1");
        assert_eq!(config.address.as_str(), "127.0.0.1:22");
        assert_eq!(config.username.as_str(), "omar");
        assert_ne!(
            config.params,
            HostParams::new(&DefaultAlgorithms::default())
        );
    }
}
