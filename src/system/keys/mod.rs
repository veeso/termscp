//! ## KeyStorage
//!
//! `keystorage` provides the trait to manipulate to a KeyStorage

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
// Storages
pub mod filestorage;
#[cfg(feature = "with-keyring")]
pub mod keyringstorage;
// ext
use thiserror::Error;

/// ## KeyStorageError
///
/// defines the error type for the `KeyStorage`
#[derive(Debug, Error, PartialEq)]
pub enum KeyStorageError {
    #[cfg(feature = "with-keyring")]
    #[error("Key has a bad syntax")]
    BadSytax,
    #[error("Provider service error")]
    ProviderError,
    #[error("No such key")]
    NoSuchKey,
}

/// ## KeyStorage
///
/// this traits provides the methods to communicate and interact with the key storage.
pub trait KeyStorage {
    /// ### get_key
    ///
    /// Retrieve key from the key storage.
    /// The key might be acccess through an identifier, which identifies
    /// the key in the storage
    fn get_key(&self, storage_id: &str) -> Result<String, KeyStorageError>;

    /// ### set_key
    ///
    /// Set the key into the key storage
    fn set_key(&self, storage_id: &str, key: &str) -> Result<(), KeyStorageError>;

    /// is_supported
    ///
    /// Returns whether the key storage is supported on the host system
    fn is_supported(&self) -> bool;
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_system_keys_mod_errors() {
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
