//! ## builder
//!
//! Remotefs client builder

use std::path::PathBuf;
use std::sync::Arc;

use remotefs::RemoteFs;
use remotefs_aws_s3::AwsS3Fs;
use remotefs_ftp::FtpFs;
use remotefs_gcs::credentials::service_account;
use remotefs_gcs::{GoogleCloudStorageCredentials, GoogleCloudStorageFs};
use remotefs_kube::KubeMultiPodFs as KubeFs;
#[cfg(smb)]
use remotefs_smb::{PavaoSmbCredentials as SmbCredentials, PavaoSmbFs as SmbFs};
#[cfg(smb_unix)]
use remotefs_smb::{PavaoSmbOptions as SmbOptions, SmbDialect as RemoteSmbDialect};
#[cfg(smb_windows)]
use remotefs_smb::{WNetSmbCredentials as SmbCredentials, WNetSmbFs as SmbFs};
use remotefs_ssh::{
    NoCheckServerKey, RusshSession as SshSession, ScpFs, SftpFs, SshAgentIdentity,
    SshConfigParseRule, SshOpts,
};
use remotefs_webdav::WebDAVFs;

#[cfg(smb_unix)]
use super::params::SmbDialect;
#[cfg(not(smb))]
use super::params::{AwsS3Params, GenericProtocolParams, GoogleCloudStorageParams};
#[cfg(smb)]
use super::params::{AwsS3Params, GenericProtocolParams, GoogleCloudStorageParams, SmbParams};
use super::params::{KubeProtocolParams, WebDAVProtocolParams};
use super::{FileTransferProtocol, ProtocolParams};
use crate::system::config_client::ConfigClient;
use crate::system::sshkey_storage::SshKeyStorage;

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
                Ok(Box::new(Self::aws_s3_client(params)?))
            }
            (FileTransferProtocol::Ftp(secure), ProtocolParams::Generic(params)) => {
                Ok(Box::new(Self::ftp_client(params, secure)))
            }
            (
                FileTransferProtocol::GoogleCloudStorage,
                ProtocolParams::GoogleCloudStorage(params),
            ) => Ok(Box::new(Self::gcs_client(params)?)),
            (FileTransferProtocol::Kube, ProtocolParams::Kube(params)) => {
                Ok(Box::new(Self::kube_client(params)?))
            }
            (FileTransferProtocol::Scp, ProtocolParams::Generic(params)) => {
                Ok(Box::new(Self::scp_client(params, config_client)?))
            }
            (FileTransferProtocol::Sftp, ProtocolParams::Generic(params)) => {
                Ok(Box::new(Self::sftp_client(params, config_client)?))
            }
            #[cfg(smb)]
            (FileTransferProtocol::Smb, ProtocolParams::Smb(params)) => {
                Ok(Box::new(Self::smb_client(params)?))
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
    fn aws_s3_client(params: AwsS3Params) -> Result<AwsS3Fs, String> {
        let rt = Arc::new(
            tokio::runtime::Builder::new_current_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .map_err(|e| format!("Unable to create tokio runtime: {e}"))?,
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
        Ok(client)
    }

    /// Build a Google Cloud Storage client from parameters.
    fn gcs_client(params: GoogleCloudStorageParams) -> Result<GoogleCloudStorageFs, String> {
        let runtime = Self::tokio_runtime()?;
        let mut client = match params.service_account_key {
            None => GoogleCloudStorageFs::new(params.bucket_name, &runtime),
            Some(path) => {
                let raw = std::fs::read_to_string(&path).map_err(|error| {
                    format!("Unable to read GCS service-account file '{path}': {error}")
                })?;
                let key = serde_json::from_str(&raw).map_err(|error| {
                    format!("Invalid GCS service-account JSON in '{path}': {error}")
                })?;
                let credentials = {
                    let _guard = runtime.enter();
                    service_account::Builder::new(key).build()
                }
                .map_err(|error| {
                    format!("Invalid GCS service-account credentials in '{path}': {error}")
                })?;
                GoogleCloudStorageFs::with_credentials(
                    params.bucket_name,
                    GoogleCloudStorageCredentials::custom(credentials),
                    &runtime,
                )
            }
        };
        client = client.endpoint(params.endpoint);
        Ok(client)
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
    fn kube_client(params: KubeProtocolParams) -> Result<KubeFs, String> {
        let rt = Self::tokio_runtime()?;
        let kube_fs = KubeFs::new(&rt);
        if let Some(config) = params.config() {
            Ok(kube_fs.config(config))
        } else {
            Ok(kube_fs)
        }
    }

    /// Build scp client
    fn scp_client(
        params: GenericProtocolParams,
        config_client: &ConfigClient,
    ) -> Result<ScpFs<SshSession<NoCheckServerKey>>, String> {
        let opts = Self::build_ssh_opts(params, config_client);
        let rt = Self::tokio_runtime()?;
        Ok(ScpFs::russh(opts, rt))
    }

    /// Build sftp client
    fn sftp_client(
        params: GenericProtocolParams,
        config_client: &ConfigClient,
    ) -> Result<SftpFs<SshSession<NoCheckServerKey>>, String> {
        let opts = Self::build_ssh_opts(params, config_client);
        let rt = Self::tokio_runtime()?;
        Ok(SftpFs::russh(opts, rt))
    }

    /// Maps the user-facing SMB family to inclusive remotefs dialect bounds.
    #[cfg(smb_unix)]
    fn smb_dialect_bounds(dialect: SmbDialect) -> (RemoteSmbDialect, RemoteSmbDialect) {
        match dialect {
            SmbDialect::Auto => (RemoteSmbDialect::Smb202, RemoteSmbDialect::Smb311),
            SmbDialect::Smb1 => (RemoteSmbDialect::Nt1, RemoteSmbDialect::Nt1),
            SmbDialect::Smb2 => (RemoteSmbDialect::Smb202, RemoteSmbDialect::Smb210),
            SmbDialect::Smb3 => (RemoteSmbDialect::Smb300, RemoteSmbDialect::Smb311),
        }
    }

    #[cfg(smb_unix)]
    fn smb_client(params: SmbParams) -> Result<SmbFs, String> {
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

        let (min_dialect, max_dialect) = Self::smb_dialect_bounds(params.dialect);
        SmbFs::try_new_with_dialect(
            credentials,
            SmbOptions::default()
                .one_share_per_server(true)
                .case_sensitive(false),
            min_dialect,
            max_dialect,
        )
        .map_err(|e| {
            error!("Invalid params for protocol SMB: {e}");
            format!("Invalid params for protocol SMB: {e}")
        })
    }

    #[cfg(smb_windows)]
    fn smb_client(params: SmbParams) -> Result<SmbFs, String> {
        let mut credentials = SmbCredentials::new(params.address, params.share);

        if let Some(username) = params.username {
            credentials = credentials.username(username);
        }
        if let Some(password) = params.password {
            credentials = credentials.password(password);
        }

        // Dialect is OS-managed on Windows.
        Ok(SmbFs::new(credentials))
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
        if let Some(username) = params.username {
            opts = opts.username(username);
        } else if let Ok(username) = whoami::username() {
            opts = opts.username(username);
        }
        // For SSH protocols, only set password if explicitly provided and non-empty.
        // This allows the SSH library to prioritize key-based and agent authentication.
        if let Some(password) = params.password
            && !password.is_empty()
        {
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

    /// Create tokio runtime to run async code for remotefs
    fn tokio_runtime() -> Result<Arc<tokio::runtime::Runtime>, String> {
        Ok(Arc::new(
            tokio::runtime::Builder::new_current_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .map_err(|e| format!("Unable to create tokio runtime: {e}"))?,
        ))
    }
}

#[cfg(test)]
mod test {

    use std::path::{Path, PathBuf};

    #[cfg(smb)]
    use serial_test::serial;
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
        assert!(
            RemoteFsBuilder::build(FileTransferProtocol::AwsS3, params, &config_client).is_ok()
        );
    }

    #[test]
    fn should_build_gcs_fs_with_application_default_credentials() {
        let params = ProtocolParams::GoogleCloudStorage(GoogleCloudStorageParams::new("my-bucket"));
        let config_client = get_config_client();

        assert!(
            RemoteFsBuilder::build(
                FileTransferProtocol::GoogleCloudStorage,
                params,
                &config_client,
            )
            .is_ok()
        );
    }

    #[test]
    fn should_reject_missing_gcs_service_account_file() {
        let directory = tempfile::TempDir::new().unwrap();
        let missing = directory.path().join("missing.json");
        let params = GoogleCloudStorageParams::new("my-bucket")
            .service_account_key(Some(missing.to_string_lossy().into_owned()));

        assert!(RemoteFsBuilder::gcs_client(params).is_err());
    }

    #[test]
    fn should_reject_malformed_gcs_service_account_json() {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), "not-json").unwrap();
        let params = GoogleCloudStorageParams::new("my-bucket")
            .service_account_key(Some(file.path().to_string_lossy().into_owned()));

        assert!(RemoteFsBuilder::gcs_client(params).is_err());
    }

    #[test]
    fn should_build_gcs_fs_with_service_account_file() {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            file.path(),
            r#"{
                "type": "service_account",
                "client_email": "termscp@example.iam.gserviceaccount.com",
                "private_key_id": "test-key",
                "private_key": "-----BEGIN PRIVATE KEY-----\ninvalid-test-key\n-----END PRIVATE KEY-----\n",
                "project_id": "termscp-test",
                "universe_domain": "googleapis.com"
            }"#,
        )
        .unwrap();
        let params = GoogleCloudStorageParams::new("my-bucket")
            .service_account_key(Some(file.path().to_string_lossy().into_owned()));

        assert!(RemoteFsBuilder::gcs_client(params).is_ok());
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
        assert!(
            RemoteFsBuilder::build(FileTransferProtocol::Ftp(true), params, &config_client).is_ok()
        );
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
        assert!(RemoteFsBuilder::build(FileTransferProtocol::Kube, params, &config_client).is_ok());
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
        assert!(RemoteFsBuilder::build(FileTransferProtocol::Scp, params, &config_client).is_ok());
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
        assert!(RemoteFsBuilder::build(FileTransferProtocol::Sftp, params, &config_client).is_ok());
    }

    #[test]
    #[cfg(smb)]
    #[serial]
    fn should_build_smb_fs() {
        let params = ProtocolParams::Smb(SmbParams::new("localhost", "share"));
        let config_client = get_config_client();
        assert!(RemoteFsBuilder::build(FileTransferProtocol::Smb, params, &config_client).is_ok());
    }

    #[test]
    #[cfg(smb_unix)]
    fn should_map_smb_dialect_to_bounds() {
        use remotefs_smb::SmbDialect as RemoteSmbDialect;

        use crate::filetransfer::params::SmbDialect;

        assert_eq!(
            RemoteFsBuilder::smb_dialect_bounds(SmbDialect::Auto),
            (RemoteSmbDialect::Smb202, RemoteSmbDialect::Smb311)
        );
        assert_eq!(
            RemoteFsBuilder::smb_dialect_bounds(SmbDialect::Smb1),
            (RemoteSmbDialect::Nt1, RemoteSmbDialect::Nt1)
        );
        assert_eq!(
            RemoteFsBuilder::smb_dialect_bounds(SmbDialect::Smb2),
            (RemoteSmbDialect::Smb202, RemoteSmbDialect::Smb210)
        );
        assert_eq!(
            RemoteFsBuilder::smb_dialect_bounds(SmbDialect::Smb3),
            (RemoteSmbDialect::Smb300, RemoteSmbDialect::Smb311)
        );
    }

    #[test]
    #[cfg(smb)]
    #[serial]
    fn should_build_smb_fs_with_dialect() {
        use crate::filetransfer::params::SmbDialect;

        let params =
            ProtocolParams::Smb(SmbParams::new("localhost", "share").dialect(SmbDialect::Smb1));
        let config_client = get_config_client();
        assert!(RemoteFsBuilder::build(FileTransferProtocol::Smb, params, &config_client).is_ok());
    }

    #[test]
    fn should_not_build_fs() {
        let params = ProtocolParams::Generic(
            GenericProtocolParams::default()
                .address("127.0.0.1")
                .port(22)
                .username(Some("omar"))
                .password(Some("qwerty123")),
        );
        let config_client = get_config_client();
        assert!(
            RemoteFsBuilder::build(FileTransferProtocol::AwsS3, params, &config_client).is_err()
        );
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
