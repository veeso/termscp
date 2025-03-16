//! ## builder
//!
//! Remotefs client builder

use std::path::PathBuf;
use std::sync::Arc;

use remotefs::RemoteFs;
use remotefs_aws_s3::AwsS3Fs;
use remotefs_ftp::FtpFs;
use remotefs_kube::KubeMultiPodFs as KubeFs;
#[cfg(smb_unix)]
use remotefs_smb::SmbOptions;
#[cfg(smb)]
use remotefs_smb::{SmbCredentials, SmbFs};
use remotefs_ssh::{ScpFs, SftpFs, SshAgentIdentity, SshConfigParseRule, SshOpts};
use remotefs_webdav::WebDAVFs;

#[cfg(not(smb))]
use super::params::{AwsS3Params, GenericProtocolParams};
#[cfg(smb)]
use super::params::{AwsS3Params, GenericProtocolParams, SmbParams};
use super::params::{KubeProtocolParams, WebDAVProtocolParams};
use super::{FileTransferProtocol, ProtocolParams};
use crate::system::config_client::ConfigClient;
use crate::system::sshkey_storage::SshKeyStorage;
use crate::utils::ssh as ssh_utils;

/// Remotefs builder
pub struct RemoteFsBuilder;

impl RemoteFsBuilder {
    /// Build RemoteFs client from protocol and params.
    ///
    /// if protocol and parameters are inconsistent, the function will panic.
    pub fn build(
        protocol: FileTransferProtocol,
        params: ProtocolParams,
        config_client: &ConfigClient,
    ) -> Result<Box<dyn RemoteFs>, String> {
        match (protocol, params) {
            (FileTransferProtocol::AwsS3, ProtocolParams::AwsS3(params)) => {
                Ok(Box::new(Self::aws_s3_client(params)))
            }
            (FileTransferProtocol::Ftp(secure), ProtocolParams::Generic(params)) => {
                Ok(Box::new(Self::ftp_client(params, secure)))
            }
            (FileTransferProtocol::Kube, ProtocolParams::Kube(params)) => {
                Ok(Box::new(Self::kube_client(params)))
            }
            (FileTransferProtocol::Scp, ProtocolParams::Generic(params)) => {
                Ok(Box::new(Self::scp_client(params, config_client)))
            }
            (FileTransferProtocol::Sftp, ProtocolParams::Generic(params)) => {
                Ok(Box::new(Self::sftp_client(params, config_client)))
            }
            #[cfg(smb)]
            (FileTransferProtocol::Smb, ProtocolParams::Smb(params)) => {
                Ok(Box::new(Self::smb_client(params)))
            }
            (FileTransferProtocol::WebDAV, ProtocolParams::WebDAV(params)) => {
                Ok(Box::new(Self::webdav_client(params)))
            }
            (protocol, params) => {
                error!("Invalid params for protocol '{:?}'", protocol);
                Err(format!(
                    "Invalid protocol '{protocol:?}' with parameters of type {params:?}",
                ))
            }
        }
    }

    /// Build aws s3 client from parameters
    fn aws_s3_client(params: AwsS3Params) -> AwsS3Fs {
        let rt = Arc::new(
            tokio::runtime::Builder::new_current_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .expect("Unable to create tokio runtime"),
        );
        let mut client =
            AwsS3Fs::new(params.bucket_name, &rt).new_path_style(params.new_path_style);
        if let Some(region) = params.region {
            client = client.region(region);
        }
        if let Some(profile) = params.profile {
            client = client.profile(profile);
        }
        if let Some(endpoint) = params.endpoint {
            client = client.endpoint(endpoint);
        }
        if let Some(access_key) = params.access_key {
            client = client.access_key(access_key);
        }
        if let Some(secret_access_key) = params.secret_access_key {
            client = client.secret_access_key(secret_access_key);
        }
        if let Some(security_token) = params.security_token {
            client = client.security_token(security_token);
        }
        if let Some(session_token) = params.session_token {
            client = client.session_token(session_token);
        }
        client
    }

    /// Build ftp client from parameters
    fn ftp_client(params: GenericProtocolParams, secure: bool) -> FtpFs {
        let mut client = FtpFs::new(params.address, params.port).passive_mode();
        if let Some(username) = params.username {
            client = client.username(username);
        }
        if let Some(password) = params.password {
            client = client.password(password);
        }
        if secure {
            client = client.secure(true, true);
        }
        client
    }

    /// Build kube client
    fn kube_client(params: KubeProtocolParams) -> KubeFs {
        let rt = Arc::new(
            tokio::runtime::Builder::new_current_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .expect("Unable to create tokio runtime"),
        );
        let kube_fs = KubeFs::new(&rt);
        if let Some(config) = params.config() {
            kube_fs.config(config)
        } else {
            kube_fs
        }
    }

    /// Build scp client
    fn scp_client(params: GenericProtocolParams, config_client: &ConfigClient) -> ScpFs {
        Self::build_ssh_opts(params, config_client).into()
    }

    /// Build sftp client
    fn sftp_client(params: GenericProtocolParams, config_client: &ConfigClient) -> SftpFs {
        Self::build_ssh_opts(params, config_client).into()
    }

    #[cfg(smb_unix)]
    fn smb_client(params: SmbParams) -> SmbFs {
        let mut credentials = SmbCredentials::default()
            .server(format!("smb://{}:{}", params.address, params.port))
            .share(params.share);

        if let Some(username) = params.username {
            credentials = credentials.username(username);
        }
        if let Some(password) = params.password {
            credentials = credentials.password(password);
        }
        if let Some(workgroup) = params.workgroup {
            credentials = credentials.workgroup(workgroup);
        }

        match SmbFs::try_new(
            credentials,
            SmbOptions::default()
                .one_share_per_server(true)
                .case_sensitive(false),
        ) {
            Ok(fs) => fs,
            Err(e) => {
                error!("Invalid params for protocol SMB: {e}");
                panic!("Invalid params for protocol SMB: {e}")
            }
        }
    }

    #[cfg(smb_windows)]
    fn smb_client(params: SmbParams) -> SmbFs {
        let mut credentials = SmbCredentials::new(params.address, params.share);

        if let Some(username) = params.username {
            credentials = credentials.username(username);
        }
        if let Some(password) = params.password {
            credentials = credentials.password(password);
        }

        SmbFs::new(credentials)
    }

    fn webdav_client(params: WebDAVProtocolParams) -> WebDAVFs {
        WebDAVFs::new(&params.username, &params.password, &params.uri)
    }

    /// Build ssh options from generic protocol params and client configuration
    fn build_ssh_opts(params: GenericProtocolParams, config_client: &ConfigClient) -> SshOpts {
        let mut opts = SshOpts::new(params.address.clone())
            .key_storage(Box::new(Self::make_ssh_storage(config_client)))
            .ssh_agent_identity(Some(SshAgentIdentity::All))
            .port(params.port);
        // get ssh config
        let ssh_config = config_client
            .get_ssh_config()
            .and_then(|path| {
                debug!("reading ssh config at {}", path);
                ssh_utils::parse_ssh2_config(path).ok()
            })
            .map(|config| config.query(&params.address));

        //* override port
        if let Some(port) = ssh_config.as_ref().and_then(|config| config.port) {
            opts = opts.port(port);
        }

        //*  get username. Case 1 provided in params
        if let Some(username) = params.username {
            opts = opts.username(username);
        } else if let Some(ssh_config) = &ssh_config {
            debug!("no username was provided, checking whether a user is set for this host");
            if let Some(username) = &ssh_config.user {
                debug!("found username from config: {username}");
                opts = opts.username(username);
            } else {
                //* case 3: use system username; can't be None
                debug!("no username was provided, using current username");
                opts = opts.username(whoami::username());
            }
        } else {
            //* case 3: use system username; can't be None
            debug!("no username was provided, using current username");
            opts = opts.username(whoami::username());
        }
        if let Some(password) = params.password {
            opts = opts.password(password);
        }
        if let Some(config_path) = config_client.get_ssh_config() {
            opts = opts.config_file(
                PathBuf::from(config_path),
                SshConfigParseRule::ALLOW_UNKNOWN_FIELDS,
            );
        }
        opts
    }

    /// Make ssh storage from `ConfigClient` if possible, empty otherwise (empty is implicit if degraded)
    fn make_ssh_storage(config_client: &ConfigClient) -> SshKeyStorage {
        SshKeyStorage::from(config_client)
    }
}

#[cfg(test)]
mod test {

    use std::path::{Path, PathBuf};

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn should_build_aws_s3_fs() {
        let params = ProtocolParams::AwsS3(
            AwsS3Params::new("omar", Some("eu-west-1"), Some("test"))
                .endpoint(Some("http://localhost:9000"))
                .new_path_style(true)
                .access_key(Some("pippo"))
                .secret_access_key(Some("pluto"))
                .security_token(Some("omar"))
                .session_token(Some("gerry-scotti")),
        );
        let config_client = get_config_client();
        let _ = RemoteFsBuilder::build(FileTransferProtocol::AwsS3, params, &config_client);
    }

    #[test]
    fn should_build_ftp_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(21)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = RemoteFsBuilder::build(FileTransferProtocol::Ftp(true), params, &config_client);
    }

    #[test]
    fn test_should_build_kube_fs() {
        let params = ProtocolParams::Kube(KubeProtocolParams {
            namespace: Some("namespace".to_string()),
            cluster_url: Some("cluster_url".to_string()),
            username: Some("username".to_string()),
            client_cert: Some("client_cert".to_string()),
            client_key: Some("client_key".to_string()),
        });
        let config_client = get_config_client();
        let _ = RemoteFsBuilder::build(FileTransferProtocol::Kube, params, &config_client);
    }

    #[test]
    fn should_build_scp_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(22)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = RemoteFsBuilder::build(FileTransferProtocol::Scp, params, &config_client);
    }

    #[test]
    fn should_build_sftp_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(22)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = RemoteFsBuilder::build(FileTransferProtocol::Sftp, params, &config_client);
    }

    #[test]
    #[cfg(smb)]
    fn should_build_smb_fs() {
        let params = ProtocolParams::Smb(SmbParams::new("localhost", "share"));
        let config_client = get_config_client();
        let _ = RemoteFsBuilder::build(FileTransferProtocol::Smb, params, &config_client);
    }

    #[test]
    #[should_panic]
    fn should_not_build_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(22)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        let _ = RemoteFsBuilder::build(FileTransferProtocol::AwsS3, params, &config_client);
    }

    fn get_config_client() -> ConfigClient {
        let tmp_dir: TempDir = TempDir::new().ok().unwrap();
        let (cfg_path, ssh_keys_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        ConfigClient::new(cfg_path.as_path(), ssh_keys_path.as_path())
            .ok()
            .unwrap()
    }

    /// Get paths for configuration and keys directory
    fn get_paths(dir: &Path) -> (PathBuf, PathBuf) {
        let mut k: PathBuf = PathBuf::from(dir);
        let mut c: PathBuf = k.clone();
        k.push("ssh-keys/");
        c.push("config.toml");
        (c, k)
    }
}
