//! ## KeyringStorage
//!
//! `keyringstorage` provides an implementation of the `KeyStorage` trait using the OS keyring

// Local
// Ext
use keyring::{Entry as Keyring, Error as KeyringError};

use super::{KeyStorage, KeyStorageError};

/// provides a `KeyStorage` implementation using the keyring crate
pub struct KeyringStorage {
    username: String,
}

impl KeyringStorage {
    /// Instantiates a new KeyringStorage
    pub fn new(username: &str) -> Self {
        KeyringStorage {
            username: username.to_string(),
        }
    }
}

impl KeyStorage for KeyringStorage {
    /// Retrieve key from the key storage.
    /// The key might be acccess through an identifier, which identifies
    /// the key in the storage
    fn get_key(&self, storage_id: &str) -> Result<String, KeyStorageError> {
        let storage: Keyring = Keyring::new(storage_id, self.username.as_str())?;
        match storage.get_password() {
            Ok(s) => Ok(s),
            Err(e) => match e {
                KeyringError::NoEntry => Err(KeyStorageError::NoSuchKey),
                KeyringError::PlatformFailure(_)
                | KeyringError::NoStorageAccess(_)
                | KeyringError::Invalid(_, _)
                | KeyringError::Ambiguous(_) => Err(KeyStorageError::ProviderError),
                KeyringError::BadEncoding(_) | KeyringError::TooLong(_, _) => {
                    Err(KeyStorageError::BadSyntax)
                }
                _ => Err(KeyStorageError::ProviderError),
            },
        }
    }

    /// Set the key into the key storage
    fn set_key(&self, storage_id: &str, key: &str) -> Result<(), KeyStorageError> {
        let storage: Keyring = Keyring::new(storage_id, self.username.as_str())?;
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
        let storage: Keyring = match Keyring::new(dummy.as_str(), self.username.as_str()) {
            Ok(s) => s,
            Err(e) => {
                error!("could not instantiate keyring {e}");
                return false;
            }
        };
        // Check what kind of error is returned
        match storage.get_password() {
            Ok(_) => true,
            Err(KeyringError::NoStorageAccess(_) | KeyringError::PlatformFailure(_)) => false,
            Err(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(all(not(feature = "github-actions"), not(feature = "isolated-tests")))]
    fn test_system_keys_keyring_storage() {
        use pretty_assertions::assert_eq;

        use super::*;

        let username = whoami::username().expect("no username");
        let storage = KeyringStorage::new(username.as_str());
        assert!(storage.is_supported());
        let app_name: &str = "termscp-test2";
        let secret: &str = "Th15-15/My-Супер-Секрет";
        let kring: Keyring = Keyring::new(app_name, username.as_str()).unwrap();
        let _ = kring.delete_credential();
        drop(kring);
        // Secret should not exist
        assert!(storage.get_key(app_name).is_err());
        // Write secret
        assert!(storage.set_key(app_name, secret).is_ok());
        // Get secret
        assert_eq!(storage.get_key(app_name).unwrap().as_str(), secret);

        // Delete the key manually...
        let kring: Keyring = Keyring::new(app_name, username.as_str()).unwrap();
        assert!(kring.delete_credential().is_ok());
    }
}
