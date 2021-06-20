//! ## KeyringStorage
//!
//! `keyringstorage` provides an implementation of the `KeyStorage` trait using the OS keyring

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
                #[cfg(target_os = "linux")]
                KeyringError::SecretServiceError(_) => Err(KeyStorageError::ProviderError),
                KeyringError::Parse(_) => Err(KeyStorageError::BadSytax),
                _ => Err(KeyStorageError::ProviderError),
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
            #[cfg(not(target_os = "linux"))]
            Err(err) => !matches!(err, KeyringError::NoBackendFound),
            #[cfg(target_os = "linux")]
            Err(err) => !matches!(
                err,
                KeyringError::NoBackendFound | KeyringError::SecretServiceError(_)
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    extern crate whoami;
    use super::*;

    use pretty_assertions::assert_eq;
    use whoami::username;

    #[test]
    fn test_system_keys_keyringstorage() {
        let username: String = username();
        let storage: KeyringStorage = KeyringStorage::new(username.as_str());
        assert!(storage.is_supported());
        let app_name: &str = "termscp-test2";
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
