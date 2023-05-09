//! ## KeyStorage
//!
//! `keystorage` provides the trait to manipulate to a KeyStorage

// Storages
pub mod filestorage;
#[cfg(feature = "with-keyring")]
pub mod keyringstorage;
// ext
#[cfg(feature = "with-keyring")]
use keyring::Error as KeyringError;
use thiserror::Error;

/// defines the error type for the `KeyStorage`
#[derive(Debug, Error)]
pub enum KeyStorageError {
    #[cfg(feature = "with-keyring")]
    #[error("Key has a bad syntax")]
    BadSytax,
    #[error("Provider service error")]
    ProviderError,
    #[error("No such key")]
    NoSuchKey,
    #[cfg(feature = "with-keyring")]
    #[error("keyring error: {0}")]
    KeyringError(KeyringError),
}

#[cfg(feature = "with-keyring")]
impl From<KeyringError> for KeyStorageError {
    fn from(e: KeyringError) -> Self {
        Self::KeyringError(e)
    }
}

/// this traits provides the methods to communicate and interact with the key storage.
pub trait KeyStorage {
    /// Retrieve key from the key storage.
    /// The key might be acccess through an identifier, which identifies
    /// the key in the storage
    fn get_key(&self, storage_id: &str) -> Result<String, KeyStorageError>;

    /// Set the key into the key storage
    fn set_key(&self, storage_id: &str, key: &str) -> Result<(), KeyStorageError>;

    /// is_supported
    ///
    /// Returns whether the key storage is supported on the host system
    fn is_supported(&self) -> bool;
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_system_keys_mod_errors() {
        #[cfg(feature = "with-keyring")]
        assert_eq!(
            KeyStorageError::BadSytax.to_string(),
            String::from("Key has a bad syntax")
        );
        assert_eq!(
            KeyStorageError::ProviderError.to_string(),
            String::from("Provider service error")
        );
        assert_eq!(
            KeyStorageError::NoSuchKey.to_string(),
            String::from("No such key")
        );
    }
}
