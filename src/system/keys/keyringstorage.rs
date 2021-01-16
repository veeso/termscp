//! ## KeyringStorage
//!
//! `keyringstorage` provides an implementation of the `KeyStorage` trait using the OS keyring

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
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

// Deps
extern crate keyring;
// Local
use super::{KeyStorage, KeyStorageError};
// Ext
use keyring::{Keyring, KeyringError};

/// ## KeyringStorage
///
/// provides a `KeyStorage` implementation using the keyring crate
pub struct KeyringStorage {
    username: String,
}

impl KeyringStorage {
    /// ### new
    ///
    /// Instantiates a new KeyringStorage
    pub fn new(username: &str) -> Self {
        KeyringStorage {
            username: username.to_string(),
        }
    }
}

impl KeyStorage for KeyringStorage {
    /// ### get_key
    ///
    /// Retrieve key from the key storage.
    /// The key might be acccess through an identifier, which identifies
    /// the key in the storage
    fn get_key(&self, storage_id: &str) -> Result<String, KeyStorageError> {
        let storage: Keyring = Keyring::new(storage_id, self.username.as_str());
        match storage.get_password() {
            Ok(s) => Ok(s),
            Err(e) => match e {
                KeyringError::NoPasswordFound => Err(KeyStorageError::NoSuchKey),
                #[cfg(target_os = "windows")]
                KeyringError::WindowsVaultError => Err(KeyStorageError::NoSuchKey),
                #[cfg(target_os = "macos")]
                KeyringError::MacOsKeychainError(_) => Err(KeyStorageError::NoSuchKey),
                _ => panic!("{}", e),
            },
        }
    }

    /// ### set_key
    ///
    /// Set the key into the key storage
    fn set_key(&self, storage_id: &str, key: &str) -> Result<(), KeyStorageError> {
        let storage: Keyring = Keyring::new(storage_id, self.username.as_str());
        match storage.set_password(key) {
            Ok(_) => Ok(()),
            Err(_) => Err(KeyStorageError::ProviderError),
        }
    }

    /// is_supported
    ///
    /// Returns whether the key storage is supported on the host system
    fn is_supported(&self) -> bool {
        let dummy: String = String::from("dummy-service");
        let storage: Keyring = Keyring::new(dummy.as_str(), self.username.as_str());
        // Check what kind of error is returned
        match storage.get_password() {
            Ok(_) => true,
            Err(err) => match err {
                KeyringError::NoBackendFound => false,
                //#[cfg(target_os = "macos")]
                //KeyringError::MacOsKeychainError(_) => false,
                //#[cfg(target_os = "windows")]
                //KeyringError::WindowsVaultError => false,
                _ => true,
            },
        }
    }
}

#[cfg(test)]
mod tests {

    extern crate whoami;
    use super::*;

    use whoami::username;

    #[test]
    fn test_system_keys_keyringstorage() {
        let username: String = username();
        let storage: KeyringStorage = KeyringStorage::new(username.as_str());
        assert!(storage.is_supported());
        let app_name: &str = "termscp";
        let secret: &str = "Th15-15/My-Супер-Секрет";
        let kring: Keyring = Keyring::new(app_name, username.as_str());
        let _ = kring.delete_password();
        drop(kring);
        // Secret should not exist
        assert_eq!(
            storage.get_key(app_name).err().unwrap(),
            KeyStorageError::NoSuchKey
        );
        // Write secret
        assert!(storage.set_key(app_name, secret).is_ok());
        // Get secret
        assert_eq!(storage.get_key(app_name).ok().unwrap().as_str(), secret);

        // Delete the key manually...
        let kring: Keyring = Keyring::new(app_name, username.as_str());
        assert!(kring.delete_password().is_ok());
    }
}
