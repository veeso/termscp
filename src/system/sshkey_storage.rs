//! ## SshKeyStorage
//!
//! `SshKeyStorage` is the module which behaves a storage for ssh keys

// Locals
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// Ext
use remotefs_ssh::SshKeyStorage as SshKeyStorageTrait;
use ssh2_config::SshConfig;

use super::config_client::ConfigClient;
use crate::utils::ssh as ssh_utils;

#[derive(Default)]
pub struct SshKeyStorage {
    /// Association between {user}@{host} and RSA key path
    hosts: HashMap<String, PathBuf>,
    /// Ssh2 configuration
    ssh_config: Option<SshConfig>,
}

impl SshKeyStorage {
    /// Make mapkey from host and username
    fn make_mapkey(host: &str, username: &str) -> String {
        format!("{username}@{host}")
    }

    #[cfg(test)]
    /// Add a key to storage
    /// NOTE: available only for tests
    pub fn add_key(&mut self, host: &str, username: &str, p: PathBuf) {
        let key: String = Self::make_mapkey(host, username);
        self.hosts.insert(key, p);
    }

    /// Resolve host via termscp ssh keys storage
    fn resolve_host_in_termscp_storage(&self, host: &str, username: &str) -> Option<&Path> {
        let key: String = Self::make_mapkey(host, username);
        self.hosts.get(&key).map(|x| x.as_path())
    }

    /// Resolve host via ssh2 configuration
    fn resolve_host_in_ssh2_configuration(&self, host: &str) -> Option<PathBuf> {
        self.ssh_config.as_ref().and_then(|x| {
            x.query(host)
                .identity_file
                .as_ref()
                .and_then(|x| x.first().cloned())
        })
    }

    /// Get default SSH identity files that SSH would normally try
    /// This mirrors the behavior of OpenSSH client
    fn get_default_identity_files(&self) -> Vec<PathBuf> {
        let Some(home_dir) = dirs::home_dir() else {
            return Vec::new();
        };

        let ssh_dir = home_dir.join(".ssh");

        // Standard SSH identity files in order of preference (matches OpenSSH)
        ["id_ed25519", "id_ecdsa", "id_rsa", "id_dsa"]
            .iter()
            .map(|key_name| ssh_dir.join(key_name))
            .filter(|key_path| key_path.exists())
            .collect()
    }
}

impl SshKeyStorageTrait for SshKeyStorage {
    fn resolve(&self, host: &str, username: &str) -> Option<PathBuf> {
        // search in termscp keys
        if let Some(path) = self.resolve_host_in_termscp_storage(host, username) {
            return Some(path.to_path_buf());
        }
        debug!(
            "couldn't find any ssh key associated to {} at {}. Trying with ssh2 config",
            username, host
        );
        // otherwise search in configuration
        if let Some(key) = self.resolve_host_in_ssh2_configuration(host) {
            debug!("Found key in SSH config for {host}: {}", key.display());
            return Some(key);
        }

        // As a final fallback, try default SSH identity files (like regular ssh does)
        self.get_default_identity_files().into_iter().next()
    }
}

impl From<&ConfigClient> for SshKeyStorage {
    fn from(cfg_client: &ConfigClient) -> Self {
        // read ssh2 config
        let ssh_config = cfg_client.get_ssh_config().and_then(|x| {
            debug!("reading ssh config at {}", x);
            ssh_utils::parse_ssh2_config(x).ok()
        });
        let mut hosts: HashMap<String, PathBuf> =
            HashMap::with_capacity(cfg_client.iter_ssh_keys().count());
        debug!("Setting up SSH key storage");
        // Iterate over keys in storage
        for key in cfg_client.iter_ssh_keys() {
            match cfg_client.get_ssh_key(key) {
                Ok(host) => match host {
                    Some((addr, username, rsa_key_path)) => {
                        let key_name: String = Self::make_mapkey(&addr, &username);
                        hosts.insert(key_name, rsa_key_path);
                    }
                    None => continue,
                },
                Err(err) => {
                    error!("Failed to get SSH key for {}: {}", key, err);
                    continue;
                }
            }
            info!("Got SSH key for {}", key);
        }
        // Return storage
        SshKeyStorage { hosts, ssh_config }
    }
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::system::config_client::ConfigClient;
    use crate::utils::test_helpers;

    #[test]
    fn test_system_sshkey_storage_new() {
        let tmp_dir: tempfile::TempDir = tempfile::TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        // Add ssh key
        assert!(
            client
                .add_ssh_key("192.168.1.31", "pi", "piroporopero")
                .is_ok()
        );
        // Create ssh key storage
        let storage: SshKeyStorage = SshKeyStorage::from(&client);
        // Verify key exists
        let mut exp_key_path: PathBuf = key_path;
        exp_key_path.push("pi@192.168.1.31.key");
        assert_eq!(
            *storage.resolve("192.168.1.31", "pi").unwrap(),
            exp_key_path
        );
        // Verify key is a default key or none
        let default_keys: Vec<PathBuf> = storage.get_default_identity_files().into_iter().collect();

        if let Some(key) = storage.resolve("deskichup", "veeso") {
            assert!(default_keys.contains(&key));
        } else {
            assert!(default_keys.is_empty());
        }
    }

    #[test]
    fn should_resolve_key_from_ssh2_config() {
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
        // make storage
        let tmp_dir: tempfile::TempDir = tempfile::TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        client.set_ssh_config(Some(ssh_config_file.path().to_string_lossy().to_string()));
        let storage: SshKeyStorage = SshKeyStorage::from(&client);
        assert_eq!(
            storage.resolve("test", "pi").unwrap().as_path(),
            rsa_key.path()
        );
    }

    #[test]
    fn test_system_sshkey_storage_empty() {
        let storage: SshKeyStorage = SshKeyStorage::default();
        assert_eq!(storage.hosts.len(), 0);
    }

    #[test]
    fn test_system_sshkey_storage_add() {
        let mut storage: SshKeyStorage = SshKeyStorage::default();
        storage.add_key("deskichup", "veeso", PathBuf::from("/tmp/omar"));
        assert_eq!(
            *storage.resolve("deskichup", "veeso").unwrap(),
            PathBuf::from("/tmp/omar")
        );
    }

    /// Get paths for configuration and keys directory
    fn get_paths(dir: &Path) -> (PathBuf, PathBuf) {
        let mut k: PathBuf = PathBuf::from(dir);
        let mut c: PathBuf = k.clone();
        k.push("ssh-keys/");
        c.push("config.toml");
        (c, k)
    }
}
