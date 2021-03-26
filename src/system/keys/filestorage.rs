//! ## FileStorage
//!
//! `filestorage` provides an implementation of the `KeyStorage` trait using a file

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
// Local
use super::{KeyStorage, KeyStorageError};
// Ext
use std::fs::{OpenOptions, Permissions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// ## FileStorage
///
/// File storage is an implementation o the `KeyStorage` which uses a file to store the key
pub struct FileStorage {
    dir_path: PathBuf,
}

impl FileStorage {
    /// ### new
    ///
    /// Instantiates a new `FileStorage`
    pub fn new(dir_path: &Path) -> Self {
        FileStorage {
            dir_path: PathBuf::from(dir_path),
        }
    }

    /// ### make_file_path
    ///
    /// Make file path for key file from `dir_path` and the application id
    fn make_file_path(&self, storage_id: &str) -> PathBuf {
        let mut p: PathBuf = self.dir_path.clone();
        let file_name = format!(".{}.key", storage_id);
        p.push(file_name);
        p
    }
}

impl KeyStorage for FileStorage {
    /// ### get_key
    ///
    /// Retrieve key from the key storage.
    /// The key might be acccess through an identifier, which identifies
    /// the key in the storage
    fn get_key(&self, storage_id: &str) -> Result<String, KeyStorageError> {
        let key_file: PathBuf = self.make_file_path(storage_id);
        // Check if file exists
        if !key_file.exists() {
            return Err(KeyStorageError::NoSuchKey);
        }
        // Read key from file
        match OpenOptions::new().read(true).open(key_file.as_path()) {
            Ok(mut file) => {
                let mut key: String = String::new();
                match file.read_to_string(&mut key) {
                    Ok(_) => Ok(key),
                    Err(_) => Err(KeyStorageError::ProviderError),
                }
            }
            Err(_) => Err(KeyStorageError::ProviderError),
        }
    }

    /// ### set_key
    ///
    /// Set the key into the key storage
    fn set_key(&self, storage_id: &str, key: &str) -> Result<(), KeyStorageError> {
        let key_file: PathBuf = self.make_file_path(storage_id);
        // Write key
        match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(key_file.as_path())
        {
            Ok(mut file) => {
                // Write key to file
                if file.write_all(key.as_bytes()).is_err() {
                    return Err(KeyStorageError::ProviderError);
                }
                // Set file to readonly
                let mut permissions: Permissions = file.metadata().unwrap().permissions();
                permissions.set_readonly(true);
                let _ = file.set_permissions(permissions);
                Ok(())
            }
            Err(_) => Err(KeyStorageError::ProviderError),
        }
    }

    /// is_supported
    ///
    /// Returns whether the key storage is supported on the host system
    fn is_supported(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_system_keys_filestorage_make_dir() {
        let storage: FileStorage = FileStorage::new(&Path::new("/tmp/"));
        assert_eq!(
            storage.make_file_path("bookmarks").as_path(),
            Path::new("/tmp/.bookmarks.key")
        );
    }

    #[test]
    fn test_system_keys_filestorage_ok() {
        let key_dir: tempfile::TempDir =
            tempfile::TempDir::new().expect("Could not create tempdir");
        let storage: FileStorage = FileStorage::new(key_dir.path());
        // Supported
        assert!(storage.is_supported());
        let app_name: &str = "termscp";
        let secret: &str = "Th15-15/My-Супер-Секрет";
        // Secret should not exist
        assert_eq!(
            storage.get_key(app_name).err().unwrap(),
            KeyStorageError::NoSuchKey
        );
        // Write secret
        assert!(storage.set_key(app_name, secret).is_ok());
        // Get secret
        assert_eq!(storage.get_key(app_name).ok().unwrap().as_str(), secret);
    }

    #[test]
    fn test_system_keys_filestorage_err() {
        let bad_dir: &Path = Path::new("/piro/poro/pero/");
        let storage: FileStorage = FileStorage::new(bad_dir);
        let app_name: &str = "termscp";
        let secret: &str = "Th15-15/My-Супер-Секрет";
        assert!(storage.set_key(app_name, secret).is_err());
    }
}
