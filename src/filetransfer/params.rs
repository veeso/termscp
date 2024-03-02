//! ## Params
//!
//! file transfer parameters

use std::path::{Path, PathBuf};

use super::FileTransferProtocol;

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
    Smb(SmbParams),
    WebDAV(WebDAVProtocolParams),
}

/// Protocol params used by most common protocols
#[derive(Debug, Clone)]
pub struct GenericProtocolParams {
    pub address: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Connection parameters for AWS S3 protocol
#[derive(Debug, Clone)]
pub struct AwsS3Params {
    pub bucket_name: String,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    pub security_token: Option<String>,
    pub session_token: Option<String>,
    pub new_path_style: bool,
}

/// Connection parameters for SMB protocol
#[derive(Debug, Clone)]
pub struct SmbParams {
    pub address: String,
    #[cfg(unix)]
    pub port: u16,
    pub share: String,
    pub username: Option<String>,
    pub password: Option<String>,
    #[cfg(unix)]
    pub workgroup: Option<String>,
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
    pub fn password_missing(&self) -> bool {
        match &self.params {
            ProtocolParams::AwsS3(params) => params.password_missing(),
            ProtocolParams::Generic(params) => params.password_missing(),
            ProtocolParams::Smb(params) => params.password_missing(),
            ProtocolParams::WebDAV(params) => params.password_missing(),
        }
    }

    /// Set the secret to ft params for the default secret field for this protocol
    pub fn set_default_secret(&mut self, secret: String) {
        match &mut self.params {
            ProtocolParams::AwsS3(params) => params.set_default_secret(secret),
            ProtocolParams::Generic(params) => params.set_default_secret(secret),
            ProtocolParams::Smb(params) => params.set_default_secret(secret),
            ProtocolParams::WebDAV(params) => params.set_default_secret(secret),
        }
    }
}

/// Protocol params used by WebDAV
#[derive(Debug, Clone)]
pub struct WebDAVProtocolParams {
    pub uri: String,
    pub username: String,
    pub password: String,
}

impl WebDAVProtocolParams {
    fn set_default_secret(&mut self, secret: String) {
        self.password = secret;
    }

    fn password_missing(&self) -> bool {
        self.password.is_empty()
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

// -- S3 params

impl AwsS3Params {
    /// Instantiates a new `AwsS3Params` struct
    pub fn new<S: AsRef<str>>(bucket: S, region: Option<S>, profile: Option<S>) -> Self {
        Self {
            bucket_name: bucket.as_ref().to_string(),
            region: region.map(|x| x.as_ref().to_string()),
            profile: profile.map(|x| x.as_ref().to_string()),
            endpoint: None,
            access_key: None,
            secret_access_key: None,
            security_token: None,
            session_token: None,
            new_path_style: false,
        }
    }

    /// Construct aws s3 params with specified endpoint
    pub fn endpoint<S: AsRef<str>>(mut self, endpoint: Option<S>) -> Self {
        self.endpoint = endpoint.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided access key
    pub fn access_key<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.access_key = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided secret_access_key
    pub fn secret_access_key<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.secret_access_key = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided security_token
    pub fn security_token<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.security_token = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided session_token
    pub fn session_token<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.session_token = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Specify new path style when constructing aws s3 params
    pub fn new_path_style(mut self, new_path_style: bool) -> Self {
        self.new_path_style = new_path_style;
        self
    }

    /// Returns whether a password is supposed to be required for this protocol params.
    /// The result true is returned ONLY if the supposed secret is MISSING!!!
    pub fn password_missing(&self) -> bool {
        self.secret_access_key.is_none() && self.security_token.is_none()
    }

    /// Set password
    pub fn set_default_secret(&mut self, secret: String) {
        self.secret_access_key = Some(secret);
    }
}

// -- SMB params

impl SmbParams {
    /// Instantiates a new `AwsS3Params` struct
    pub fn new<S: AsRef<str>>(address: S, share: S) -> Self {
        Self {
            address: address.as_ref().to_string(),
            #[cfg(unix)]
            port: 445,
            share: share.as_ref().to_string(),
            username: None,
            password: None,
            #[cfg(unix)]
            workgroup: None,
        }
    }

    #[cfg(unix)]
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn username(mut self, username: Option<impl ToString>) -> Self {
        self.username = username.map(|x| x.to_string());
        self
    }

    pub fn password(mut self, password: Option<impl ToString>) -> Self {
        self.password = password.map(|x| x.to_string());
        self
    }

    #[cfg(unix)]
    pub fn workgroup(mut self, workgroup: Option<impl ToString>) -> Self {
        self.workgroup = workgroup.map(|x| x.to_string());
        self
    }

    /// Returns whether a password is supposed to be required for this protocol params.
    /// The result true is returned ONLY if the supposed secret is MISSING!!!
    pub fn password_missing(&self) -> bool {
        self.password.is_none()
    }

    /// Set password
    #[cfg(unix)]
    pub fn set_default_secret(&mut self, secret: String) {
        self.password = Some(secret);
    }

    #[cfg(windows)]
    pub fn set_default_secret(&mut self, _secret: String) {}
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
    fn should_init_aws_s3_params() {
        let params: AwsS3Params = AwsS3Params::new("omar", Some("eu-west-1"), Some("test"));
        assert_eq!(params.bucket_name.as_str(), "omar");
        assert_eq!(params.region.as_deref().unwrap(), "eu-west-1");
        assert_eq!(params.profile.as_deref().unwrap(), "test");
        assert!(params.endpoint.is_none());
        assert!(params.access_key.is_none());
        assert!(params.secret_access_key.is_none());
        assert!(params.security_token.is_none());
        assert!(params.session_token.is_none());
        assert_eq!(params.new_path_style, false);
    }

    #[test]
    fn should_init_aws_s3_params_with_optionals() {
        let params: AwsS3Params = AwsS3Params::new("omar", Some("eu-west-1"), Some("test"))
            .endpoint(Some("http://omar.it"))
            .access_key(Some("pippo"))
            .secret_access_key(Some("pluto"))
            .security_token(Some("omar"))
            .session_token(Some("gerry-scotti"))
            .new_path_style(true);
        assert_eq!(params.bucket_name.as_str(), "omar");
        assert_eq!(params.region.as_deref().unwrap(), "eu-west-1");
        assert_eq!(params.profile.as_deref().unwrap(), "test");
        assert_eq!(params.endpoint.as_deref().unwrap(), "http://omar.it");
        assert_eq!(params.access_key.as_deref().unwrap(), "pippo");
        assert_eq!(params.secret_access_key.as_deref().unwrap(), "pluto");
        assert_eq!(params.security_token.as_deref().unwrap(), "omar");
        assert_eq!(params.session_token.as_deref().unwrap(), "gerry-scotti");
        assert_eq!(params.new_path_style, true);
    }

    #[test]
    fn should_init_smb_params() {
        let params = SmbParams::new("localhost", "temp");
        assert_eq!(&params.address, "localhost");

        #[cfg(unix)]
        assert_eq!(params.port, 445);
        assert_eq!(&params.share, "temp");

        #[cfg(unix)]
        assert!(params.username.is_none());
        #[cfg(unix)]
        assert!(params.password.is_none());
        #[cfg(unix)]
        assert!(params.workgroup.is_none());
    }

    #[test]
    #[cfg(unix)]
    fn should_init_smb_params_with_optionals() {
        let params = SmbParams::new("localhost", "temp")
            .port(3456)
            .username(Some("foo"))
            .password(Some("bar"))
            .workgroup(Some("baz"));

        assert_eq!(&params.address, "localhost");
        assert_eq!(params.port, 3456);
        assert_eq!(&params.share, "temp");
        assert_eq!(params.username.as_deref().unwrap(), "foo");
        assert_eq!(params.password.as_deref().unwrap(), "bar");
        assert_eq!(params.workgroup.as_deref().unwrap(), "baz");
    }

    #[test]
    #[cfg(windows)]
    fn should_init_smb_params_with_optionals() {
        let params = SmbParams::new("localhost", "temp")
            .username(Some("foo"))
            .password(Some("bar"));

        assert_eq!(&params.address, "localhost");
        assert_eq!(&params.share, "temp");
        assert_eq!(params.username.as_deref().unwrap(), "foo");
        assert_eq!(params.password.as_deref().unwrap(), "bar");
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
    #[cfg(linux)]
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
