use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use ssh2_config::{Host, HostClause, ParseRule, SshConfig};

use crate::filetransfer::params::GenericProtocolParams;
use crate::filetransfer::{FileTransferParams, FileTransferProtocol, ProtocolParams};

/// Parameters required to add an ssh key for a host.
struct SshKeyParams {
    host: String,
    ssh_key: String,
    username: String,
}

/// Import ssh hosts from the specified ssh config file, or from the default location
/// and save them as bookmarks.
pub fn import_ssh_hosts(ssh_config: Option<PathBuf>, keyring: bool) -> Result<(), String> {
    // get config client
    let mut cfg_client = super::get_config_client()
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
        .for_each(|(name, params, identity_file_params)| {
            debug!("Adding bookmark for host: {name} with params: {params:?}");
            bookmarks_client.add_bookmark(name, params, false);

            // add ssh key if any
            if let Some(identity_file_params) = identity_file_params {
                debug!(
                    "Host {host} has identity file, will add ssh key for it",
                    host = identity_file_params.host
                );
                if let Err(err) = cfg_client.add_ssh_key(
                    &identity_file_params.host,
                    &identity_file_params.username,
                    &identity_file_params.ssh_key,
                ) {
                    error!(
                        "Could not add ssh key for host {host}: {err}",
                        host = identity_file_params.host
                    );
                }
            }
        });

    // save bookmarks
    if let Err(err) = bookmarks_client.write_bookmarks() {
        return Err(format!(
            "Could not save imported ssh hosts as bookmarks: {err}"
        ));
    }

    println!("Imported ssh hosts");

    Ok(())
}

/// Tries to derive [`FileTransferParams`] from the specified ssh host.
fn host_to_params(
    host: &Host,
) -> impl Iterator<Item = (String, FileTransferParams, Option<SshKeyParams>)> {
    host.pattern
        .iter()
        .filter_map(|pattern| host_pattern_to_params(host, pattern))
}

/// Tries to derive [`FileTransferParams`] from the specified ssh host and pattern.
///
/// If `IdentityFile` is specified in the host parameters, it will be included in the returned tuple.
fn host_pattern_to_params(
    host: &Host,
    pattern: &HostClause,
) -> Option<(String, FileTransferParams, Option<SshKeyParams>)> {
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

    let identity_file_params = resolve_identity_file_path(host, pattern, &address);

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
        identity_file_params,
    ))
}

fn resolve_identity_file_path(
    host: &Host,
    pattern: &HostClause,
    resolved_address: &str,
) -> Option<SshKeyParams> {
    let (Some(username), Some(identity_file)) = (
        host.params.user.as_ref(),
        host.params.identity_file.as_ref().and_then(|v| v.first()),
    ) else {
        debug!(
            "No identity file specified for host {host}, skipping ssh key import",
            host = pattern.pattern
        );
        return None;
    };

    // expand tilde
    let identity_filepath = shellexpand::tilde(&identity_file.display().to_string()).to_string();
    debug!("Resolved identity file for pattern {pattern}: {identity_filepath}",);
    let Ok(mut ssh_file) = File::open(identity_file) else {
        error!(
            "Could not open identity file {identity_filepath} for host {host}",
            host = pattern.pattern
        );
        return None;
    };
    let mut ssh_key = String::new();
    use std::io::Read as _;
    if let Err(err) = ssh_file.read_to_string(&mut ssh_key) {
        error!(
            "Could not read identity file {identity_filepath} for host {host}: {err}",
            host = pattern.pattern
        );
        return None;
    }

    Some(SshKeyParams {
        host: resolved_address.to_string(),
        username: username.clone(),
        ssh_key,
    })
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use super::*;
    use crate::system::bookmarks_client::BookmarksClient;

    #[test]
    fn test_should_import_ssh_hosts() {
        // enable debug env logger
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let ssh_test_config = ssh_test_config();

        // import ssh hosts
        let result = import_ssh_hosts(Some(ssh_test_config.config.path().to_path_buf()), false);
        assert!(result.is_ok());

        // verify imported hosts
        let config_client = super::super::get_config_client()
            .ok_or_else(|| String::from("Could not import ssh hosts: could not load configuration"))
            .expect("failed to load config client");

        // load bookmarks client
        let bookmarks_client = super::super::bookmarks_client(false)
            .expect("failed to load bookmarks client")
            .expect("bookmarks client is none");

        // verify bookmarks
        check_bookmark(&bookmarks_client, "test1", "test1.example.com", 2200, None);
        check_bookmark(
            &bookmarks_client,
            "test2",
            "test2.example.com",
            22,
            Some("test2user"),
        );
        check_bookmark(
            &bookmarks_client,
            "test3",
            "test3.example.com",
            2222,
            Some("test3user"),
        );

        // verify ssh keys
        let (host, username, _key) = config_client
            .get_ssh_key("test3user@test3.example.com")
            .expect("ssh key is missing for test3user@test3.example.com");

        assert_eq!(host, "test3.example.com");
        assert_eq!(username, "test3user");
    }

    fn check_bookmark(
        bookmarks_client: &BookmarksClient,
        name: &str,
        expected_address: &str,
        expected_port: u16,
        expected_username: Option<&str>,
    ) {
        // verify bookmarks
        let bookmark = bookmarks_client
            .get_bookmark(name)
            .expect("failed to get bookmark");
        let params1 = bookmark
            .params
            .generic_params()
            .expect("should have generic params");
        assert_eq!(params1.address, expected_address);
        assert_eq!(params1.port, expected_port);
        assert_eq!(params1.username.as_deref(), expected_username);
        assert!(params1.password.is_none());
    }

    struct SshTestConfig {
        config: NamedTempFile,
        #[allow(dead_code)]
        identity_file: NamedTempFile,
    }

    fn ssh_test_config() -> SshTestConfig {
        use std::io::Write as _;
        let mut identity_file = NamedTempFile::new().expect("failed to create tempfile");
        writeln!(
            identity_file,
            r"-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAABFwAAAAdzc2gtcn
NhAAAAAwEAAQAAAQEAxKyYUMRCNPlb4ZV1VMofrzApu2l3wgP4Ot9wBvHsw/+RMpcHIbQK
9iQqAVp8Z+M1fJyPXTKjoJtIzuCLF6Sjo0KI7/tFTh+yPnA5QYNLZOIRZb8skumL4gwHww
5Z942FDPuUDQ30C2mZR9lr3Cd5pA8S1ZSPTAV9QQHkpgoS8cAL8QC6dp3CJjUC8wzvXh3I
oN3bTKxCpM10KMEVuWO3lM4Nvr71auB9gzo1sFJ3bwebCZIRH01FROyA/GXRiaOtJFG/9N
nWWI/iG5AJzArKpLZNHIP+FxV/NoRH0WBXm9Wq5MrBYrD1NQzm+kInpS/2sXk3m1aZWqLm
HF2NKRXSbQAAA8iI+KSniPikpwAAAAdzc2gtcnNhAAABAQDErJhQxEI0+VvhlXVUyh+vMC
m7aXfCA/g633AG8ezD/5EylwchtAr2JCoBWnxn4zV8nI9dMqOgm0jO4IsXpKOjQojv+0VO
H7I+cDlBg0tk4hFlvyyS6YviDAfDDln3jYUM+5QNDfQLaZlH2WvcJ3mkDxLVlI9MBX1BAe
SmChLxwAvxALp2ncImNQLzDO9eHcig3dtMrEKkzXQowRW5Y7eUzg2+vvVq4H2DOjWwUndv
B5sJkhEfTUVE7ID8ZdGJo60kUb/02dZYj+IbkAnMCsqktk0cg/4XFX82hEfRYFeb1arkys
FisPU1DOb6QielL/axeTebVplaouYcXY0pFdJtAAAAAwEAAQAAAP8u3PFuTVV5SfGazwIm
MgNaux82iOsAT/HWFWecQAkqqrruUw5f+YajH/riV61NE9aq2qNOkcJrgpTWtqpt980GGd
SHWlgpRWQzfIooEiDk6Pk8RVFZsEykkDlJQSIu2onZjhi5A5ojHgZoGGabDsztSqoyOjPq
6WPvGYRiDAR3leBMyp1WufBCJqAsC4L8CjPJSmnZhc5a0zXkC9Syz74Fa08tdM7bGhtvP1
GmzuYxkgxHH2IFeoumUSBHRiTZayGuRUDel6jgEiUMxenaDKXe7FpYzMm9tQZA10Mm4LhK
5rP9nd2/KRTFRnfZMnKvtIRC9vtlSLBe14qw+4ZCl60AAACAf1kghlO3+HIWplOmk/lCL0
w75Zz+RdvueL9UuoyNN1QrUEY420LsixgWSeRPby+Rb/hW+XSAZJQHowQ8acFJhU85So7f
4O4wcDuE4f6hpsW9tTfkCEUdLCQJ7EKLCrod6jIV7hvI6rvXiVucRpeAzdOaq4uzj2cwDd
tOdYVsnmQAAACBAOVxBsvO/Sr3rZUbNtA6KewZh/09HNGoKNaCeiD7vaSn2UJbbPRByF/o
Oo5zv8ee8r3882NnmG808XfSn7pPZAzbbTmOaJt0fmyZhivCghSNzV6njW3o0PdnC0fGZQ
ruVXgkd7RJFbsIiD4dDcF4VCjwWHfTK21EOgJUA5pN6TNvAAAAgQDbcJWRx8Uyhkj2+srb
3n2Rt6CR7kEl9cw17ItFjMn+pO81/5U2aGw0iLlX7E06TAMQC+dyW/WaxQRey8RRdtbJ1e
TNKCN34QCWkyuYRHGhcNc0quEDayPw5QWGXlP4BzjfRUcPxY9cCXLe5wDLYsX33HwOAc59
RorU9FCmS/654wAAABFyb290QDhjNTBmZDRjMzQ1YQECAw==
-----END OPENSSH PRIVATE KEY-----"
        )
        .expect("failed to write identity file");

        let mut file = NamedTempFile::new().expect("failed to create tempfile");

        // let's declare a couple of hosts
        writeln!(
            file,
            r#"
Host test1
    HostName test1.example.com
    Port 2200

Host test2
    HostName test2.example.com
    User test2user

Host test3
    HostName test3.example.com
    User test3user
    Port 2222
    IdentityFile {identity_path}
"#,
            identity_path = identity_file.path().display()
        )
        .expect("failed to write ssh config");

        SshTestConfig {
            config: file,
            identity_file,
        }
    }
}
