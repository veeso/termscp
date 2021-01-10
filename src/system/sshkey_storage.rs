//! ## SshKeyStorage
//!
//! `SshKeyStorage` is the module which behaves a storage for ssh keys

/*
*
*   Copyright (C) 2020-2021Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// Locals
use super::config_client::ConfigClient;
// Ext
use std::collections::HashMap;
use std::path::PathBuf;

pub struct SshKeyStorage {
    hosts: HashMap<String, PathBuf>, // Association between {user}@{host} and RSA key path
}

impl SshKeyStorage {
    /// ### storage_from_config
    ///
    /// Create a `SshKeyStorage` starting from a `ConfigClient`
    pub fn storage_from_config(cfg_client: &ConfigClient) -> Self {
        let mut hosts: HashMap<String, PathBuf> =
            HashMap::with_capacity(cfg_client.iter_ssh_keys().count());
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
                Err(_) => continue,
            }
        }
        // Return storage
        SshKeyStorage { hosts }
    }

    /// ### empty
    ///
    /// Create an empty ssh key storage; used in case `ConfigClient` is not available
    pub fn empty() -> Self {
        SshKeyStorage {
            hosts: HashMap::new(),
        }
    }

    /// ### resolve
    ///
    /// Return RSA key path from host and username
    pub fn resolve(&self, host: &str, username: &str) -> Option<&PathBuf> {
        let key: String = Self::make_mapkey(host, username);
        self.hosts.get(&key)
    }

    /// ### make_mapkey
    ///
    /// Make mapkey from host and username
    fn make_mapkey(host: &str, username: &str) -> String {
        format!("{}@{}", username, host)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::system::config_client::ConfigClient;
    use std::path::Path;

    #[test]
    fn test_system_sshkey_storage_new() {
        let tmp_dir: tempfile::TempDir = create_tmp_dir();
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

    /// ### get_paths
    ///
    /// Get paths for configuration and keys directory
    fn get_paths(dir: &Path) -> (PathBuf, PathBuf) {
        let mut k: PathBuf = PathBuf::from(dir);
        let mut c: PathBuf = k.clone();
        k.push("ssh-keys/");
        c.push("config.toml");
        (c, k)
    }

    /// ### create_tmp_dir
    ///
    /// Create temporary directory
    fn create_tmp_dir() -> tempfile::TempDir {
        tempfile::TempDir::new().ok().unwrap()
    }
}
