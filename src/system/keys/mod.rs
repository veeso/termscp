//! ## KeyStorage
//!
//! `keystorage` provides the trait to manipulate to a KeyStorage

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

// Storages
pub mod filestorage;

/// ## KeyStorageError
/// 
/// defines the error type for the `KeyStorage`
#[derive(PartialEq, std::fmt::Debug)]
pub enum KeyStorageError {
    BadKey,
    Io,
    NoSuchKey,
}

impl std::fmt::Display for KeyStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err: String = String::from(match &self {
            KeyStorageError::BadKey => "Bad key syntax",
            KeyStorageError::Io => "Input/Output error",
            KeyStorageError::NoSuchKey => "No such key",
        });
        write!(f, "{}", err)
    }
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
