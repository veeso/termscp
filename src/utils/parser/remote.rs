use std::path::PathBuf;

#[cfg(smb)]
use super::REMOTE_SMB_OPT_REGEX;
use super::credentials::{optional_capture, required_capture};
use super::ports::{default_port_for_protocol, parse_port};
use super::protocol::parse_remote_opt_protocol;
use super::{
    REMOTE_GENERIC_OPT_REGEX, REMOTE_KUBE_OPT_REGEX, REMOTE_S3_OPT_REGEX, REMOTE_WEBDAV_OPT_REGEX,
};
#[cfg(smb)]
use crate::filetransfer::params::SmbParams;
use crate::filetransfer::params::{
    AwsS3Params, GenericProtocolParams, KubeProtocolParams, ProtocolParams, WebDAVProtocolParams,
};
use crate::filetransfer::{FileTransferParams, FileTransferProtocol};
#[cfg(not(test))]
use crate::system::config_client::ConfigClient;
#[cfg(not(test))]
use crate::system::environment;

pub(super) fn parse_remote_opt(s: &str) -> Result<FileTransferParams, String> {
    let default_protocol = default_protocol();
    let (protocol, remote) = parse_remote_opt_protocol(s, default_protocol)?;

    match protocol {
        FileTransferProtocol::AwsS3 => parse_s3_remote_opt(remote.as_str()),
        FileTransferProtocol::Kube => parse_kube_remote_opt(remote.as_str()),
        #[cfg(smb)]
        FileTransferProtocol::Smb => parse_smb_remote_opts(remote.as_str()),
        FileTransferProtocol::WebDAV => {
            let prefix = if s.starts_with("https") {
                "https"
            } else {
                "http"
            };

            parse_webdav_remote_opt(remote.as_str(), prefix)
        }
        protocol => parse_generic_remote_opt(remote.as_str(), protocol),
    }
}

#[cfg(not(test))]
fn default_protocol() -> FileTransferProtocol {
    match environment::init_config_dir() {
        Ok(Some(path)) => {
            let (config_path, ssh_key_path) = environment::get_config_paths(path.as_path());
            match ConfigClient::new(config_path.as_path(), ssh_key_path.as_path()) {
                Ok(config) => config.get_default_protocol(),
                Err(_) => FileTransferProtocol::Sftp,
            }
        }
        Ok(None) | Err(_) => FileTransferProtocol::Sftp,
    }
}

#[cfg(test)]
fn default_protocol() -> FileTransferProtocol {
    FileTransferProtocol::Sftp
}

fn parse_generic_remote_opt(
    s: &str,
    protocol: FileTransferProtocol,
) -> Result<FileTransferParams, String> {
    let groups = REMOTE_GENERIC_OPT_REGEX
        .captures(s)
        .ok_or_else(|| String::from("Bad remote host syntax!"))?;

    let username = optional_capture(&groups, 1);
    let address = required_capture(&groups, 2, "address")?;
    let port = parse_port(groups.get(3), default_port_for_protocol(protocol))?;
    let remote_path = groups.get(4).map(|group| PathBuf::from(group.as_str()));
    let params = ProtocolParams::Generic(
        GenericProtocolParams::default()
            .address(address)
            .port(port)
            .username(username),
    );

    Ok(FileTransferParams::new(protocol, params).remote_path(remote_path))
}

fn parse_webdav_remote_opt(s: &str, prefix: &str) -> Result<FileTransferParams, String> {
    let groups = REMOTE_WEBDAV_OPT_REGEX
        .captures(s)
        .ok_or_else(|| String::from("Bad remote host syntax!"))?;

    let username = required_capture(&groups, 1, "username")?;
    let password = required_capture(&groups, 2, "password")?;
    let uri = required_capture(&groups, 3, "server URI")?;
    let remote_path = groups.get(4).map(|group| PathBuf::from(group.as_str()));
    let params = ProtocolParams::WebDAV(WebDAVProtocolParams {
        uri: format!("{prefix}://{uri}"),
        username,
        password,
    });

    Ok(FileTransferParams::new(FileTransferProtocol::WebDAV, params).remote_path(remote_path))
}

fn parse_s3_remote_opt(s: &str) -> Result<FileTransferParams, String> {
    let groups = REMOTE_S3_OPT_REGEX
        .captures(s)
        .ok_or_else(|| String::from("Bad remote host syntax!"))?;

    let bucket = optional_capture(&groups, 1).unwrap_or_default();
    let region = optional_capture(&groups, 2).unwrap_or_default();
    let profile = optional_capture(&groups, 3);
    let remote_path = groups.get(4).map(|group| PathBuf::from(group.as_str()));

    Ok(FileTransferParams::new(
        FileTransferProtocol::AwsS3,
        ProtocolParams::AwsS3(AwsS3Params::new(bucket, Some(region), profile)),
    )
    .remote_path(remote_path))
}

fn parse_kube_remote_opt(s: &str) -> Result<FileTransferParams, String> {
    let groups = REMOTE_KUBE_OPT_REGEX
        .captures(s)
        .ok_or_else(|| String::from("Bad remote host syntax!"))?;

    let namespace = optional_capture(&groups, 1);
    let cluster_url = optional_capture(&groups, 3);
    let remote_path = groups.get(5).map(|group| PathBuf::from(group.as_str()));

    Ok(FileTransferParams::new(
        FileTransferProtocol::Kube,
        ProtocolParams::Kube(KubeProtocolParams {
            namespace,
            cluster_url,
            username: None,
            client_cert: None,
            client_key: None,
        }),
    )
    .remote_path(remote_path))
}

#[cfg(smb_unix)]
fn parse_smb_remote_opts(s: &str) -> Result<FileTransferParams, String> {
    let groups = REMOTE_SMB_OPT_REGEX
        .captures(s)
        .ok_or_else(|| String::from("Bad remote host syntax!"))?;

    let username = optional_capture(&groups, 1).or_else(|| whoami::username().ok());
    let address = required_capture(&groups, 2, "address")?;
    let port = parse_port(groups.get(3), 445)?;
    let share = required_capture(&groups, 4, "share")?;
    let remote_path = groups.get(5).map(|group| PathBuf::from(group.as_str()));

    Ok(FileTransferParams::new(
        FileTransferProtocol::Smb,
        ProtocolParams::Smb(SmbParams::new(address, share).port(port).username(username)),
    )
    .remote_path(remote_path))
}

#[cfg(smb_windows)]
fn parse_smb_remote_opts(s: &str) -> Result<FileTransferParams, String> {
    let groups = REMOTE_SMB_OPT_REGEX
        .captures(s)
        .ok_or_else(|| String::from("Bad remote host syntax!"))?;

    let username = optional_capture(&groups, 1);
    let address = required_capture(&groups, 2, "address")?;
    let share = required_capture(&groups, 3, "share")?;
    let remote_path = groups.get(4).map(|group| PathBuf::from(group.as_str()));

    Ok(FileTransferParams::new(
        FileTransferProtocol::Smb,
        ProtocolParams::Smb(SmbParams::new(address, share).username(username)),
    )
    .remote_path(remote_path))
}
