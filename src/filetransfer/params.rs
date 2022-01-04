//! ## Params
//!
//! file transfer parameters

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::FileTransferProtocol;

use std::path::{Path, PathBuf};

/// ### FileTransferParams
///
/// Holds connection parameters for file transfers
#[derive(Debug, Clone)]
pub struct FileTransferParams {
    pub protocol: FileTransferProtocol,
    pub params: ProtocolParams,
    pub entry_directory: Option<PathBuf>,
}

/// Container for protocol params
#[derive(Debug, Clone)]
pub enum ProtocolParams {
    Generic(GenericProtocolParams),
    AwsS3(AwsS3Params),
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
    pub region: String,
    pub profile: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    pub security_token: Option<String>,
    pub session_token: Option<String>,
}

impl FileTransferParams {
    /// Instantiates a new `FileTransferParams`
    pub fn new(protocol: FileTransferProtocol, params: ProtocolParams) -> Self {
        Self {
            protocol,
            params,
            entry_directory: None,
        }
    }

    /// Set entry directory
    pub fn entry_directory<P: AsRef<Path>>(mut self, dir: Option<P>) -> Self {
        self.entry_directory = dir.map(|x| x.as_ref().to_path_buf());
        self
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
    #[cfg(test)]
    /// Retrieve generic parameters from protocol params if any
    pub fn generic_params(&self) -> Option<&GenericProtocolParams> {
        match self {
            ProtocolParams::Generic(params) => Some(params),
            _ => None,
        }
    }

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
}

// -- S3 params

impl AwsS3Params {
    /// Instantiates a new `AwsS3Params` struct
    pub fn new<S: AsRef<str>>(bucket: S, region: S, profile: Option<S>) -> Self {
        Self {
            bucket_name: bucket.as_ref().to_string(),
            region: region.as_ref().to_string(),
            profile: profile.map(|x| x.as_ref().to_string()),
            access_key: None,
            secret_access_key: None,
            security_token: None,
            session_token: None,
        }
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
}

#[cfg(test)]
mod test {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_filetransfer_params() {
        let params: FileTransferParams =
            FileTransferParams::new(FileTransferProtocol::Scp, ProtocolParams::default())
                .entry_directory(Some(&Path::new("/tmp")));
        assert_eq!(
            params.params.generic_params().unwrap().address.as_str(),
            "localhost"
        );
        assert_eq!(params.protocol, FileTransferProtocol::Scp);
        assert_eq!(
            params.entry_directory.as_deref().unwrap(),
            Path::new("/tmp")
        );
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
        let params: AwsS3Params = AwsS3Params::new("omar", "eu-west-1", Some("test"));
        assert_eq!(params.bucket_name.as_str(), "omar");
        assert_eq!(params.region.as_str(), "eu-west-1");
        assert_eq!(params.profile.as_deref().unwrap(), "test");
        assert!(params.access_key.is_none());
        assert!(params.secret_access_key.is_none());
        assert!(params.security_token.is_none());
        assert!(params.session_token.is_none());
    }

    #[test]
    fn should_init_aws_s3_params_with_optionals() {
        let params: AwsS3Params = AwsS3Params::new("omar", "eu-west-1", Some("test"))
            .access_key(Some("pippo"))
            .secret_access_key(Some("pluto"))
            .security_token(Some("omar"))
            .session_token(Some("gerry-scotti"));
        assert_eq!(params.bucket_name.as_str(), "omar");
        assert_eq!(params.region.as_str(), "eu-west-1");
        assert_eq!(params.profile.as_deref().unwrap(), "test");
        assert_eq!(params.access_key.as_deref().unwrap(), "pippo");
        assert_eq!(params.secret_access_key.as_deref().unwrap(), "pluto");
        assert_eq!(params.security_token.as_deref().unwrap(), "omar");
        assert_eq!(params.session_token.as_deref().unwrap(), "gerry-scotti");
    }

    #[test]
    fn references() {
        let mut params = ProtocolParams::AwsS3(AwsS3Params::new("omar", "eu-west-1", Some("test")));
        assert!(params.s3_params().is_some());
        assert!(params.generic_params().is_none());
        assert!(params.mut_generic_params().is_none());
        let mut params = ProtocolParams::default();
        assert!(params.s3_params().is_none());
        assert!(params.generic_params().is_some());
        assert!(params.mut_generic_params().is_some());
    }
}
