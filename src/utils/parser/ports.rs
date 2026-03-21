use crate::filetransfer::FileTransferProtocol;

pub(super) fn default_port_for_protocol(protocol: FileTransferProtocol) -> u16 {
    match protocol {
        FileTransferProtocol::Ftp(_) => 21,
        FileTransferProtocol::Scp | FileTransferProtocol::Sftp => 22,
        _ => 22,
    }
}

pub(super) fn parse_port(port: Option<regex::Match<'_>>, default: u16) -> Result<u16, String> {
    match port {
        Some(port) => port
            .as_str()
            .parse::<u16>()
            .map_err(|err| format!("Bad port \"{}\": {err}", port.as_str())),
        None => Ok(default),
    }
}
