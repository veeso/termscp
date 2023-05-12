//! ## Parser
//!
//! `parser` is the module which provides utilities for parsing different kind of stuff

// Locals
use std::path::PathBuf;
use std::str::FromStr;

// Ext
use bytesize::ByteSize;
use lazy_regex::{Lazy, Regex};
use tuirealm::tui::style::Color;
use tuirealm::utils::parser as tuirealm_parser;

use crate::filetransfer::params::{AwsS3Params, GenericProtocolParams, ProtocolParams};
use crate::filetransfer::{FileTransferParams, FileTransferProtocol};
#[cfg(not(test))] // NOTE: don't use configuration during tests
use crate::system::config_client::ConfigClient;
#[cfg(not(test))] // NOTE: don't use configuration during tests
use crate::system::environment;

// Regex

/**
 * This regex matches the protocol used as option
 * Regex matches:
 * - group 1: Some(protocol) | None
 * - group 2: SMB windows prefix
 * - group 3: Some(other args)
 */
static REMOTE_OPT_PROTOCOL_REGEX: Lazy<Regex> = lazy_regex!(r"(?:([a-z0-9]+)://)?(\\)?(?:(.+))");

/**
 * Regex matches:
 *  - group 1: Some(user) | None
 *  - group 2: Address
 *  - group 3: Some(port) | None
 *  - group 4: Some(path) | None
 */
static REMOTE_GENERIC_OPT_REGEX: Lazy<Regex> = lazy_regex!(
    r"(?:([^@]+)@)?(?:([^:]+))(?::((?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])(?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])))?(?::([^:]+))?"
);

/**
 * Regex matches:
 * - group 1: Bucket
 * - group 2: Region
 * - group 3: Some(profile) | None
 * - group 4: Some(path) | None
 */
static REMOTE_S3_OPT_REGEX: Lazy<Regex> =
    lazy_regex!(r"(?:([^@]+)@)(?:([^:]+))(?::([a-zA-Z0-9][^:]+))?(?::([^:]+))?");

/**
 * Regex matches:
 * - group 1: username
 * - group 2: address
 * - group 3: port?
 * - group 4: share?
 * - group 5: remote-dir?
 */
#[cfg(unix)]
static REMOTE_SMB_OPT_REGEX: Lazy<Regex> = lazy_regex!(
    r"(?:([^@]+)@)?(?:([^/:]+))(?::((?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])(?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])))?(?:/([^/]+))?(?:(/.+))?"
);

/**
 * Regex matches:
 * - group 1: address
 * - group 2: port?
 * - group 3: share
 * - group 4: remote-dir?
 */
#[cfg(windows)]
static REMOTE_SMB_OPT_REGEX: Lazy<Regex> = lazy_regex!(
    r"(?::((?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])(?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])))?(?:\\([^\\]+))?(?:(\\.+))?"
);

/**
 * Regex matches:
 * - group 1: Version
 * E.g. termscp-0.3.2 => 0.3.2
 *      v0.4.0 => 0.4.0
 */
static SEMVER_REGEX: Lazy<Regex> = lazy_regex!(r".*(:?[0-9]\.[0-9]\.[0-9])");

/**
 * Regex matches:
 * - group 1: amount (number)
 * - group 4: unit (K, M, G, T, P)
 */
static BYTESIZE_REGEX: Lazy<Regex> = lazy_regex!(r"(:?([0-9])+)( )*(:?[KMGTP])?B$");

// -- remote opts

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
    // Set protocol to default protocol
    #[cfg(not(test))] // NOTE: don't use configuration during tests
    let default_protocol: FileTransferProtocol = match environment::init_config_dir() {
        Ok(p) => match p {
            Some(p) => {
                // Create config client
                let (config_path, ssh_key_path) = environment::get_config_paths(p.as_path());
                match ConfigClient::new(config_path.as_path(), ssh_key_path.as_path()) {
                    Ok(cli) => cli.get_default_protocol(),
                    Err(_) => FileTransferProtocol::Sftp,
                }
            }
            None => FileTransferProtocol::Sftp,
        },
        Err(_) => FileTransferProtocol::Sftp,
    };
    #[cfg(test)] // NOTE: during test set protocol just to Sftp
    let default_protocol: FileTransferProtocol = FileTransferProtocol::Sftp;
    // Get protocol
    let (protocol, s): (FileTransferProtocol, String) =
        parse_remote_opt_protocol(s, default_protocol)?;
    // Match against regex for protocol type
    match protocol {
        FileTransferProtocol::AwsS3 => parse_s3_remote_opt(s.as_str()),
        FileTransferProtocol::Smb => parse_smb_remote_opts(s.as_str()),
        protocol => parse_generic_remote_opt(s.as_str(), protocol),
    }
}

/// Parse protocol from CLI option. In case of success, return the protocol to be used and the remaining arguments
fn parse_remote_opt_protocol(
    s: &str,
    default: FileTransferProtocol,
) -> Result<(FileTransferProtocol, String), String> {
    match REMOTE_OPT_PROTOCOL_REGEX.captures(s) {
        Some(groups) => {
            // Parse protocol or use default
            let protocol = groups.get(1).map(|x| {
                FileTransferProtocol::from_str(x.as_str())
                    .map_err(|_| format!("Unknown protocol \"{}\"", x.as_str()))
            });
            let protocol = match protocol {
                Some(Ok(protocol)) => protocol,
                Some(Err(err)) => return Err(err),
                #[cfg(windows)]
                None if groups.get(2).is_some() => FileTransferProtocol::Smb,
                None => default,
            };
            // Return protocol and remaining arguments
            Ok((
                protocol,
                groups
                    .get(3)
                    .map(|x| x.as_str().to_string())
                    .unwrap_or_default(),
            ))
        }
        None => Err("Invalid args".to_string()),
    }
}

/// Parse generic remote options
fn parse_generic_remote_opt(
    s: &str,
    protocol: FileTransferProtocol,
) -> Result<FileTransferParams, String> {
    match REMOTE_GENERIC_OPT_REGEX.captures(s) {
        Some(groups) => {
            // Match user
            let username: Option<String> = match groups.get(1) {
                Some(group) => Some(group.as_str().to_string()),
                None => match protocol {
                    // If group is empty, set to current user
                    FileTransferProtocol::Scp | FileTransferProtocol::Sftp => {
                        Some(whoami::username())
                    }
                    _ => None,
                },
            };
            // Get address
            let address: String = match groups.get(2) {
                Some(group) => group.as_str().to_string(),
                None => return Err(String::from("Missing address")),
            };
            // Get port
            let port: u16 = match groups.get(3) {
                Some(port) => match port.as_str().parse::<u16>() {
                    // Try to parse port
                    Ok(p) => p,
                    Err(err) => return Err(format!("Bad port \"{}\": {}", port.as_str(), err)),
                },
                None => match protocol {
                    // Set port based on protocol
                    FileTransferProtocol::Ftp(_) => 21,
                    FileTransferProtocol::Scp => 22,
                    FileTransferProtocol::Sftp => 22,
                    _ => 22, // Doesn't matter
                },
            };
            // Get workdir
            let entry_directory: Option<PathBuf> =
                groups.get(4).map(|group| PathBuf::from(group.as_str()));
            let params: ProtocolParams = ProtocolParams::Generic(
                GenericProtocolParams::default()
                    .address(address)
                    .port(port)
                    .username(username),
            );
            Ok(FileTransferParams::new(protocol, params).entry_directory(entry_directory))
        }
        None => Err(String::from("Bad remote host syntax!")),
    }
}

/// Parse remote options for s3 protocol
fn parse_s3_remote_opt(s: &str) -> Result<FileTransferParams, String> {
    match REMOTE_S3_OPT_REGEX.captures(s) {
        Some(groups) => {
            let bucket: String = groups
                .get(1)
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
            let region: String = groups
                .get(2)
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
            let profile: Option<String> = groups.get(3).map(|x| x.as_str().to_string());
            let entry_directory: Option<PathBuf> =
                groups.get(4).map(|group| PathBuf::from(group.as_str()));
            Ok(FileTransferParams::new(
                FileTransferProtocol::AwsS3,
                ProtocolParams::AwsS3(AwsS3Params::new(bucket, Some(region), profile)),
            )
            .entry_directory(entry_directory))
        }
        None => Err(String::from("Bad remote host syntax!")),
    }
}

/// Parse remote options for smb protocol
#[cfg(unix)]
fn parse_smb_remote_opts(s: &str) -> Result<FileTransferParams, String> {
    use crate::filetransfer::params::SmbParams;

    match REMOTE_SMB_OPT_REGEX.captures(s) {
        Some(groups) => {
            let username: Option<String> = match groups.get(1) {
                Some(group) => Some(group.as_str().to_string()),
                None => Some(whoami::username()),
            };
            let address = match groups.get(2) {
                Some(group) => group.as_str().to_string(),
                None => return Err(String::from("Missing address")),
            };
            let port = match groups.get(3) {
                Some(port) => match port.as_str().parse::<u16>() {
                    // Try to parse port
                    Ok(p) => p,
                    Err(err) => return Err(format!("Bad port \"{}\": {}", port.as_str(), err)),
                },
                None => 445,
            };
            let share = match groups.get(4) {
                Some(group) => group.as_str().to_string(),
                None => return Err(String::from("Missing address")),
            };
            let entry_directory: Option<PathBuf> =
                groups.get(5).map(|group| PathBuf::from(group.as_str()));

            Ok(FileTransferParams::new(
                FileTransferProtocol::Smb,
                ProtocolParams::Smb(SmbParams::new(address, port, share).username(username)),
            )
            .entry_directory(entry_directory))
        }
        None => Err(String::from("Bad remote host syntax!")),
    }
}

#[cfg(windows)]
fn parse_smb_remote_opts(s: &str) -> Result<FileTransferParams, String> {
    use crate::filetransfer::params::SmbParams;

    match REMOTE_SMB_OPT_REGEX.captures(s) {
        Some(groups) => {
            let address = match groups.get(1) {
                Some(group) => group.as_str().to_string(),
                None => return Err(String::from("Missing address")),
            };
            let port = match groups.get(2) {
                Some(port) => match port.as_str().parse::<u16>() {
                    // Try to parse port
                    Ok(p) => p,
                    Err(err) => return Err(format!("Bad port \"{}\": {}", port.as_str(), err)),
                },
                None => 445,
            };
            let share = match groups.get(3) {
                Some(group) => group.as_str().to_string(),
                None => return Err(String::from("Missing address")),
            };
            let entry_directory: Option<PathBuf> =
                groups.get(4).map(|group| PathBuf::from(group.as_str()));

            Ok(FileTransferParams::new(
                FileTransferProtocol::Smb,
                ProtocolParams::Smb(SmbParams::new(address, port, share)),
            )
            .entry_directory(entry_directory))
        }
        None => Err(String::from("Bad remote host syntax!")),
    }
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
            let unit = ByteUnit::from_str(unit.as_str()).unwrap();
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

    use pretty_assertions::assert_eq;

    use super::*;

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
        assert!(params.username.is_some());
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
        assert!(result.entry_directory.is_none());
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
        assert!(result.entry_directory.is_none());
        // Port only
        let result: FileTransferParams = parse_remote_opt(&String::from("172.26.104.1:4022"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 4022);
        assert!(params.username.is_some());
        assert!(result.entry_directory.is_none());
        // Protocol
        let result: FileTransferParams = parse_remote_opt(&String::from("ftp://172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Ftp(false));
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 21); // Fallback to ftp default
        assert!(params.username.is_none()); // Doesn't fall back
        assert!(result.entry_directory.is_none());
        // Protocol
        let result: FileTransferParams = parse_remote_opt(&String::from("sftp://172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22); // Fallback to sftp default
        assert!(params.username.is_some()); // Doesn't fall back
        assert!(result.entry_directory.is_none());
        let result: FileTransferParams = parse_remote_opt(&String::from("scp://172.26.104.1"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Scp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22); // Fallback to scp default
        assert!(params.username.is_some()); // Doesn't fall back
        assert!(result.entry_directory.is_none());
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
        assert!(result.entry_directory.is_none());
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
        assert_eq!(result.entry_directory.unwrap(), PathBuf::from("/var"));
        // Port only
        let result: FileTransferParams = parse_remote_opt(&String::from("172.26.104.1:home"))
            .ok()
            .unwrap();
        let params = result.params.generic_params().unwrap();
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(params.address, String::from("172.26.104.1"));
        assert_eq!(params.port, 22);
        assert!(params.username.is_some());
        assert_eq!(result.entry_directory.unwrap(), PathBuf::from("home"));
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
        assert_eq!(result.entry_directory.unwrap(), PathBuf::from("/tmp"));
        // bad syntax
        // Bad protocol
        assert!(parse_remote_opt(&String::from("omar://172.26.104.1")).is_err());
        // Bad port
        assert!(parse_remote_opt(&String::from("scp://172.26.104.1:650000")).is_err());
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
        assert_eq!(result.entry_directory, None);
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
        assert_eq!(result.entry_directory, None);
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
        assert_eq!(result.entry_directory, Some(PathBuf::from("/foobar")));
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
        assert_eq!(result.entry_directory, Some(PathBuf::from("/foobar")));
        assert_eq!(params.bucket_name.as_str(), "mybucket");
        assert_eq!(params.region.as_deref().unwrap(), "eu-central-1");
        assert_eq!(params.profile.as_deref(), Some("default"));
        // -- bad args
        assert!(parse_remote_opt(&String::from("s3://mybucket:default:/foobar")).is_err());
    }

    #[test]
    #[cfg(unix)]
    fn should_parse_smb_address() {
        let result = parse_remote_opt("smb://myserver/myshare").ok().unwrap();
        let params = result.params.smb_params().unwrap();

        assert_eq!(params.address.as_str(), "myserver");
        assert_eq!(params.port, 445);
        assert_eq!(params.share.as_str(), "myshare");
        assert!(params.username.is_some());
        assert!(params.password.is_none());
        assert!(params.workgroup.is_none());
        assert!(result.entry_directory.is_none());
    }

    #[test]
    #[cfg(unix)]
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
            result.entry_directory.as_deref().unwrap(),
            std::path::Path::new("/dir/subdir")
        );
    }

    #[test]
    #[cfg(windows)]
    fn should_parse_smb_address() {
        let result = parse_remote_opt(&String::from("\\\\myserver\\myshare"))
            .ok()
            .unwrap();
        let params = result.params.smb_params().unwrap();

        assert_eq!(params.address.as_str(), "myserver");
        assert_eq!(params.port, 445);
        assert_eq!(params.share.as_str(), "myshare");
        assert!(result.entry_directory.is_none());
    }

    #[test]
    #[cfg(windows)]
    fn should_parse_smb_address_with_opts() {
        let result = parse_remote_opt(&String::from("\\\\myserver:3445\\myshare\\path"))
            .ok()
            .unwrap();
        let params = result.params.smb_params().unwrap();

        assert_eq!(params.address.as_str(), "myserver");
        assert_eq!(params.port, 3445);
        assert_eq!(params.share.as_str(), "myshare");
        assert_eq!(
            result.entry_directory.as_deref().unwrap(),
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
        assert!(parse_bytesize("1 GB aaaaa").is_none());
        assert!(parse_bytesize("1 GBaaaaa").is_none());
        assert!(parse_bytesize("1MBaaaaa").is_none());
    }
}
