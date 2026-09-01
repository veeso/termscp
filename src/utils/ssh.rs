//! ## SSH Utilities
//!
//! Provides small helpers for loading SSH configuration files used by bookmarks
//! and setup flows.

use ssh2_config::{ParseRule, SshConfig};

/// The standard port used when an SSH configuration does not define one.
const DEFAULT_SSH_PORT: u16 = 22;

/// Connection parameters resolved from an SSH host configuration.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SshHostParams {
    /// Resolved SSH port.
    pub(crate) port: u16,
    /// Resolved SSH username, when configured.
    pub(crate) username: Option<String>,
}

/// Parses an OpenSSH-style config file into an `ssh2_config::SshConfig`.
pub fn parse_ssh2_config(path: &str) -> Result<SshConfig, String> {
    use std::fs::File;
    use std::io::BufReader;

    let mut reader = File::open(path)
        .map_err(|e| format!("failed to open {path}: {e}"))
        .map(BufReader::new)?;
    SshConfig::default()
        .parse(&mut reader, ParseRule::ALLOW_UNKNOWN_FIELDS)
        .map_err(|e| format!("Failed to parse ssh2 config: {e}"))
}

/// Resolves SSH connection parameters for a host from the startup-parsed configuration.
pub(crate) fn resolve_ssh_host_params(config: Option<&SshConfig>, host: &str) -> SshHostParams {
    let params = config.map(|config| config.query(host));

    SshHostParams {
        port: params
            .as_ref()
            .and_then(|params| params.port)
            .unwrap_or(DEFAULT_SSH_PORT),
        username: params.and_then(|params| params.user),
    }
}

#[cfg(test)]
mod test {

    use super::{SshHostParams, parse_ssh2_config, resolve_ssh_host_params};
    use crate::utils::test_helpers;

    #[test]
    fn should_parse_ssh2_config() {
        let rsa_key = test_helpers::create_sample_file_with_content(
            "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDErJhQxEI0+VvhlXVUyh+vMCm7aXfCA/g633AG8ezD/5EylwchtAr2JCoBWnxn4zV8nI9dMqOgm0jO4IsXpKOjQojv+0VOH7I+cDlBg0tk4hFlvyyS6YviDAfDDln3jYUM+5QNDfQLaZlH2WvcJ3mkDxLVlI9MBX1BAeSmChLxwAvxALp2ncImNQLzDO9eHcig3dtMrEKkzXQowRW5Y7eUzg2+vvVq4H2DOjWwUndvB5sJkhEfTUVE7ID8ZdGJo60kUb/02dZYj+IbkAnMCsqktk0cg/4XFX82hEfRYFeb1arkysFisPU1DOb6QielL/axeTebVplaouYcXY0pFdJt root@8c50fd4c345a",
        );
        let ssh_config_file = test_helpers::create_sample_file_with_content(format!(
            r#"
Host test
        HostName 127.0.0.1
        Port 2222
        User test
        IdentityFile {}
        StrictHostKeyChecking no
        UserKnownHostsFile /dev/null
"#,
            rsa_key.path().display()
        ));

        assert!(
            parse_ssh2_config(
                ssh_config_file
                    .path()
                    .to_string_lossy()
                    .to_string()
                    .as_str()
            )
            .is_ok()
        );
    }

    #[test]
    fn ssh_host_params_should_resolve_exact_and_wildcard_hosts() {
        let ssh_config_file = test_helpers::create_sample_file_with_content(
            r#"
Host exact-host
    Port 2222
    User exact-user

Host *.example.com
    Port 2200
    User wildcard-user
"#,
        );
        let config = parse_ssh2_config(&ssh_config_file.path().to_string_lossy())
            .expect("test SSH configuration should parse");

        assert_eq!(
            resolve_ssh_host_params(Some(&config), "exact-host"),
            SshHostParams {
                port: 2222,
                username: Some("exact-user".to_string()),
            }
        );
        assert_eq!(
            resolve_ssh_host_params(Some(&config), "server.example.com"),
            SshHostParams {
                port: 2200,
                username: Some("wildcard-user".to_string()),
            }
        );
    }

    #[test]
    fn ssh_host_params_should_default_when_configuration_has_no_values() {
        let ssh_config_file =
            test_helpers::create_sample_file_with_content("Host unconfigured-host\n");
        let config = parse_ssh2_config(&ssh_config_file.path().to_string_lossy())
            .expect("test SSH configuration should parse");

        assert_eq!(
            resolve_ssh_host_params(Some(&config), "unconfigured-host"),
            SshHostParams {
                port: 22,
                username: None,
            }
        );
    }
}
