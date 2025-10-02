use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use ssh2_config::{Host, HostClause, ParseRule, SshConfig};

use crate::filetransfer::params::GenericProtocolParams;
use crate::filetransfer::{FileTransferParams, FileTransferProtocol, ProtocolParams};

/// Import ssh hosts from the specified ssh config file, or from the default location
/// and save them as bookmarks.
pub fn import_ssh_hosts(ssh_config: Option<PathBuf>, keyring: bool) -> Result<(), String> {
    // get config client
    let cfg_client = super::get_config_client()
        .ok_or_else(|| String::from("Could not import ssh hosts: could not load configuration"))?;

    // resolve ssh_config
    let ssh_config = ssh_config.or_else(|| cfg_client.get_ssh_config().map(PathBuf::from));

    // load bookmarks client
    let mut bookmarks_client = super::bookmarks_client(keyring)?
        .ok_or_else(|| String::from("Could not import ssh hosts: could not load bookmarks"))?;

    // load ssh config
    let ssh_config = match ssh_config {
        Some(p) => {
            debug!("Importing ssh hosts from file: {}", p.display());
            let mut reader = BufReader::new(
                File::open(&p)
                    .map_err(|e| format!("Could not open ssh config file {}: {e}", p.display()))?,
            );
            SshConfig::default().parse(&mut reader, ParseRule::ALLOW_UNKNOWN_FIELDS)
        }
        None => {
            debug!("Importing ssh hosts from default location");
            SshConfig::parse_default_file(ParseRule::ALLOW_UNKNOWN_FIELDS)
        }
    }
    .map_err(|e| format!("Could not parse ssh config file: {e}"))?;

    // iter hosts and add bookmarks
    ssh_config
        .get_hosts()
        .iter()
        .flat_map(host_to_params)
        .for_each(|(name, params)| {
            debug!("Adding bookmark for host: {name} with params: {params:?}");
            bookmarks_client.add_bookmark(name, params, false)
        });

    println!("Imported ssh hosts");

    Ok(())
}

/// Tries to derive [`FileTransferParams`] from the specified ssh host.
fn host_to_params(host: &Host) -> impl Iterator<Item = (String, FileTransferParams)> {
    host.pattern
        .iter()
        .filter_map(|pattern| host_pattern_to_params(host, pattern))
}

/// Tries to derive [`FileTransferParams`] from the specified ssh host and pattern.
fn host_pattern_to_params(
    host: &Host,
    pattern: &HostClause,
) -> Option<(String, FileTransferParams)> {
    debug!("Processing host with pattern: {pattern:?}",);
    if pattern.negated || pattern.pattern.contains('*') || pattern.pattern.contains('?') {
        debug!("Skipping host with pattern: {pattern}",);
        return None;
    }

    let address = host
        .params
        .host_name
        .as_deref()
        .unwrap_or(pattern.pattern.as_str())
        .to_string();
    debug!("Resolved address for pattern {pattern}: {address}");
    let port = host.params.port.unwrap_or(22);
    debug!("Resolved port for pattern {pattern}: {port}");
    let username = host.params.user.clone();
    debug!("Resolved username for pattern {pattern}: {username:?}");

    Some((
        pattern.to_string(),
        FileTransferParams::new(
            FileTransferProtocol::Sftp,
            ProtocolParams::Generic(
                GenericProtocolParams::default()
                    .address(address)
                    .port(port)
                    .username(username),
            ),
        ),
    ))
}
