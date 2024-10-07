//! ## Params
//!
//! file transfer parameters

mod aws_s3;
mod kube;
mod smb;
mod webdav;

use std::path::{Path, PathBuf};

pub use self::aws_s3::AwsS3Params;
pub use self::kube::KubeProtocolParams;
pub use self::smb::SmbParams;
pub use self::webdav::WebDAVProtocolParams;
use super::FileTransferProtocol;

/// Host bridge params
#[derive(Debug, Clone)]
pub enum HostBridgeParams {
    /// Localhost with starting working directory
    Localhost(PathBuf),
    /// Remote host with protocol and file transfer params
    Remote(FileTransferProtocol, ProtocolParams),
}

impl HostBridgeParams {
    pub fn unwrap_protocol_params(&self) -> &ProtocolParams {
        match self {
            HostBridgeParams::Localhost(_) => panic!("Localhost has no protocol params"),
            HostBridgeParams::Remote(_, params) => params,
        }
    }
}

/// Holds connection parameters for file transfers
#[derive(Debug, Clone)]
pub struct FileTransferParams {
    pub protocol: FileTransferProtocol,
    pub params: ProtocolParams,
    pub remote_path: Option<PathBuf>,
    pub local_path: Option<PathBuf>,
}

/// Container for protocol params
#[derive(Debug, Clone)]
pub enum ProtocolParams {
    Generic(GenericProtocolParams),
    AwsS3(AwsS3Params),
    Kube(KubeProtocolParams),
    Smb(SmbParams),
    WebDAV(WebDAVProtocolParams),
}

impl ProtocolParams {
    pub fn password_missing(&self) -> bool {
        match self {
            ProtocolParams::AwsS3(params) => params.password_missing(),
            ProtocolParams::Generic(params) => params.password_missing(),
            ProtocolParams::Kube(params) => params.password_missing(),
            ProtocolParams::Smb(params) => params.password_missing(),
            ProtocolParams::WebDAV(params) => params.password_missing(),
        }
    }

    /// Set the secret to ft params for the default secret field for this protocol
    pub fn set_default_secret(&mut self, secret: String) {
        match self {
            ProtocolParams::AwsS3(params) => params.set_default_secret(secret),
            ProtocolParams::Generic(params) => params.set_default_secret(secret),
            ProtocolParams::Kube(params) => params.set_default_secret(secret),
            ProtocolParams::Smb(params) => params.set_default_secret(secret),
            ProtocolParams::WebDAV(params) => params.set_default_secret(secret),
        }
    }

    pub fn host_name(&self) -> String {
        match self {
            ProtocolParams::AwsS3(params) => params.bucket_name.clone(),
            ProtocolParams::Generic(params) => params.address.clone(),
            ProtocolParams::Kube(params) => params
                .namespace
                .as_ref()
                .cloned()
                .unwrap_or_else(|| String::from("default")),
            ProtocolParams::Smb(params) => params.address.clone(),
            ProtocolParams::WebDAV(params) => params.uri.clone(),
        }
    }
}

/// Protocol params used by most common protocols
#[derive(Debug, Clone)]
pub struct GenericProtocolParams {
    pub address: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl FileTransferParams {
    /// Instantiates a new `FileTransferParams`
    pub fn new(protocol: FileTransferProtocol, params: ProtocolParams) -> Self {
        Self {
            protocol,
            params,
            remote_path: None,
            local_path: None,
        }
    }

    /// Set remote directory
    pub fn remote_path<P: AsRef<Path>>(mut self, dir: Option<P>) -> Self {
        self.remote_path = dir.map(|x| x.as_ref().to_path_buf());
        self
    }

    /// Set local directory
    pub fn local_path<P: AsRef<Path>>(mut self, dir: Option<P>) -> Self {
        self.local_path = dir.map(|x| x.as_ref().to_path_buf());
        self
    }

    /// Returns whether a password is supposed to be required for this protocol params.
    /// The result true is returned ONLY if the supposed secret is MISSING!!!
    #[cfg(test)]
    pub fn password_missing(&self) -> bool {
        self.params.password_missing()
    }

    /// Set the secret to ft params for the default secret field for this protocol
    #[cfg(test)]
    pub fn set_default_secret(&mut self, secret: String) {
        self.params.set_default_secret(secret);
    }
}

impl Default for FileTransferParams {
    fn default() -> Self {
        Self::new(FileTransferProtocol::Sftp, ProtocolParams::default())
    }
}

impl Default for ProtocolParams {
    fn default() -> Self {
        Self::Generic(GenericProtocolParams::default())
    }
}

impl ProtocolParams {
    /// Retrieve generic parameters from protocol params if any
    pub fn generic_params(&self) -> Option<&GenericProtocolParams> {
        match self {
            ProtocolParams::Generic(params) => Some(params),
            _ => None,
        }
    }

    #[cfg(test)]
    /// Get a mutable reference to the inner generic protocol params
    pub fn mut_generic_params(&mut self) -> Option<&mut GenericProtocolParams> {
        match self {
            ProtocolParams::Generic(params) => Some(params),
            _ => None,
        }
    }

    #[cfg(test)]
    /// Retrieve AWS S3 parameters if any
    pub fn s3_params(&self) -> Option<&AwsS3Params> {
        match self {
            ProtocolParams::AwsS3(params) => Some(params),
            _ => None,
        }
    }

    #[cfg(test)]
    /// Retrieve Kube params parameters if any
    pub fn kube_params(&self) -> Option<&KubeProtocolParams> {
        match self {
            ProtocolParams::Kube(params) => Some(params),
            _ => None,
        }
    }

    #[cfg(test)]
    /// Retrieve SMB parameters if any
    pub fn smb_params(&self) -> Option<&SmbParams> {
        match self {
            ProtocolParams::Smb(params) => Some(params),
            _ => None,
        }
    }

    #[cfg(test)]
    /// Retrieve WebDAV parameters if any
    pub fn webdav_params(&self) -> Option<&WebDAVProtocolParams> {
        match self {
            ProtocolParams::WebDAV(params) => Some(params),
            _ => None,
        }
    }
}

// -- Generic protocol params

impl Default for GenericProtocolParams {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 22,
            username: None,
            password: None,
        }
    }
}

impl GenericProtocolParams {
    /// Set address to params
    pub fn address<S: AsRef<str>>(mut self, address: S) -> Self {
        self.address = address.as_ref().to_string();
        self
    }

    /// Set port to params
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set username for params
    pub fn username<S: AsRef<str>>(mut self, username: Option<S>) -> Self {
        self.username = username.map(|x| x.as_ref().to_string());
        self
    }

    /// Set password for params
    pub fn password<S: AsRef<str>>(mut self, password: Option<S>) -> Self {
        self.password = password.map(|x| x.as_ref().to_string());
        self
    }

    /// Returns whether a password is supposed to be required for this protocol params.
    /// The result true is returned ONLY if the supposed secret is MISSING!!!
    pub fn password_missing(&self) -> bool {
        self.password.is_none()
    }

    /// Set password
    pub fn set_default_secret(&mut self, secret: String) {
        self.password = Some(secret);
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_filetransfer_params() {
        let params: FileTransferParams =
            FileTransferParams::new(FileTransferProtocol::Scp, ProtocolParams::default())
                .remote_path(Some(&Path::new("/tmp")))
                .local_path(Some(&Path::new("/usr")));
        assert_eq!(
            params.params.generic_params().unwrap().address.as_str(),
            "localhost"
        );
        assert_eq!(params.protocol, FileTransferProtocol::Scp);
        assert_eq!(params.remote_path.as_deref().unwrap(), Path::new("/tmp"));
        assert_eq!(params.local_path.as_deref().unwrap(), Path::new("/usr"));
    }

    #[test]
    fn params_default() {
        let params: GenericProtocolParams = ProtocolParams::default()
            .generic_params()
            .unwrap()
            .to_owned();
        assert_eq!(params.address.as_str(), "localhost");
        assert_eq!(params.port, 22);
        assert!(params.username.is_none());
        assert!(params.password.is_none());
    }

    #[test]
    fn references() {
        let mut params =
            ProtocolParams::AwsS3(AwsS3Params::new("omar", Some("eu-west-1"), Some("test")));
        assert!(params.s3_params().is_some());
        assert!(params.generic_params().is_none());
        assert!(params.mut_generic_params().is_none());
        let mut params = ProtocolParams::default();
        assert!(params.s3_params().is_none());
        assert!(params.generic_params().is_some());
        assert!(params.mut_generic_params().is_some());
    }

    #[test]
    fn password_missing() {
        assert!(FileTransferParams::new(
            FileTransferProtocol::Scp,
            ProtocolParams::AwsS3(AwsS3Params::new("omar", Some("eu-west-1"), Some("test")))
        )
        .password_missing());
        assert_eq!(
            FileTransferParams::new(
                FileTransferProtocol::Scp,
                ProtocolParams::AwsS3(
                    AwsS3Params::new("omar", Some("eu-west-1"), Some("test"))
                        .secret_access_key(Some("test"))
                )
            )
            .password_missing(),
            false
        );
        assert_eq!(
            FileTransferParams::new(
                FileTransferProtocol::Scp,
                ProtocolParams::AwsS3(
                    AwsS3Params::new("omar", Some("eu-west-1"), Some("test"))
                        .security_token(Some("test"))
                )
            )
            .password_missing(),
            false
        );
        assert!(
            FileTransferParams::new(FileTransferProtocol::Scp, ProtocolParams::default())
                .password_missing()
        );
        assert_eq!(
            FileTransferParams::new(
                FileTransferProtocol::Scp,
                ProtocolParams::Generic(GenericProtocolParams::default().password(Some("Hello")))
            )
            .password_missing(),
            false
        );
    }

    #[test]
    fn set_default_secret_aws_s3() {
        let mut params = FileTransferParams::new(
            FileTransferProtocol::Scp,
            ProtocolParams::AwsS3(AwsS3Params::new("omar", Some("eu-west-1"), Some("test"))),
        );
        params.set_default_secret(String::from("secret"));
        assert_eq!(
            params
                .params
                .s3_params()
                .unwrap()
                .secret_access_key
                .as_deref()
                .unwrap(),
            "secret"
        );
    }

    #[test]
    #[cfg(unix)]
    fn set_default_secret_smb() {
        let mut params = FileTransferParams::new(
            FileTransferProtocol::Scp,
            ProtocolParams::Smb(SmbParams::new("localhost", "temp")),
        );
        params.set_default_secret(String::from("secret"));
        assert_eq!(
            params
                .params
                .smb_params()
                .unwrap()
                .password
                .as_deref()
                .unwrap(),
            "secret"
        );
    }

    #[test]
    fn set_default_secret_webdav() {
        let mut params = FileTransferParams::new(
            FileTransferProtocol::Scp,
            ProtocolParams::WebDAV(WebDAVProtocolParams {
                uri: "http://localhost".to_string(),
                username: "user".to_string(),
                password: "pass".to_string(),
            }),
        );
        params.set_default_secret(String::from("secret"));
        assert_eq!(params.params.webdav_params().unwrap().password, "secret");
    }

    #[test]
    fn set_default_secret_generic() {
        let mut params =
            FileTransferParams::new(FileTransferProtocol::Scp, ProtocolParams::default());
        params.set_default_secret(String::from("secret"));
        assert_eq!(
            params
                .params
                .generic_params()
                .unwrap()
                .password
                .as_deref()
                .unwrap(),
            "secret"
        );
    }
}
