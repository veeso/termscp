use std::str::FromStr;

use super::REMOTE_OPT_PROTOCOL_REGEX;
use crate::filetransfer::FileTransferProtocol;

pub(super) fn parse_remote_opt_protocol(
    s: &str,
    default: FileTransferProtocol,
) -> Result<(FileTransferProtocol, String), String> {
    let groups = REMOTE_OPT_PROTOCOL_REGEX
        .captures(s)
        .ok_or_else(|| "Invalid args".to_string())?;

    let protocol = match groups.get(1) {
        Some(protocol) => FileTransferProtocol::from_str(protocol.as_str())
            .map_err(|_| format!("Unknown protocol \"{}\"", protocol.as_str()))?,
        #[cfg(smb_windows)]
        None if groups.get(2).is_some() => FileTransferProtocol::Smb,
        None => default,
    };

    let remote = groups
        .get(3)
        .map(|group| group.as_str().to_string())
        .unwrap_or_default();

    Ok((protocol, remote))
}
