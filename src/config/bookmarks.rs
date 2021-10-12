//! ## Bookmarks
//!
//! `bookmarks` is the module which provides data types and de/serializer for bookmarks

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
use crate::filetransfer::params::{AwsS3Params, GenericProtocolParams, ProtocolParams};
use crate::filetransfer::{FileTransferParams, FileTransferProtocol};

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::str::FromStr;

/// ## UserHosts
///
/// UserHosts contains all the hosts saved by the user in the data storage
/// It contains both `Bookmark`
#[derive(Deserialize, Serialize, Debug)]
pub struct UserHosts {
    pub bookmarks: HashMap<String, Bookmark>,
    pub recents: HashMap<String, Bookmark>,
}

/// ## Bookmark
///
/// Bookmark describes a single bookmark entry in the user hosts storage
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
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
    /// S3 params; optional. When used other fields are empty for sure
    pub s3: Option<S3Params>,
}

/// ## S3Params
///
/// Connection parameters for Aws s3 protocol
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct S3Params {
    pub bucket: String,
    pub region: String,
    pub profile: Option<String>,
}

// -- impls

impl Default for UserHosts {
    fn default() -> Self {
        Self {
            bookmarks: HashMap::new(),
            recents: HashMap::new(),
        }
    }
}

impl From<FileTransferParams> for Bookmark {
    fn from(params: FileTransferParams) -> Self {
        let protocol: FileTransferProtocol = params.protocol;
        // Create generic or others
        match params.params {
            ProtocolParams::Generic(params) => Self {
                protocol,
                address: Some(params.address),
                port: Some(params.port),
                username: params.username,
                password: params.password,
                s3: None,
            },
            ProtocolParams::AwsS3(params) => Self {
                protocol,
                address: None,
                port: None,
                username: None,
                password: None,
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
            protocol => {
                let params = GenericProtocolParams::default()
                    .address(bookmark.address.unwrap_or_default())
                    .port(bookmark.port.unwrap_or(22))
                    .username(bookmark.username)
                    .password(bookmark.password);
                Self::new(protocol, ProtocolParams::Generic(params))
            }
        }
    }
}

impl From<AwsS3Params> for S3Params {
    fn from(params: AwsS3Params) -> Self {
        S3Params {
            bucket: params.bucket_name,
            region: params.region,
            profile: params.profile,
        }
    }
}

impl From<S3Params> for AwsS3Params {
    fn from(params: S3Params) -> Self {
        AwsS3Params::new(params.bucket, params.region, params.profile)
    }
}

fn deserialize_protocol<'de, D>(deserializer: D) -> Result<FileTransferProtocol, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    // Parse color
    match FileTransferProtocol::from_str(s) {
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
            s3: None,
        };
        let recent: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.2")),
            port: Some(22),
            protocol: FileTransferProtocol::Scp,
            username: Some(String::from("admin")),
            password: Some(String::from("password")),
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
        let bookmark: &Bookmark = hosts
            .recents
            .get(&String::from("ISO20201218T181432"))
            .unwrap();
        assert_eq!(bookmark.address.as_deref().unwrap(), "192.168.1.2");
        assert_eq!(bookmark.port.unwrap(), 22);
        assert_eq!(bookmark.protocol, FileTransferProtocol::Scp);
        assert_eq!(bookmark.username.as_deref().unwrap(), "admin");
        assert_eq!(bookmark.password.as_deref().unwrap(), "password");
    }

    #[test]
    fn bookmark_from_generic_ftparams() {
        let params = ProtocolParams::Generic(GenericProtocolParams {
            address: "127.0.0.1".to_string(),
            port: 10222,
            username: Some(String::from("root")),
            password: Some(String::from("omar")),
        });
        let params: FileTransferParams = FileTransferParams::new(FileTransferProtocol::Scp, params);
        let bookmark = Bookmark::from(params);
        assert_eq!(bookmark.protocol, FileTransferProtocol::Scp);
        assert_eq!(bookmark.address.as_deref().unwrap(), "127.0.0.1");
        assert_eq!(bookmark.port.unwrap(), 10222);
        assert_eq!(bookmark.username.as_deref().unwrap(), "root");
        assert_eq!(bookmark.password.as_deref().unwrap(), "omar");
        assert!(bookmark.s3.is_none());
    }

    #[test]
    fn bookmark_from_s3_ftparams() {
        let params = ProtocolParams::AwsS3(AwsS3Params::new("omar", "eu-west-1", Some("test")));
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
        assert_eq!(s3.region.as_str(), "eu-west-1");
        assert_eq!(s3.profile.as_deref().unwrap(), "test");
    }

    #[test]
    fn ftparams_from_generic_bookmark() {
        let bookmark: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.1")),
            port: Some(22),
            protocol: FileTransferProtocol::Sftp,
            username: Some(String::from("root")),
            password: Some(String::from("password")),
            s3: None,
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::Sftp);
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
            s3: Some(S3Params {
                bucket: String::from("veeso"),
                region: String::from("eu-west-1"),
                profile: None,
            }),
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::AwsS3);
        let gparams = params.params.s3_params().unwrap();
        assert_eq!(gparams.bucket_name.as_str(), "veeso");
        assert_eq!(gparams.region.as_str(), "eu-west-1");
        assert_eq!(gparams.profile, None);
    }
}
