//! ## Bookmarks
//!
//! `bookmarks` is the module which provides data types and de/serializer for bookmarks

mod aws_s3;
mod kube;
mod smb;

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use self::aws_s3::S3Params;
pub use self::kube::KubeParams;
pub use self::smb::SmbParams;
use crate::filetransfer::params::{
    AwsS3Params, GenericProtocolParams, KubeProtocolParams, ProtocolParams,
    SmbParams as TransferSmbParams, WebDAVProtocolParams,
};
use crate::filetransfer::{FileTransferParams, FileTransferProtocol};

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
    /// Remote folder to connect to (serde rename for legacy reasons)
    #[serde(rename = "directory")]
    pub remote_path: Option<PathBuf>,
    /// local folder to open at startup
    pub local_path: Option<PathBuf>,
    /// Kube params; optional. When used other fields are empty for sure
    pub kube: Option<KubeParams>,
    /// S3 params; optional. When used other fields are empty for sure
    pub s3: Option<S3Params>,
    /// SMB params; optional. Extra params required for SMB protocol
    pub smb: Option<SmbParams>,
}

// -- impls

impl From<FileTransferParams> for Bookmark {
    fn from(params: FileTransferParams) -> Self {
        let protocol = params.protocol;
        let remote_path = params.remote_path;
        let local_path = params.local_path;
        // Create generic or others
        match params.params {
            ProtocolParams::Generic(params) => Self {
                protocol,
                address: Some(params.address),
                port: Some(params.port),
                username: params.username,
                password: params.password,
                remote_path,
                local_path,
                kube: None,
                s3: None,
                smb: None,
            },
            ProtocolParams::AwsS3(params) => Self {
                protocol,
                address: None,
                port: None,
                username: None,
                password: None,
                remote_path,
                local_path,
                kube: None,
                s3: Some(S3Params::from(params)),
                smb: None,
            },
            ProtocolParams::Kube(params) => Self {
                protocol,
                address: None,
                port: None,
                username: None,
                password: None,
                remote_path,
                local_path,
                kube: Some(KubeParams::from(params)),
                s3: None,
                smb: None,
            },
            ProtocolParams::Smb(params) => Self {
                smb: Some(SmbParams::from(params.clone())),
                protocol,
                address: Some(params.address),
                #[cfg(unix)]
                port: Some(params.port),
                #[cfg(windows)]
                port: None,
                username: params.username,
                password: params.password,
                remote_path,
                local_path,
                kube: None,
                s3: None,
            },
            ProtocolParams::WebDAV(parms) => Self {
                protocol,
                address: Some(parms.uri),
                port: None,
                username: Some(parms.username),
                password: Some(parms.password),
                remote_path,
                local_path,
                kube: None,
                s3: None,
                smb: None,
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
            FileTransferProtocol::Kube => {
                let params = bookmark.kube.unwrap_or_default();
                let params = KubeProtocolParams::from(params);
                Self::new(bookmark.protocol, ProtocolParams::Kube(params))
            }
            #[cfg(unix)]
            FileTransferProtocol::Smb => {
                let params = TransferSmbParams::new(
                    bookmark.address.unwrap_or_default(),
                    bookmark.smb.clone().map(|x| x.share).unwrap_or_default(),
                )
                .port(bookmark.port.unwrap_or(445))
                .username(bookmark.username)
                .password(bookmark.password)
                .workgroup(bookmark.smb.and_then(|x| x.workgroup));

                Self::new(bookmark.protocol, ProtocolParams::Smb(params))
            }
            #[cfg(windows)]
            FileTransferProtocol::Smb => {
                let params = TransferSmbParams::new(
                    bookmark.address.unwrap_or_default(),
                    bookmark.smb.clone().map(|x| x.share).unwrap_or_default(),
                )
                .username(bookmark.username)
                .password(bookmark.password);

                Self::new(bookmark.protocol, ProtocolParams::Smb(params))
            }
            FileTransferProtocol::WebDAV => Self::new(
                FileTransferProtocol::WebDAV,
                ProtocolParams::WebDAV(WebDAVProtocolParams {
                    uri: bookmark.address.unwrap_or_default(),
                    username: bookmark.username.unwrap_or_default(),
                    password: bookmark.password.unwrap_or_default(),
                }),
            ),
        }
        .remote_path(bookmark.remote_path) // Set entry remote_path
        .local_path(bookmark.local_path) // Set entry local path
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

    use pretty_assertions::assert_eq;

    use super::*;

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
            remote_path: Some(PathBuf::from("/tmp")),
            local_path: Some(PathBuf::from("/usr")),
            kube: None,
            s3: None,
            smb: None,
        };
        let recent: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.2")),
            port: Some(22),
            protocol: FileTransferProtocol::Scp,
            username: Some(String::from("admin")),
            password: Some(String::from("password")),
            remote_path: Some(PathBuf::from("/home")),
            local_path: Some(PathBuf::from("/usr")),
            kube: None,
            s3: None,
            smb: None,
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
            bookmark.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        assert_eq!(
            bookmark.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
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
            bookmark.remote_path.as_deref().unwrap(),
            std::path::Path::new("/home")
        );
        assert_eq!(
            bookmark.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
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
            .remote_path(Some(PathBuf::from("/home")))
            .local_path(Some(PathBuf::from("/tmp")));
        let bookmark = Bookmark::from(params);
        assert_eq!(bookmark.protocol, FileTransferProtocol::Scp);
        assert_eq!(bookmark.address.as_deref().unwrap(), "127.0.0.1");
        assert_eq!(bookmark.port.unwrap(), 10222);
        assert_eq!(bookmark.username.as_deref().unwrap(), "root");
        assert_eq!(bookmark.password.as_deref().unwrap(), "omar");
        assert_eq!(
            bookmark.remote_path.as_deref().unwrap(),
            std::path::Path::new("/home")
        );
        assert_eq!(
            bookmark.local_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
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
    fn bookmark_from_kube_ftparams() {
        let params = ProtocolParams::Kube(KubeProtocolParams {
            pod: "pod".to_string(),
            container: "container".to_string(),
            namespace: Some("default".to_string()),
            username: Some("root".to_string()),
            cluster_url: Some("https://localhost:6443".to_string()),
            client_cert: Some("cert".to_string()),
            client_key: Some("key".to_string()),
        });
        let params: FileTransferParams =
            FileTransferParams::new(FileTransferProtocol::Kube, params);
        let bookmark = Bookmark::from(params);
        assert_eq!(bookmark.protocol, FileTransferProtocol::Kube);
        assert!(bookmark.address.is_none());
        assert!(bookmark.port.is_none());
        assert!(bookmark.username.is_none());
        assert!(bookmark.password.is_none());
        let kube: &KubeParams = bookmark.kube.as_ref().unwrap();
        assert_eq!(kube.pod_name.as_str(), "pod");
        assert_eq!(kube.container.as_str(), "container");
        assert_eq!(kube.namespace.as_deref().unwrap(), "default");
        assert_eq!(
            kube.cluster_url.as_deref().unwrap(),
            "https://localhost:6443"
        );
        assert_eq!(kube.username.as_deref().unwrap(), "root");
        assert_eq!(kube.client_cert.as_deref().unwrap(), "cert");
        assert_eq!(kube.client_key.as_deref().unwrap(), "key");
    }

    #[test]
    fn ftparams_from_generic_bookmark() {
        let bookmark: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.1")),
            port: Some(22),
            protocol: FileTransferProtocol::Sftp,
            username: Some(String::from("root")),
            password: Some(String::from("password")),
            remote_path: Some(PathBuf::from("/tmp")),
            local_path: Some(PathBuf::from("/usr")),
            kube: None,
            s3: None,
            smb: None,
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::Sftp);
        assert_eq!(
            params.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        assert_eq!(
            params.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
        );
        let gparams = params.params.generic_params().unwrap();
        assert_eq!(gparams.address.as_str(), "192.168.1.1");
        assert_eq!(gparams.port, 22);
        assert_eq!(gparams.username.as_deref().unwrap(), "root");
        assert_eq!(gparams.password.as_deref().unwrap(), "password");
    }

    #[test]
    fn ftparams_from_webdav() {
        let bookmark: Bookmark = Bookmark {
            address: Some(String::from("192.168.1.1")),
            port: None,
            protocol: FileTransferProtocol::WebDAV,
            username: Some(String::from("root")),
            password: Some(String::from("password")),
            remote_path: Some(PathBuf::from("/tmp")),
            local_path: Some(PathBuf::from("/usr")),
            kube: None,
            s3: None,
            smb: None,
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::WebDAV);
        assert_eq!(
            params.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        assert_eq!(
            params.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
        );
        let gparams = params.params.webdav_params().unwrap();
        assert_eq!(gparams.uri.as_str(), "192.168.1.1");
        assert_eq!(gparams.username, "root");
        assert_eq!(gparams.password, "password");
    }

    #[test]
    fn ftparams_from_s3_bookmark() {
        let bookmark: Bookmark = Bookmark {
            protocol: FileTransferProtocol::AwsS3,
            address: None,
            port: None,
            username: None,
            password: None,
            remote_path: Some(PathBuf::from("/tmp")),
            local_path: Some(PathBuf::from("/usr")),
            kube: None,
            s3: Some(S3Params {
                bucket: String::from("veeso"),
                region: Some(String::from("eu-west-1")),
                endpoint: Some(String::from("omar")),
                profile: Some(String::from("default")),
                access_key: Some(String::from("pippo")),
                secret_access_key: Some(String::from("pluto")),
                new_path_style: Some(true),
            }),
            smb: None,
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::AwsS3);
        assert_eq!(
            params.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        assert_eq!(
            params.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
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

    #[test]
    fn ftparams_from_kube_bookmark() {
        let bookmark: Bookmark = Bookmark {
            protocol: FileTransferProtocol::Kube,
            address: None,
            port: None,
            username: None,
            password: None,
            remote_path: Some(PathBuf::from("/tmp")),
            local_path: Some(PathBuf::from("/usr")),
            kube: Some(KubeParams {
                pod_name: String::from("pod"),
                container: String::from("container"),
                namespace: Some(String::from("default")),
                cluster_url: Some(String::from("https://localhost:6443")),
                username: Some(String::from("root")),
                client_cert: Some(String::from("cert")),
                client_key: Some(String::from("key")),
            }),
            s3: None,
            smb: None,
        };
        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::Kube);
        assert_eq!(
            params.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        assert_eq!(
            params.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
        );
        let gparams = params.params.kube_params().unwrap();
        assert_eq!(gparams.pod.as_str(), "pod");
        assert_eq!(gparams.namespace.as_deref().unwrap(), "default");
        assert_eq!(
            gparams.cluster_url.as_deref().unwrap(),
            "https://localhost:6443"
        );
        assert_eq!(gparams.username.as_deref().unwrap(), "root");
        assert_eq!(gparams.client_cert.as_deref().unwrap(), "cert");
        assert_eq!(gparams.client_key.as_deref().unwrap(), "key");
    }

    #[test]
    #[cfg(unix)]
    fn should_get_ftparams_from_smb_bookmark() {
        let bookmark: Bookmark = Bookmark {
            protocol: FileTransferProtocol::Smb,
            address: Some("localhost".to_string()),
            port: Some(445),
            username: Some("foo".to_string()),
            password: Some("bar".to_string()),
            remote_path: Some(PathBuf::from("/tmp")),
            local_path: Some(PathBuf::from("/usr")),
            kube: None,
            s3: None,
            smb: Some(SmbParams {
                share: "test".to_string(),
                workgroup: Some("testone".to_string()),
            }),
        };

        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::Smb);
        assert_eq!(
            params.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        assert_eq!(
            params.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
        );
        let smb_params = params.params.smb_params().unwrap();
        assert_eq!(smb_params.address.as_str(), "localhost");
        assert_eq!(smb_params.port, 445);
        assert_eq!(smb_params.share.as_str(), "test");
        assert_eq!(smb_params.password.as_deref().unwrap(), "bar");
        assert_eq!(smb_params.username.as_deref().unwrap(), "foo");
        assert_eq!(smb_params.workgroup.as_deref().unwrap(), "testone");
    }

    #[test]
    #[cfg(windows)]
    fn should_get_ftparams_from_smb_bookmark() {
        let bookmark: Bookmark = Bookmark {
            protocol: FileTransferProtocol::Smb,
            address: Some("localhost".to_string()),
            port: Some(445),
            username: None,
            password: None,
            remote_path: Some(PathBuf::from("/tmp")),
            local_path: Some(PathBuf::from("/usr")),
            s3: None,
            kube: None,
            smb: Some(SmbParams {
                share: "test".to_string(),
                workgroup: None,
            }),
        };

        let params = FileTransferParams::from(bookmark);
        assert_eq!(params.protocol, FileTransferProtocol::Smb);
        assert_eq!(
            params.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
        assert_eq!(
            params.local_path.as_deref().unwrap(),
            std::path::Path::new("/usr")
        );
        let smb_params = params.params.smb_params().unwrap();
        assert_eq!(smb_params.address.as_str(), "localhost");
        assert_eq!(smb_params.share.as_str(), "test");
    }
}
