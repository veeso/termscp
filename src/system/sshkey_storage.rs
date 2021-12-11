//! ## SshKeyStorage
//!
//! `SshKeyStorage` is the module which behaves a storage for ssh keys

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// Locals
use super::config_client::ConfigClient;
// Ext
use remotefs::client::ssh::SshKeyStorage as SshKeyStorageT;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct SshKeyStorage {
    hosts: HashMap<String, PathBuf>, // Association between {user}@{host} and RSA key path
}

impl SshKeyStorage {
    /// Create a `SshKeyStorage` starting from a `ConfigClient`
    pub fn storage_from_config(cfg_client: &ConfigClient) -> Self {
        let mut hosts: HashMap<String, PathBuf> =
            HashMap::with_capacity(cfg_client.iter_ssh_keys().count());
        debug!("Setting up SSH key storage");
        // Iterate over keys
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
        SshKeyStorage { hosts }
    }

    /// Create an empty ssh key storage; used in case `ConfigClient` is not available
    #[cfg(test)]
    pub fn empty() -> Self {
        SshKeyStorage {
            hosts: HashMap::new(),
        }
    }

    /// Make mapkey from host and username
    fn make_mapkey(host: &str, username: &str) -> String {
        format!("{}@{}", username, host)
    }

    #[cfg(test)]
    /// Add a key to storage
    /// NOTE: available only for tests
    pub fn add_key(&mut self, host: &str, username: &str, p: PathBuf) {
        let key: String = Self::make_mapkey(host, username);
        self.hosts.insert(key, p);
    }
}

impl SshKeyStorageT for SshKeyStorage {
    fn resolve(&self, host: &str, username: &str) -> Option<&Path> {
        let key: String = Self::make_mapkey(host, username);
        self.hosts.get(&key).map(|x| x.as_path())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::system::config_client::ConfigClient;

    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn test_system_sshkey_storage_new() {
        let tmp_dir: tempfile::TempDir = tempfile::TempDir::new().ok().unwrap();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        let mut client: ConfigClient = ConfigClient::new(cfg_path.as_path(), key_path.as_path())
            .ok()
            .unwrap();
        // Add ssh key
        assert!(client
            .add_ssh_key("192.168.1.31", "pi", "piroporopero")
            .is_ok());
        // Create ssh key storage
        let storage: SshKeyStorage = SshKeyStorage::storage_from_config(&client);
        // Verify key exists
        let mut exp_key_path: PathBuf = key_path.clone();
        exp_key_path.push("pi@192.168.1.31.key");
        assert_eq!(
            *storage.resolve("192.168.1.31", "pi").unwrap(),
            exp_key_path
        );
        // Verify unexisting key
        assert!(storage.resolve("deskichup", "veeso").is_none());
    }

    #[test]
    fn test_system_sshkey_storage_empty() {
        let storage: SshKeyStorage = SshKeyStorage::empty();
        assert_eq!(storage.hosts.len(), 0);
    }

    #[test]
    fn test_system_sshkey_storage_add() {
        let mut storage: SshKeyStorage = SshKeyStorage::empty();
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
