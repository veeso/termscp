//! ## Bookmarks
//!
//! `bookmarks` is the module which provides data types and de/serializer for bookmarks

use crate::filetransfer::params::{AwsS3Params, GenericProtocolParams, ProtocolParams};
use crate::filetransfer::{FileTransferParams, FileTransferProtocol};

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

/// UserHosts contains all the hosts saved by the user in the data storage
/// It contains both `Bookmark`
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserHosts {
    pub bookmarks: HashMap<String, Bookmark>,
    pub recents: HashMap<String, Bookmark>,
}

/// Bookmark describes a single bookmark entry in the user hosts storage
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Bookmark {
    #[serde(
        deserialize_with = "deserialize_protocol",
        serialize_with = "serialize_protocol"
    )]
    pub protocol: FileTransferProtocol,
    /// Address for generic parameters
    pub address: Option<String>,
    /// Port number for generic parameters
    pub port: Option<u16>,
    /// Username for generic parameters
    pub username: Option<String>,
    /// Password is optional; base64, aes-128 encrypted password
    pub password: Option<String>,
    /// Remote folder to connect to
    pub directory: Option<PathBuf>,
    /// S3 params; optional. When used other fields are empty for sure
    pub s3: Option<S3Params>,
}

/// Connection parameters for Aws s3 protocol
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Default)]
pub struct S3Params {
    pub bucket: String,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    /// NOTE: there are no session token and security token since they are always temporary
    pub new_path_style: Option<bool>,
}

// -- impls

impl From<FileTransferParams> for Bookmark {
    fn from(params: FileTransferParams) -> Self {
        let protocol = params.protocol;
        let directory = params.entry_directory;
        // Create generic or others
        match params.params {
            ProtocolParams::Generic(params) => Self {
                protocol,
                address: Some(params.address),
                port: Some(params.port),
                username: params.username,
                password: params.password,
                directory,
                s3: None,
            },
            ProtocolParams::AwsS3(params) => Self {
                protocol,
                address: None,
                port: None,
                username: None,
                password: None,
                directory,
                s3: Some(S3Params::from(params)),
            },
        }
    }
}

impl From<Bookmark> for FileTransferParams {
    fn from(bookmark: Bookmark) -> Self {
        // Create generic or others based on protocol
        match bookmark.protocol {
            FileTransferProtocol::AwsS3 => {
                let params = bookmark.s3.unwrap_or_default();
                let params = AwsS3Params::from(params);
                Self::new(FileTransferProtocol::AwsS3, ProtocolParams::AwsS3(params))
            }
            FileTransferProtocol::Ftp(_)
            | FileTransferProtocol::Scp
            | FileTransferProtocol::Sftp => {
                let params = GenericProtocolParams::default()
                    .address(bookmark.address.unwrap_or_default())
                    .port(bookmark.port.unwrap_or(22))
                    .username(bookmark.username)
                    .password(bookmark.password);
                Self::new(bookmark.protocol, ProtocolParams::Generic(params))
            }
        }
        .entry_directory(bookmark.directory) // Set entry directory
    }
}

impl From<AwsS3Params> for S3Params {
    fn from(params: AwsS3Params) -> Self {
        S3Params {
            bucket: params.bucket_name,
            region: params.region,
            endpoint: params.endpoint,
            profile: params.profile,
            access_key: params.access_key,
            secret_access_key: params.secret_access_key,
            new_path_style: Some(params.new_path_style),
        }
    }
}

impl From<S3Params> for AwsS3Params {
    fn from(params: S3Params) -> Self {
        AwsS3Params::new(params.bucket, params.region, params.profile)
            .endpoint(params.endpoint)
            .access_key(params.access_key)
            .secret_access_key(params.secret_access_key)
            .new_path_style(params.new_path_style.unwrap_or(false))
    }
}

fn deserialize_protocol<'de, D>(deserializer: D) -> Result<FileTransferProtocol, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // Parse color
    match FileTransferProtocol::from_str(&s) {
        Err(err) => Err(DeError::custom(err)),
        Ok(protocol) => Ok(protocol),
    }
}

fn serialize_protocol<S>(protocol: &FileTransferProtocol, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(protocol.to_string().as_str())
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_bookmarks_default() {
        let bookmarks: UserHosts = UserHosts::default();
        assert_eq!(bookmarks.bookmarks.len(), 0);
        assert_eq!(bookmarks.recents.len(), 0);
    }

    #[test]
    fn test_bookmarks_bookmark_new() {
        let bookmark: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.1")),
            port: Some(22),
            protocol: FileTransferProtocol::Sftp,
            username: Some(String::from("root")),
            password: Some(String::from("password")),
            directory: Some(PathBuf::from("/tmp")),
            s3: None,
        };
        let recent: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.2")),
            port: Some(22),
            protocol: FileTransferProtocol::Scp,
            username: Some(String::from("admin")),
            password: Some(String::from("password")),
            directory: Some(PathBuf::from("/home")),
            s3: None,
        };
        let mut bookmarks: HashMap<String, Bookmark> = HashMap::with_capacity(1);
        bookmarks.insert(String::from("test"), bookmark);
        let mut recents: HashMap<String, Bookmark> = HashMap::with_capacity(1);
        recents.insert(String::from("ISO20201218T181432"), recent);
        let hosts: UserHosts = UserHosts { bookmarks, recents };
        // Verify
        let bookmark: &Bookmark = hosts.bookmarks.get(&String::from("test")).unwrap();
        assert_eq!(bookmark.address.as_deref().unwrap(), "192.168.1.1");
        assert_eq!(bookmark.port.unwrap(), 22);
        assert_eq!(bookmark.protocol, FileTransferProtocol::Sftp);
        assert_eq!(bookmark.username.as_deref().unwrap(), "root");
        assert_eq!(bookmark.password.as_deref().unwrap(), "password");
        assert_eq!(
            bookmark.directory.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        let bookmark: &Bookmark = hosts
            .recents
            .get(&String::from("ISO20201218T181432"))
            .unwrap();
        assert_eq!(bookmark.address.as_deref().unwrap(), "192.168.1.2");
        assert_eq!(bookmark.port.unwrap(), 22);
        assert_eq!(bookmark.protocol, FileTransferProtocol::Scp);
        assert_eq!(bookmark.username.as_deref().unwrap(), "admin");
        assert_eq!(bookmark.password.as_deref().unwrap(), "password");
        assert_eq!(
            bookmark.directory.as_deref().unwrap(),
            std::path::Path::new("/home")
        );
    }

    #[test]
    fn bookmark_from_generic_ftparams() {
        let params = ProtocolParams::Generic(GenericProtocolParams {
            address: "127.0.0.1".to_string(),
            port: 10222,
            username: Some(String::from("root")),
            password: Some(String::from("omar")),
        });
        let params: FileTransferParams = FileTransferParams::new(FileTransferProtocol::Scp, params)
            .entry_directory(Some(PathBuf::from("/home")));
        let bookmark = Bookmark::from(params);
        assert_eq!(bookmark.protocol, FileTransferProtocol::Scp);
        assert_eq!(bookmark.address.as_deref().unwrap(), "127.0.0.1");
        assert_eq!(bookmark.port.unwrap(), 10222);
        assert_eq!(bookmark.username.as_deref().unwrap(), "root");
        assert_eq!(bookmark.password.as_deref().unwrap(), "omar");
        assert_eq!(
            bookmark.directory.as_deref().unwrap(),
            std::path::Path::new("/home")
        );
        assert!(bookmark.s3.is_none());
    }

    #[test]
    fn bookmark_from_s3_ftparams() {
        let params = ProtocolParams::AwsS3(
            AwsS3Params::new("omar", Some("eu-west-1"), Some("test"))
                .access_key(Some("pippo"))
                .secret_access_key(Some("pluto")),
        );
        let params: FileTransferParams =
            FileTransferParams::new(FileTransferProtocol::AwsS3, params);
        let bookmark = Bookmark::from(params);
        assert_eq!(bookmark.protocol, FileTransferProtocol::AwsS3);
        assert!(bookmark.address.is_none());
        assert!(bookmark.port.is_none());
        assert!(bookmark.username.is_none());
        assert!(bookmark.password.is_none());
        let s3: &S3Params = bookmark.s3.as_ref().unwrap();
        assert_eq!(s3.bucket.as_str(), "omar");
        assert_eq!(s3.region.as_deref().unwrap(), "eu-west-1");
        assert_eq!(s3.profile.as_deref().unwrap(), "test");
        assert_eq!(s3.access_key.as_deref().unwrap(), "pippo");
        assert_eq!(s3.secret_access_key.as_deref().unwrap(), "pluto");
    }

    #[test]
    fn ftparams_from_generic_bookmark() {
        let bookmark: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.1")),
            port: Some(22),
            protocol: FileTransferProtocol::Sftp,
            username: Some(String::from("root")),
            password: Some(String::from("password")),
            directory: Some(PathBuf::from("/tmp")),
            s3: None,
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::Sftp);
        assert_eq!(
            params.entry_directory.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        let gparams = params.params.generic_params().unwrap();
        assert_eq!(gparams.address.as_str(), "192.168.1.1");
        assert_eq!(gparams.port, 22);
        assert_eq!(gparams.username.as_deref().unwrap(), "root");
        assert_eq!(gparams.password.as_deref().unwrap(), "password");
    }

    #[test]
    fn ftparams_from_s3_bookmark() {
        let bookmark: Bookmark = Bookmark {
            protocol: FileTransferProtocol::AwsS3,
            address: None,
            port: None,
            username: None,
            password: None,
            directory: Some(PathBuf::from("/tmp")),
            s3: Some(S3Params {
                bucket: String::from("veeso"),
                region: Some(String::from("eu-west-1")),
                endpoint: Some(String::from("omar")),
                profile: Some(String::from("default")),
                access_key: Some(String::from("pippo")),
                secret_access_key: Some(String::from("pluto")),
                new_path_style: Some(true),
            }),
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::AwsS3);
        assert_eq!(
            params.entry_directory.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        let gparams = params.params.s3_params().unwrap();
        assert_eq!(gparams.bucket_name.as_str(), "veeso");
        assert_eq!(gparams.region.as_deref().unwrap(), "eu-west-1");
        assert_eq!(gparams.endpoint.as_deref().unwrap(), "omar");
        assert_eq!(gparams.profile.as_deref().unwrap(), "default");
        assert_eq!(gparams.access_key.as_deref().unwrap(), "pippo");
        assert_eq!(gparams.secret_access_key.as_deref().unwrap(), "pluto");
        assert_eq!(gparams.new_path_style, true);
    }
}
