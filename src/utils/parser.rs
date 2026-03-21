//! ## Parser
//!
//! `parser` is the module which provides utilities for parsing different kind of stuff

use std::str::FromStr;

use bytesize::ByteSize;
use lazy_regex::{Lazy, Regex};
use tuirealm::ratatui::style::Color;
use tuirealm::utils::parser as tuirealm_parser;

use crate::filetransfer::FileTransferParams;
#[path = "parser/credentials.rs"]
mod credentials;
#[path = "parser/ports.rs"]
mod ports;
#[path = "parser/protocol.rs"]
mod protocol;
#[path = "parser/remote.rs"]
mod remote;

/// This regex matches the protocol used as option.
pub(super) static REMOTE_OPT_PROTOCOL_REGEX: Lazy<Regex> =
    lazy_regex!(r"(?:([a-z0-9]+)://)?(\\\\)?(?:(.+))");

/// Regex matches generic remote options.
pub(super) static REMOTE_GENERIC_OPT_REGEX: Lazy<Regex> = lazy_regex!(
    r"(?:(.+[^@])@)?(?:([^:]+))(?::((?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])(?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])))?(?::([^:]+))?"
);

/// Regex matches WebDAV remote options.
pub(super) static REMOTE_WEBDAV_OPT_REGEX: Lazy<Regex> =
    lazy_regex!(r"(?:([^:]+):)(?:(.+[^@])@)(?:([^/]+))(?:(.+))?");

/// Regex matches kube remote options.
pub(super) static REMOTE_KUBE_OPT_REGEX: Lazy<Regex> =
    lazy_regex!(r"(?:([^@]+))(@(?:([^$]+)))?(\$(?:(.+)))?");

/// Regex matches s3 remote options.
pub(super) static REMOTE_S3_OPT_REGEX: Lazy<Regex> =
    lazy_regex!(r"(?:(.+[^@])@)(?:([^:]+))(?::([a-zA-Z0-9][^:]+))?(?::([^:]+))?");

/// Regex matches SMB remote options on Unix platforms.
#[cfg(smb_unix)]
pub(super) static REMOTE_SMB_OPT_REGEX: Lazy<Regex> = lazy_regex!(
    r"(?:(.+[^@])@)?(?:([^/:]+))(?::((?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])(?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])))?(?:/([^/]+))?(?:(/.+))?"
);

/// Regex matches SMB remote options on Windows.
#[cfg(smb_windows)]
pub(super) static REMOTE_SMB_OPT_REGEX: Lazy<Regex> =
    lazy_regex!(r"(?:(.+[^@])@)?(?:([^:\\]+))(?:\\([^\\]+))?(?:(\\.+))?");

/// Regex matches semantic versions.
static SEMVER_REGEX: Lazy<Regex> = lazy_regex!(r"v?((0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*))");

/**
 * Regex matches:
 * - group 1: amount (number)
 * - group 4: unit (K, M, G, T, P)
 */
static BYTESIZE_REGEX: Lazy<Regex> = lazy_regex!(r"(:?([0-9])+)( )*(:?[KMGTP])?B$");

/// Parse remote option string. Returns in case of success a RemoteOptions struct
/// For ssh if username is not provided, current user will be used.
/// In case of error, message is returned
/// If port is missing default port will be used for each protocol
///     SFTP => 22
///     FTP => 21
/// The option string has the following syntax
/// [protocol://][username@]{address}[:port][:path]
/// The only argument which is mandatory is address
/// NOTE: possible strings
/// - 172.26.104.1
/// - root@172.26.104.1
/// - sftp://root@172.26.104.1
/// - sftp://172.26.104.1:4022
/// - sftp://172.26.104.1
/// - ...
///
/// For s3:
///
/// s3://<bucket-name>@<region>[:profile][:/wrkdir]
///
/// For SMB:
///
/// on UNIX derived (macos, linux, ...)
///
/// smb://[username@]<address>[:port]/<share>[/path]
///
/// on Windows
///
/// \\<address>\<share>[\path]
///
pub fn parse_remote_opt(s: &str) -> Result<FileTransferParams, String> {
    remote::parse_remote_opt(s)
}

/// Parse semver string
pub fn parse_semver(haystack: &str) -> Option<String> {
    match SEMVER_REGEX.captures(haystack) {
        Some(groups) => groups.get(1).map(|version| version.as_str().to_string()),
        None => None,
    }
}

/// Parse color from string into a `Color` enum.
///
/// Color may be in different format:
///
/// 1. color name:
///     - Black,
///     - Blue,
///     - Cyan,
///     - DarkGray,
///     - Gray,
///     - Green,
///     - LightBlue,
///     - LightCyan,
///     - LightGreen,
///     - LightMagenta,
///     - LightRed,
///     - LightYellow,
///     - Magenta,
///     - Red,
///     - Reset,
///     - White,
///     - Yellow,
/// 2. Hex format:
///     - #f0ab05
///     - #AA33BC
/// 3. Rgb format:
///     - rgb(255, 64, 32)
///     - rgb(255,64,32)
///     - 255, 64, 32
pub fn parse_color(color: &str) -> Option<Color> {
    tuirealm_parser::parse_color(color)
}

#[derive(Debug, PartialEq)]
enum ByteUnit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Petabyte,
}

impl FromStr for ByteUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Self::Byte),
            "KB" => Ok(Self::Kilobyte),
            "MB" => Ok(Self::Megabyte),
            "GB" => Ok(Self::Gigabyte),
            "TB" => Ok(Self::Terabyte),
            "PB" => Ok(Self::Petabyte),
            _ => Err("Invalid unit"),
        }
    }
}

/// Parse bytes repr (e.g. `24 MB`) into `ByteSize`
pub fn parse_bytesize<S: AsRef<str>>(bytes: S) -> Option<ByteSize> {
    match BYTESIZE_REGEX.captures(bytes.as_ref()) {
        None => None,
        Some(groups) => {
            let amount = groups
                .get(1)
                .map(|x| x.as_str().parse::<u64>().unwrap_or(0))?;
            let unit = groups.get(4).map(|x| x.as_str().to_string());
            let unit = format!("{}B", unit.unwrap_or_default());
            let unit = ByteUnit::from_str(unit.as_str()).ok()?;
            Some(match unit {
                ByteUnit::Byte => ByteSize::b(amount),
                ByteUnit::Gigabyte => ByteSize::gib(amount),
                ByteUnit::Kilobyte => ByteSize::kib(amount),
                ByteUnit::Megabyte => ByteSize::mib(amount),
                ByteUnit::Petabyte => ByteSize::pib(amount),
                ByteUnit::Terabyte => ByteSize::tib(amount),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::filetransfer::FileTransferProtocol;

    #[test]
    fn test_utils_parse_remote_opt() {
        // Base case
        let result: FileTransferParams = parse_remote_opt(&String::from("172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22);
        assert!(params.username.is_none());
        // User case
        let result: FileTransferParams = parse_remote_opt(&String::from("root@172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22);
        assert_eq!(
            params.username.as_deref().unwrap().to_string(),
            String::from("root")
        );
        assert!(result.remote_path.is_none());
        // User + port
        let result: FileTransferParams = parse_remote_opt(&String::from("root@172.26.104.1:8022"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 8022);
        assert_eq!(
            params.username.as_deref().unwrap().to_string(),
            String::from("root")
        );
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert!(result.remote_path.is_none());
        // Port only
        let result: FileTransferParams = parse_remote_opt(&String::from("172.26.104.1:4022"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 4022);
        assert!(params.username.is_none());
        assert!(result.remote_path.is_none());
        // Protocol
        let result: FileTransferParams = parse_remote_opt(&String::from("ftp://172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Ftp(false));
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 21); // Fallback to ftp default
        assert!(params.username.is_none()); // Doesn't fall back
        assert!(result.remote_path.is_none());
        // Protocol
        let result: FileTransferParams = parse_remote_opt(&String::from("sftp://172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22); // Fallback to sftp default
        assert!(params.username.is_none()); // Doesn't fall back
        assert!(result.remote_path.is_none());
        let result: FileTransferParams = parse_remote_opt(&String::from("scp://172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Scp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22); // Fallback to scp default
        assert!(params.username.is_none()); // Doesn't fall back
        assert!(result.remote_path.is_none());
        // Protocol + user
        let result: FileTransferParams =
            parse_remote_opt(&String::from("ftps://anon@172.26.104.1"))
                .ok()
                .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Ftp(true));
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 21); // Fallback to ftp default
        assert_eq!(
            params.username.as_deref().unwrap().to_string(),
            String::from("anon")
        );
        assert!(result.remote_path.is_none());
        // Path
        let result: FileTransferParams =
            parse_remote_opt(&String::from("root@172.26.104.1:8022:/var"))
                .ok()
                .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 8022);
        assert_eq!(
            params.username.as_deref().unwrap().to_string(),
            String::from("root")
        );
        assert_eq!(result.remote_path.unwrap(), PathBuf::from("/var"));
        // Port only
        let result: FileTransferParams = parse_remote_opt(&String::from("172.26.104.1:home"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22);
        assert!(params.username.is_none());
        assert_eq!(result.remote_path.unwrap(), PathBuf::from("home"));
        // All together now
        let result: FileTransferParams =
            parse_remote_opt(&String::from("ftp://anon@172.26.104.1:8021:/tmp"))
                .ok()
                .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Ftp(false));
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 8021); // Fallback to ftp default
        assert_eq!(
            params.username.as_deref().unwrap().to_string(),
            String::from("anon")
        );
        assert_eq!(result.remote_path.unwrap(), PathBuf::from("/tmp"));
        // bad syntax
        // Bad protocol
        assert!(parse_remote_opt(&String::from("omar://172.26.104.1")).is_err());
        // Bad port
        assert!(parse_remote_opt(&String::from("scp://172.26.104.1:650000")).is_err());

        // with @ in username
        let result: FileTransferParams =
            parse_remote_opt(&String::from("dummy@veeso.dev@172.26.104.1:8022"))
                .ok()
                .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 8022);
        assert_eq!(
            params.username.as_deref().unwrap().to_string(),
            String::from("dummy@veeso.dev")
        );
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert!(result.remote_path.is_none());
    }

    #[test]
    fn test_should_parse_webdav_opt() {
        let result =
            parse_remote_opt("https://omar:password@myserver:4445/myshare/dir/subdir").unwrap();

        let params = result.params.webdav_params().unwrap();
        assert_eq!(params.uri.as_str(), "https://myserver:4445");
        assert_eq!(params.username.as_str(), "omar");
        assert_eq!(params.password.as_str(), "password");

        let result =
            parse_remote_opt("http://omar:password@myserver:4445/myshare/dir/subdir").unwrap();

        let params = result.params.webdav_params().unwrap();
        assert_eq!(params.uri.as_str(), "http://myserver:4445");
        assert_eq!(params.username.as_str(), "omar");
        assert_eq!(params.password.as_str(), "password");

        // remote path
        assert_eq!(
            result.remote_path.unwrap(),
            PathBuf::from("/myshare/dir/subdir")
        );
    }

    #[test]
    fn test_should_parse_webdav_opt_with_at() {
        let result =
            parse_remote_opt("https://omar@veeso.dev:password@myserver:4445/myshare/dir/subdir")
                .unwrap();

        let params = result.params.webdav_params().unwrap();
        assert_eq!(params.uri.as_str(), "https://myserver:4445");
        assert_eq!(params.username.as_str(), "omar@veeso.dev");
        assert_eq!(params.password.as_str(), "password");
    }

    #[test]
    fn should_reject_malformed_webdav_options() {
        let result = parse_remote_opt("https://omar@myserver:4445/myshare");

        assert!(result.is_err());
    }

    #[test]
    fn parse_aws_s3_opt() {
        // Simple
        let result: FileTransferParams =
            parse_remote_opt(&String::from("s3://mybucket@eu-central-1"))
                .ok()
                .unwrap();
        let params = result.params.s3_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::AwsS3);
        assert_eq!(result.remote_path, None);
        assert_eq!(params.bucket_name.as_str(), "mybucket");
        assert_eq!(params.region.as_deref().unwrap(), "eu-central-1");
        assert_eq!(params.profile, None);
        // With profile
        let result: FileTransferParams =
            parse_remote_opt(&String::from("s3://mybucket@eu-central-1:default"))
                .ok()
                .unwrap();
        let params = result.params.s3_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::AwsS3);
        assert_eq!(result.remote_path, None);
        assert_eq!(params.bucket_name.as_str(), "mybucket");
        assert_eq!(params.region.as_deref().unwrap(), "eu-central-1");
        assert_eq!(params.profile.as_deref(), Some("default"));
        // With wrkdir only
        let result: FileTransferParams =
            parse_remote_opt(&String::from("s3://mybucket@eu-central-1:/foobar"))
                .ok()
                .unwrap();
        let params = result.params.s3_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::AwsS3);
        assert_eq!(result.remote_path, Some(PathBuf::from("/foobar")));
        assert_eq!(params.bucket_name.as_str(), "mybucket");
        assert_eq!(params.region.as_deref().unwrap(), "eu-central-1");
        assert_eq!(params.profile, None);
        // With all arguments
        let result: FileTransferParams =
            parse_remote_opt(&String::from("s3://mybucket@eu-central-1:default:/foobar"))
                .ok()
                .unwrap();
        let params = result.params.s3_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::AwsS3);
        assert_eq!(result.remote_path, Some(PathBuf::from("/foobar")));
        assert_eq!(params.bucket_name.as_str(), "mybucket");
        assert_eq!(params.region.as_deref().unwrap(), "eu-central-1");
        assert_eq!(params.profile.as_deref(), Some("default"));
        // -- bad args
        assert!(parse_remote_opt(&String::from("s3://mybucket:default:/foobar")).is_err());

        // with @
        let result: FileTransferParams = parse_remote_opt(&String::from(
            "s3://omar@mybucket@eu-central-1:default:/foobar",
        ))
        .ok()
        .unwrap();
        let params = result.params.s3_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::AwsS3);
        assert_eq!(result.remote_path, Some(PathBuf::from("/foobar")));
        assert_eq!(params.bucket_name.as_str(), "omar@mybucket");
        assert_eq!(params.region.as_deref().unwrap(), "eu-central-1");
    }

    #[test]
    fn should_parse_kube_address() {
        let result = parse_remote_opt("kube://my-namespace@http://localhost:1234$/tmp")
            .ok()
            .unwrap();
        let params = result.params.kube_params().unwrap();

        assert_eq!(params.namespace, Some("my-namespace".to_string()));
        assert_eq!(params.cluster_url.as_deref(), Some("http://localhost:1234"));
        assert_eq!(params.username, None);
        assert_eq!(params.client_cert, None);
        assert_eq!(params.client_key, None);
        assert_eq!(
            result.remote_path.as_deref().unwrap(),
            std::path::Path::new("/tmp")
        );
    }

    #[test]
    #[cfg(smb_unix)]
    fn should_parse_smb_address() {
        let result = parse_remote_opt("smb://myserver/myshare").ok().unwrap();
        let params = result.params.smb_params().unwrap();

        assert_eq!(params.address.as_str(), "myserver");
        assert_eq!(params.port, 445);
        assert_eq!(params.share.as_str(), "myshare");
        assert!(params.username.is_some());
        assert!(params.password.is_none());
        assert!(params.workgroup.is_none());
        assert!(result.remote_path.is_none());
    }

    #[test]
    #[cfg(smb_unix)]
    fn should_parse_smb_address_with_opts() {
        let result = parse_remote_opt("smb://omar@myserver:4445/myshare/dir/subdir")
            .ok()
            .unwrap();
        let params = result.params.smb_params().unwrap();

        assert_eq!(params.address.as_str(), "myserver");
        assert_eq!(params.port, 4445);
        assert_eq!(params.username.as_deref().unwrap(), "omar");
        assert!(params.password.is_none());
        assert!(params.workgroup.is_none());
        assert_eq!(params.share.as_str(), "myshare");
        assert_eq!(
            result.remote_path.as_deref().unwrap(),
            std::path::Path::new("/dir/subdir")
        );
    }

    #[test]
    #[cfg(smb_windows)]
    fn should_parse_smb_address() {
        let result = parse_remote_opt(&String::from("\\\\myserver\\myshare"))
            .ok()
            .unwrap();
        let params = result.params.smb_params().unwrap();

        assert_eq!(params.address.as_str(), "myserver");
        assert_eq!(params.share.as_str(), "myshare");
        assert!(result.remote_path.is_none());
    }

    #[test]
    #[cfg(smb_windows)]
    fn should_parse_smb_address_with_opts() {
        let result = parse_remote_opt(&String::from("\\\\omar@myserver\\myshare\\path"))
            .ok()
            .unwrap();
        let params = result.params.smb_params().unwrap();

        assert_eq!(params.address.as_str(), "myserver");
        assert_eq!(params.share.as_str(), "myshare");
        assert_eq!(params.username.as_deref().unwrap(), "omar");
        assert_eq!(
            result.remote_path.as_deref().unwrap(),
            std::path::Path::new("\\path")
        );
    }

    #[test]
    fn test_utils_parse_semver() {
        assert_eq!(
            parse_semver("termscp-0.3.2").unwrap(),
            String::from("0.3.2")
        );
        assert_eq!(parse_semver("v0.4.1").unwrap(), String::from("0.4.1"),);
        assert_eq!(parse_semver("1.0.0").unwrap(), String::from("1.0.0"),);
        assert!(parse_semver("v1.1").is_none());

        assert_eq!(parse_semver("10.15.10"), Some("10.15.10".to_string()));
    }

    #[test]
    fn test_utils_parse_color() {
        assert_eq!(parse_color("Black").unwrap(), Color::Black);
        assert_eq!(parse_color("#f0f0f0").unwrap(), Color::Rgb(240, 240, 240));
        // -- css colors
        assert_eq!(parse_color("aliceblue"), Some(Color::Rgb(240, 248, 255)));
        // -- hex and rgb
        assert_eq!(
            parse_color("rgb(255, 64, 32)").unwrap(),
            Color::Rgb(255, 64, 32)
        );
        // bad
        assert!(parse_color("redd").is_none());
    }

    #[test]
    fn parse_byteunit() {
        assert_eq!(ByteUnit::from_str("B").ok().unwrap(), ByteUnit::Byte);
        assert_eq!(ByteUnit::from_str("KB").ok().unwrap(), ByteUnit::Kilobyte);
        assert_eq!(ByteUnit::from_str("MB").ok().unwrap(), ByteUnit::Megabyte);
        assert_eq!(ByteUnit::from_str("GB").ok().unwrap(), ByteUnit::Gigabyte);
        assert_eq!(ByteUnit::from_str("TB").ok().unwrap(), ByteUnit::Terabyte);
        assert_eq!(ByteUnit::from_str("PB").ok().unwrap(), ByteUnit::Petabyte);
        assert!(ByteUnit::from_str("uB").is_err());
    }

    #[test]
    fn parse_str_as_bytesize() {
        assert_eq!(parse_bytesize("1024 B").unwrap().as_u64(), 1024);
        assert_eq!(parse_bytesize("1024B").unwrap().as_u64(), 1024);
        assert_eq!(parse_bytesize("10240 KB").unwrap().as_u64(), 10485760);
        assert_eq!(parse_bytesize("2 GB").unwrap().as_u64(), 2147483648);
        assert_eq!(parse_bytesize("1 TB").unwrap().as_u64(), 1099511627776);
        assert!(parse_bytesize("1 XB").is_none());
        assert!(parse_bytesize("10 YB").is_none());
        assert!(parse_bytesize("1 GB aaaaa").is_none());
        assert!(parse_bytesize("1 GBaaaaa").is_none());
        assert!(parse_bytesize("1MBaaaaa").is_none());
    }
}
